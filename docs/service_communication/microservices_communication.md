# マイクロサービス間通信仕様

## 1. 通信方式

### 1.1 同期通信（REST API）

- サービス間の直接的なリクエスト/レスポンス通信
- HTTP/1.1 を使用
- JSON 形式でデータをやり取り

### 1.2 非同期通信（イベント駆動）

- RabbitMQ を使用したメッセージキュー
- Pub/Sub パターンでイベントを配信
- 非同期処理による疎結合な設計

## 2. サービス間通信の実装

### 2.1 共通の HTTP クライアント実装

```rust
use reqwest::{Client, Error};
use serde::{de::DeserializeOwned, Serialize};

pub struct ServiceClient {
    client: Client,
    base_url: String,
}

impl ServiceClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get<T>(&self, path: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        self.client.get(&url)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<T, Error>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = format!("{}{}", self.base_url, path);
        self.client.post(&url)
            .json(body)
            .send()
            .await?
            .json()
            .await
    }
}
```

### 2.2 サービス別クライアント実装

```rust
// Auth Serviceクライアント
pub struct AuthServiceClient {
    client: ServiceClient,
}

impl AuthServiceClient {
    pub fn new() -> Self {
        let base_url = std::env::var("AUTH_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:3001".to_string());
        Self {
            client: ServiceClient::new(&base_url),
        }
    }

    pub async fn validate_token(&self, token: &str) -> Result<User, Error> {
        self.client.post("/auth/validate", &json!({ "token": token })).await
    }
}

// Blog Serviceクライアント
pub struct BlogServiceClient {
    client: ServiceClient,
}

impl BlogServiceClient {
    pub fn new() -> Self {
        let base_url = std::env::var("BLOG_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:3002".to_string());
        Self {
            client: ServiceClient::new(&base_url),
        }
    }

    pub async fn get_post(&self, id: Uuid) -> Result<Post, Error> {
        self.client.get(&format!("/posts/{}", id)).await
    }
}
```

## 3. イベント駆動アーキテクチャ

### 3.1 イベント定義

```rust
#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    UserCreated(UserCreatedEvent),
    PostPublished(PostPublishedEvent),
    CommentCreated(CommentCreatedEvent),
    CommentModerated(CommentModeratedEvent),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostPublishedEvent {
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentCreatedEvent {
    pub comment_id: Uuid,
    pub post_id: Uuid,
    pub author_id: Option<Uuid>,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}
```

### 3.2 メッセージブローカー設定

```rust
use lapin::{
    options::*, types::FieldTable, Connection,
    ConnectionProperties, ExchangeKind,
};

pub struct MessageBroker {
    conn: Connection,
}

impl MessageBroker {
    pub async fn new() -> Result<Self, Error> {
        let addr = std::env::var("RABBITMQ_URL")
            .unwrap_or_else(|_| "amqp://localhost:5672".to_string());

        let conn = Connection::connect(
            &addr,
            ConnectionProperties::default(),
        ).await?;

        Ok(Self { conn })
    }

    pub async fn setup_exchanges(&self) -> Result<(), Error> {
        let channel = self.conn.create_channel().await?;

        // トピック交換の作成
        channel
            .exchange_declare(
                "blog.events",
                ExchangeKind::Topic,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        Ok(())
    }
}
```

### 3.3 イベントの発行と購読

```rust
impl MessageBroker {
    pub async fn publish_event(&self, event: Event) -> Result<(), Error> {
        let channel = self.conn.create_channel().await?;
        let routing_key = match &event {
            Event::UserCreated(_) => "user.created",
            Event::PostPublished(_) => "post.published",
            Event::CommentCreated(_) => "comment.created",
            Event::CommentModerated(_) => "comment.moderated",
        };

        channel
            .basic_publish(
                "blog.events",
                routing_key,
                BasicPublishOptions::default(),
                &serde_json::to_vec(&event)?,
                BasicProperties::default(),
            )
            .await?;

        Ok(())
    }

    pub async fn subscribe<F>(&self, routing_key: &str, mut handler: F) -> Result<(), Error>
    where
        F: FnMut(Event) -> Result<(), Error> + Send + 'static,
    {
        let channel = self.conn.create_channel().await?;

        let queue = channel
            .queue_declare(
                "",
                QueueDeclareOptions {
                    exclusive: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        channel
            .queue_bind(
                &queue.name(),
                "blog.events",
                routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let mut consumer = channel
            .basic_consume(
                &queue.name(),
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        tokio::spawn(async move {
            while let Some(delivery) = consumer.next().await {
                if let Ok(delivery) = delivery {
                    if let Ok(event) = serde_json::from_slice(&delivery.data) {
                        if let Err(e) = handler(event) {
                            eprintln!("Error handling event: {}", e);
                        }
                    }
                    delivery.ack(BasicAckOptions::default()).await.ok();
                }
            }
        });

        Ok(())
    }
}
```

## 4. サービス間の依存関係

### 4.1 API Gateway → Auth Service

- トークンの検証
- ユーザー認証状態の確認

### 4.2 API Gateway → Blog Service

- 記事の取得・作成・更新・削除
- カテゴリとタグの管理

