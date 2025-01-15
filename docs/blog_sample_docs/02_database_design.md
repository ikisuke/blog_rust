# データベース設計

## 概要

Supabase を使用したデータベース設計について説明します。

## テーブル構成

### users（ユーザー）

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(50) NOT NULL UNIQUE,
    display_name VARCHAR(100),
    bio TEXT,
    avatar_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

### profiles（プロフィール設定）

```sql
CREATE TABLE profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    website VARCHAR(255),
    location VARCHAR(100),
    social_links JSONB,
    preferences JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

### posts（ブログ記事）

```sql
CREATE TABLE posts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    author_id UUID REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    content TEXT NOT NULL,
    excerpt TEXT,
    status VARCHAR(20) DEFAULT 'draft',
    published_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX posts_author_id_idx ON posts(author_id);
CREATE INDEX posts_status_idx ON posts(status);
```

### categories（カテゴリ）

```sql
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(50) NOT NULL UNIQUE,
    slug VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,
    parent_id UUID REFERENCES categories(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

### post_categories（記事とカテゴリの中間テーブル）

```sql
CREATE TABLE post_categories (
    post_id UUID REFERENCES posts(id) ON DELETE CASCADE,
    category_id UUID REFERENCES categories(id) ON DELETE CASCADE,
    PRIMARY KEY (post_id, category_id)
);
```

### tags（タグ）

```sql
CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(50) NOT NULL UNIQUE,
    slug VARCHAR(50) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

### post_tags（記事とタグの中間テーブル）

```sql
CREATE TABLE post_tags (
    post_id UUID REFERENCES posts(id) ON DELETE CASCADE,
    tag_id UUID REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (post_id, tag_id)
);
```

### comments（コメント）

```sql
CREATE TABLE comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    post_id UUID REFERENCES posts(id) ON DELETE CASCADE,
    author_id UUID REFERENCES users(id) ON DELETE SET NULL,
    parent_id UUID REFERENCES comments(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    status VARCHAR(20) DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX comments_post_id_idx ON comments(post_id);
CREATE INDEX comments_author_id_idx ON comments(author_id);
```

### follows（フォロー関係）

```sql
CREATE TABLE follows (
    follower_id UUID REFERENCES users(id) ON DELETE CASCADE,
    following_id UUID REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (follower_id, following_id)
);
```

## Row Level Security (RLS)ポリシー

### users

```sql
-- 読み取り: 全ユーザー可能
CREATE POLICY "Users are viewable by everyone" ON users
    FOR SELECT USING (true);

-- 更新: 自分のレコードのみ
CREATE POLICY "Users can update own record" ON users
    FOR UPDATE USING (auth.uid() = id);
```

### posts

```sql
-- 読み取り: 公開済み記事は全ユーザー、下書きは作成者のみ
CREATE POLICY "Posts are viewable by everyone if published" ON posts
    FOR SELECT USING (
        status = 'published' OR
        auth.uid() = author_id
    );

-- 作成・更新・削除: 作成者のみ
CREATE POLICY "Posts are editable by author" ON posts
    FOR ALL USING (auth.uid() = author_id);
```

### comments

```sql
-- 読み取り: 承認済みコメントは全ユーザー、保留中は作成者とモデレーターのみ
CREATE POLICY "Comments are viewable if approved" ON comments
    FOR SELECT USING (
        status = 'approved' OR
        auth.uid() = author_id OR
        auth.uid() IN (SELECT id FROM users WHERE role = 'moderator')
    );

-- 作成: 認証済みユーザー
CREATE POLICY "Comments can be created by authenticated users" ON comments
    FOR INSERT WITH CHECK (auth.uid() IS NOT NULL);

-- 更新・削除: 作成者とモデレーターのみ
CREATE POLICY "Comments are editable by author and moderators" ON comments
    FOR ALL USING (
        auth.uid() = author_id OR
        auth.uid() IN (SELECT id FROM users WHERE role = 'moderator')
    );
```

## インデックス戦略

- 主キー（UUID）: デフォルトでインデックス作成
- 外部キー: 参照整合性のためインデックス作成
- 検索頻度の高いカラム:
  - posts.status
  - posts.slug
  - comments.status
  - users.username
  - tags.slug
  - categories.slug

## バックアップ戦略

- Supabase の自動バックアップ機能を利用
- 日次フルバックアップ
- Point-in-Time Recovery (PITR) の有効化
