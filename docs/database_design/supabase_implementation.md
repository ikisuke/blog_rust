# Supabase 実装仕様

## 1. データベース接続設定

各マイクロサービスで Supabase クライアントを初期化する共通実装：

```rust
use postgrest::Postgrest;
use std::env;

pub struct SupabaseClient {
    client: Postgrest,
    service_key: String,
}

impl SupabaseClient {
    pub fn new() -> Self {
        let url = env::var("SUPABASE_URL")
            .expect("SUPABASE_URL must be set");
        let service_key = env::var("SUPABASE_SERVICE_KEY")
            .expect("SUPABASE_SERVICE_KEY must be set");

        let client = Postgrest::new(format!("{}/rest/v1", url))
            .insert_header("apikey", &service_key)
            .insert_header("Authorization", format!("Bearer {}", &service_key));

        Self {
            client,
            service_key,
        }
    }
}
```

## 2. サービス別データベース実装

### 2.1 Auth Service

```rust
impl AuthService {
    pub async fn create_user(&self, user: &User) -> Result<User> {
        let response = self.db.client
            .from("users")
            .insert(json!({
                "email": user.email,
                "username": user.username,
                "password_hash": user.password_hash,
            }))
            .execute()
            .await?;

        // レスポンスのパース処理
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let response = self.db.client
            .from("users")
            .select("*")
            .eq("email", email)
            .single()
            .execute()
            .await?;

        // レスポンスのパース処理
    }
}
```

### 2.2 Blog Service

```rust
impl BlogService {
    pub async fn create_post(&self, post: &Post) -> Result<Post> {
        // トランザクション開始
        let response = self.db.client
            .from("posts")
            .insert(json!({
                "author_id": post.author_id,
                "title": post.title,
                "content": post.content,
                "status": post.status,
            }))
            .execute()
            .await?;

        // カテゴリとタグの関連付け
        if let Some(categories) = &post.categories {
            self.db.client
                .from("post_categories")
                .insert(categories.iter().map(|c| json!({
                    "post_id": post.id,
                    "category_id": c.id,
                })).collect::<Vec<_>>())
                .execute()
                .await?;
        }

        // レスポンスのパース処理
    }

    pub async fn get_posts(&self, filters: &PostFilters) -> Result<Vec<Post>> {
        let mut query = self.db.client
            .from("posts")
            .select("*, author:users(username, avatar_url), categories:post_categories(category:categories(*))");

        if let Some(status) = &filters.status {
            query = query.eq("status", status);
        }

        let response = query.execute().await?;
        // レスポンスのパース処理
    }
}
```

### 2.3 User Service

```rust
impl UserService {
    pub async fn update_profile(&self, profile: &Profile) -> Result<Profile> {
        let response = self.db.client
            .from("profiles")
            .update(json!({
                "website": profile.website,
                "location": profile.location,
                "social_links": profile.social_links,
                "preferences": profile.preferences,
            }))
            .eq("user_id", profile.user_id)
            .execute()
            .await?;

        // レスポンスのパース処理
    }

    pub async fn follow_user(&self, follower_id: Uuid, following_id: Uuid) -> Result<()> {
        self.db.client
            .from("follows")
            .insert(json!({
                "follower_id": follower_id,
                "following_id": following_id,
            }))
            .execute()
            .await?;

        Ok(())
    }
}
```

### 2.4 Comment Service

```rust
impl CommentService {
    pub async fn create_comment(&self, comment: &Comment) -> Result<Comment> {
        let response = self.db.client
            .from("comments")
            .insert(json!({
                "post_id": comment.post_id,
                "author_id": comment.author_id,
                "parent_id": comment.parent_id,
                "content": comment.content,
                "status": comment.status,
            }))
            .execute()
            .await?;

        // レスポンスのパース処理
    }

    pub async fn get_comments(&self, post_id: Uuid, filters: &CommentFilters) -> Result<Vec<Comment>> {
        let mut query = self.db.client
            .from("comments")
            .select("*, author:users(username, avatar_url)")
            .eq("post_id", post_id);

        if let Some(status) = &filters.status {
            query = query.eq("status", status);
        }

        let response = query.execute().await?;
        // レスポンスのパース処理
    }
}
```

## 3. データベースマイグレーション

Supabase のマイグレーション管理：