### 4.3 API Gateway → User Service

- ユーザープロフィールの管理
- フォロー関係の管理

### 4.4 API Gateway → Comment Service

- コメントの取得・作成・更新・削除
- コメントのモデレーション

### 4.5 イベントベースの連携

- Auth Service → User Service: ユーザー作成時のプロフィール初期化
- Blog Service → Comment Service: 記事削除時の関連コメント削除
- Comment Service → User Service: コメント数の更新

## 5. エラーハンドリング

### 5.1 サーキットブレーカーパターン

```rust
use std::time::{Duration, Instant};

pub struct CircuitBreaker {
    failure_threshold: u32,
    reset_timeout: Duration,
    failure_count: u32,
    last_failure: Option<Instant>,
    state: CircuitState,
}

#[derive(PartialEq)]
enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, reset_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            reset_timeout,
            failure_count: 0,
            last_failure: None,
            state: CircuitState::Closed,
        }
    }

    pub async fn execute<F, T, E>(&mut self, f: F) -> Result<T, E>
    where
        F: Future<Output = Result<T, E>>,
    {
        match self.state {
            CircuitState::Open => {
                if let Some(last_failure) = self.last_failure {
                    if last_failure.elapsed() >= self.reset_timeout {
                        self.state = CircuitState::HalfOpen;
                    } else {
                        return Err("Circuit is open".into());
                    }
                }
            }
            CircuitState::HalfOpen => {
                // 半開状態での試行
            }
            CircuitState::Closed => {
                // 通常の実行
            }
        }

        match f.await {
            Ok(result) => {
                self.success();
                Ok(result)
            }
            Err(e) => {
                self.failure();
                Err(e)
            }
        }
    }

    fn success(&mut self) {
        match self.state {
            CircuitState::HalfOpen => {
                self.state = CircuitState::Closed;
                self.failure_count = 0;
                self.last_failure = None;
            }
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            _ => {}
        }
    }

    fn failure(&mut self) {
        self.failure_count += 1;
        self.last_failure = Some(Instant::now());

        if self.failure_count >= self.failure_threshold {
            self.state = CircuitState::Open;
        }
    }
}
```

### 5.2 リトライパターン

```rust
use tokio::time::{sleep, Duration};

pub async fn with_retry<F, T, E>(
    mut f: F,
    max_retries: u32,
    initial_delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> Future<Output = Result<T, E>>,
{
    let mut current_retry = 0;
    let mut delay = initial_delay;

    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if current_retry >= max_retries {
                    return Err(e);
                }
                current_retry += 1;
                sleep(delay).await;
                delay *= 2; // 指数バックオフ
            }
        }
    }
}
```

## 6. 監視とトレーシング

### 6.1 分散トレーシング

```rust
use opentelemetry::{
    trace::{Tracer, TraceError},
    Context, KeyValue,
};

pub struct TracingMiddleware<S> {
    service_name: String,
    inner: S,
}

impl<S> TracingMiddleware<S> {
    pub fn new(service_name: &str, inner: S) -> Self {
        Self {
            service_name: service_name.to_string(),
            inner,
        }
    }
}

impl<S> Service<Request<Body>> for TracingMiddleware<S>
where
    S: Service<Request<Body>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let tracer = global::tracer("");
        let mut span = tracer
            .span_builder(format!("{} {}", req.method(), req.uri().path()))
            .with_attributes(vec![
                KeyValue::new("service.name", self.service_name.clone()),
                KeyValue::new("http.method", req.method().to_string()),
                KeyValue::new("http.url", req.uri().to_string()),
            ])
            .start(&tracer);

        let future = self.inner.call(req);
        Box::pin(async move {
            let result = future.await;
            span.end();
            result
        })
    }
}
```

### 6.2 メトリクス収集

```rust
use prometheus::{
    Counter, CounterVec, Histogram, HistogramOpts, IntCounterVec, Opts, Registry,
};

pub struct Metrics {
    pub registry: Registry,
    pub request_counter: IntCounterVec,
    pub response_time: Histogram,
    pub error_counter: IntCounterVec,
}

impl Metrics {
    pub fn new(service_name: &str) -> Self {
        let registry = Registry::new();

        let request_counter = IntCounterVec::new(
            Opts::new(
                "http_requests_total",
                "Total number of HTTP requests",
            )
            .const_label("service", service_name),
            &["method", "path", "status"],
        )
        .unwrap();

        let response_time = Histogram::with_opts(
            HistogramOpts::new(
                "http_request_duration_seconds",
                "HTTP request duration in seconds",
            )
            .const_label("service", service_name),
        )
        .unwrap();

        let error_counter = IntCounterVec::new(
            Opts::new(
                "error_total",
                "Total number of errors",
            )
            .const_label("service", service_name),
            &["type"],
        )
        .unwrap();

        registry.register(Box::new(request_counter.clone())).unwrap();
        registry.register(Box::new(response_time.clone())).unwrap();
        registry.register(Box::new(error_counter.clone())).unwrap();

        Self {
            registry,
            request_counter,
            response_time,
            error_counter,
        }
    }
}
```
