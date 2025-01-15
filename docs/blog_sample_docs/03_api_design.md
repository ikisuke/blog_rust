# API 設計

## 概要

各マイクロサービスの API 設計について説明します。

## API Gateway

### ベース URL

```
https://api.blog.example.com/v1
```

### 共通レスポンスフォーマット

```json
{
  "status": "success" | "error",
  "data": {}, // レスポンスデータ
  "error": {  // エラー時のみ
    "code": "ERROR_CODE",
    "message": "エラーメッセージ"
  }
}
```

## 認証サービス API

### エンドポイント: /auth

#### ユーザー登録

```
POST /auth/register
Content-Type: application/json

Request:
{
  "email": "string",
  "password": "string",
  "username": "string"
}

Response:
{
  "user": {
    "id": "uuid",
    "email": "string",
    "username": "string"
  },
  "token": "string"
}
```

#### ログイン

```
POST /auth/login
Content-Type: application/json

Request:
{
  "email": "string",
  "password": "string"
}

Response:
{
  "token": "string",
  "user": {
    "id": "uuid",
    "email": "string",
    "username": "string"
  }
}
```

#### トークンリフレッシュ

```
POST /auth/refresh
Authorization: Bearer {refresh_token}

Response:
{
  "token": "string"
}
```

## ブログサービス API

### エンドポイント: /posts

#### 記事一覧取得

```
GET /posts
Query Parameters:
- page: number (default: 1)
- per_page: number (default: 10)
- status: string (published|draft)
- category: string
- tag: string
- author: uuid

Response:
{
  "posts": [
    {
      "id": "uuid",
      "title": "string",
      "excerpt": "string",
      "author": {
        "id": "uuid",
        "username": "string"
      },
      "published_at": "datetime",
      "categories": ["string"],
      "tags": ["string"]
    }
  ],
  "pagination": {
    "current_page": number,
    "total_pages": number,
    "total_items": number
  }
}
```

#### 記事詳細取得

```
GET /posts/{slug}

Response:
{
  "post": {
    "id": "uuid",
    "title": "string",
    "content": "string",
    "author": {
      "id": "uuid",
      "username": "string",
      "bio": "string"
    },
    "published_at": "datetime",
    "categories": ["string"],
    "tags": ["string"]
  }
}
```

#### 記事作成

```
POST /posts
Authorization: Bearer {token}
Content-Type: application/json

Request:
{
  "title": "string",
  "content": "string",
  "excerpt": "string",
  "status": "draft|published",
  "categories": ["uuid"],
  "tags": ["uuid"]
}

Response:
{
  "id": "uuid",
  "slug": "string"
}
```

## ユーザーサービス API

### エンドポイント: /users

#### プロフィール取得

```
GET /users/{username}

Response:
{
  "user": {
    "id": "uuid",
    "username": "string",
    "display_name": "string",
    "bio": "string",
    "avatar_url": "string",
    "website": "string",
    "social_links": {
      "twitter": "string",
      "github": "string"
    }
  }
}
```

#### プロフィール更新

```
PUT /users/profile
Authorization: Bearer {token}
Content-Type: application/json

Request:
{
  "display_name": "string",
  "bio": "string",
  "website": "string",
  "social_links": {
    "twitter": "string",
    "github": "string"
  }
}
```

#### フォロー/アンフォロー

```
POST /users/{username}/follow
DELETE /users/{username}/follow
Authorization: Bearer {token}
```

## コメントサービス API

### エンドポイント: /comments

#### コメント一覧取得

```
GET /posts/{post_id}/comments
Query Parameters:
- page: number (default: 1)
- per_page: number (default: 20)

Response:
{
  "comments": [
    {
      "id": "uuid",
      "content": "string",
      "author": {
        "id": "uuid",
        "username": "string",
        "avatar_url": "string"
      },
      "created_at": "datetime",
      "replies": [
        // ネストされたコメント
      ]
    }
  ],
  "pagination": {
    "current_page": number,
    "total_pages": number,
    "total_items": number
  }
}
```

#### コメント投稿

```
POST /posts/{post_id}/comments
Authorization: Bearer {token}
Content-Type: application/json

Request:
{
  "content": "string",
  "parent_id": "uuid" // オプション（返信の場合）
}

Response:
{
  "id": "uuid",
  "created_at": "datetime"
}
```

## エラーコード

### 共通エラーコード

- `AUTH_REQUIRED`: 認証が必要
- `INVALID_TOKEN`: 無効なトークン
- `PERMISSION_DENIED`: 権限がない
- `RESOURCE_NOT_FOUND`: リソースが見つからない
- `VALIDATION_ERROR`: バリデーションエラー
- `RATE_LIMIT_EXCEEDED`: レート制限超過

### サービス固有エラーコード

- `POST_NOT_PUBLISHED`: 記事が公開されていない
- `COMMENT_DISABLED`: コメントが無効
- `USER_ALREADY_EXISTS`: ユーザーが既に存在する
- `INVALID_CREDENTIALS`: 認証情報が無効

## レート制限

- 認証なし: 60 requests/minute
- 認証あり: 1000 requests/minute
- IP ベースの制限: 1000 requests/minute/IP

## キャッシュ戦略

- 公開済み記事: 5 分
- ユーザープロフィール: 1 分
- コメント一覧: 1 分
- 記事一覧: 2 分

## セキュリティ

- すべてのエンドポイントで HTTPS 必須
- JWT 認証
- CORS 設定
- CSRF トークン