```sql
-- 1. ユーザー関連
create table public.users (
    id uuid primary key default uuid_generate_v4(),
    email text unique not null,
    username text unique not null,
    password_hash text not null,
    display_name text,
    bio text,
    avatar_url text,
    created_at timestamp with time zone default timezone('utc'::text, now()) not null,
    updated_at timestamp with time zone default timezone('utc'::text, now()) not null
);

create table public.profiles (
    user_id uuid primary key references public.users(id) on delete cascade,
    website text,
    location text,
    social_links jsonb,
    preferences jsonb,
    created_at timestamp with time zone default timezone('utc'::text, now()) not null,
    updated_at timestamp with time zone default timezone('utc'::text, now()) not null
);

-- 2. ブログ関連
create table public.posts (
    id uuid primary key default uuid_generate_v4(),
    author_id uuid references public.users(id) on delete cascade not null,
    title text not null,
    slug text unique not null,
    content text not null,
    excerpt text,
    status text not null default 'draft',
    published_at timestamp with time zone,
    created_at timestamp with time zone default timezone('utc'::text, now()) not null,
    updated_at timestamp with time zone default timezone('utc'::text, now()) not null
);

create table public.categories (
    id uuid primary key default uuid_generate_v4(),
    name text not null,
    slug text unique not null,
    description text,
    parent_id uuid references public.categories(id),
    created_at timestamp with time zone default timezone('utc'::text, now()) not null
);

create table public.post_categories (
    post_id uuid references public.posts(id) on delete cascade,
    category_id uuid references public.categories(id) on delete cascade,
    primary key (post_id, category_id)
);

-- 3. コメント関連
create table public.comments (
    id uuid primary key default uuid_generate_v4(),
    post_id uuid references public.posts(id) on delete cascade not null,
    author_id uuid references public.users(id) on delete set null,
    parent_id uuid references public.comments(id) on delete cascade,
    content text not null,
    status text not null default 'pending',
    created_at timestamp with time zone default timezone('utc'::text, now()) not null,
    updated_at timestamp with time zone default timezone('utc'::text, now()) not null,
    deleted_at timestamp with time zone
);

-- 4. フォロー関連
create table public.follows (
    follower_id uuid references public.users(id) on delete cascade,
    following_id uuid references public.users(id) on delete cascade,
    created_at timestamp with time zone default timezone('utc'::text, now()) not null,
    primary key (follower_id, following_id)
);
```

## 4. Row Level Security (RLS)ポリシー

```sql
-- ユーザー
alter table public.users enable row level security;
create policy "Users are viewable by everyone" on public.users
    for select using (true);
create policy "Users can update own record" on public.users
    for update using (auth.uid() = id);

-- プロフィール
alter table public.profiles enable row level security;
create policy "Profiles are viewable by everyone" on public.profiles
    for select using (true);
create policy "Users can update own profile" on public.profiles
    for update using (auth.uid() = user_id);

-- 投稿
alter table public.posts enable row level security;
create policy "Posts are viewable by everyone if published" on public.posts
    for select using (status = 'published' or auth.uid() = author_id);
create policy "Posts are editable by author" on public.posts
    for all using (auth.uid() = author_id);

-- コメント
alter table public.comments enable row level security;
create policy "Comments are viewable if approved" on public.comments
    for select using (
        status = 'approved' or
        auth.uid() = author_id or
        exists (
            select 1 from public.users
            where id = auth.uid() and role = 'moderator'
        )
    );
create policy "Comments are editable by author" on public.comments
    for update using (auth.uid() = author_id);
```

## 5. インデックス

```sql
-- ユーザー検索用
create index users_username_idx on public.users using btree (username);
create index users_email_idx on public.users using btree (email);

-- 投稿検索用
create index posts_slug_idx on public.posts using btree (slug);
create index posts_status_idx on public.posts using btree (status);
create index posts_author_id_idx on public.posts using btree (author_id);
create index posts_created_at_idx on public.posts using btree (created_at);

-- コメント検索用
create index comments_post_id_idx on public.comments using btree (post_id);
create index comments_author_id_idx on public.comments using btree (author_id);
create index comments_parent_id_idx on public.comments using btree (parent_id);
create index comments_status_idx on public.comments using btree (status);

-- カテゴリ検索用
create index categories_slug_idx on public.categories using btree (slug);
create index categories_parent_id_idx on public.categories using btree (parent_id);
```

## 6. トリガー

```sql
-- 更新日時の自動更新
create or replace function public.update_updated_at_column()
returns trigger as $$
begin
    new.updated_at = timezone('utc'::text, now());
    return new;
end;
$$ language plpgsql;

create trigger update_users_updated_at
    before update on public.users
    for each row
    execute function public.update_updated_at_column();

create trigger update_profiles_updated_at
    before update on public.profiles
    for each row
    execute function public.update_updated_at_column();

create trigger update_posts_updated_at
    before update on public.posts
    for each row
    execute function public.update_updated_at_column();

create trigger update_comments_updated_at
    before update on public.comments
    for each row
    execute function public.update_updated_at_column();
```
