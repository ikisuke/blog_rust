# フロントエンド設計

## 技術スタック

- React + Remix
- TailwindCSS
- React Query
- Zod (バリデーション)

## ディレクトリ構造

```
src/
├── components/         # 共通コンポーネント
│   ├── common/        # 汎用的なUI部品
│   ├── layout/        # レイアウト関連
│   ├── post/          # 記事関連
│   ├── user/          # ユーザー関連
│   └── comment/       # コメント関連
├── routes/            # Remixルート定義
├── models/            # データモデル定義
├── hooks/             # カスタムフック
├── utils/             # ユーティリティ関数
├── styles/            # スタイル定義
└── lib/               # 外部サービス連携
```

## コンポーネント設計

### 共通コンポーネント

```typescript
// components/common/Button.tsx
interface ButtonProps {
  variant: "primary" | "secondary" | "danger";
  size: "sm" | "md" | "lg";
  isLoading?: boolean;
  disabled?: boolean;
  children: React.ReactNode;
  onClick?: () => void;
}

// components/common/Input.tsx
interface InputProps {
  type: "text" | "email" | "password";
  label: string;
  error?: string;
  value: string;
  onChange: (value: string) => void;
}

// components/common/Card.tsx
interface CardProps {
  title?: string;
  footer?: React.ReactNode;
  children: React.ReactNode;
}
```

### レイアウトコンポーネント

```typescript
// components/layout/Header.tsx
interface HeaderProps {
  user?: {
    username: string;
    avatar: string;
  };
}

// components/layout/Sidebar.tsx
interface SidebarProps {
  categories: Array<{
    id: string;
    name: string;
    count: number;
  }>;
}

// components/layout/Footer.tsx
interface FooterProps {
  showNewsletter?: boolean;
}
```

### 記事関連コンポーネント

```typescript
// components/post/PostCard.tsx
interface PostCardProps {
  post: {
    id: string;
    title: string;
    excerpt: string;
    author: {
      name: string;
      avatar: string;
    };
    publishedAt: string;
    categories: string[];
    tags: string[];
  };
}

// components/post/PostEditor.tsx
interface PostEditorProps {
  initialContent?: string;
  onSave: (content: string) => Promise<void>;
  onPublish: () => Promise<void>;
}

// components/post/PostContent.tsx
interface PostContentProps {
  content: string;
  isPreview?: boolean;
}
```

## ルート設計

```typescript
// routes/index.tsx
export default function Index() {
  const posts = useLoaderData<typeof loader>();
  return <PostList posts={posts} />;
}

// routes/posts.$slug.tsx
export default function Post() {
  const { post } = useLoaderData<typeof loader>();
  return <PostDetail post={post} />;
}

// routes/dashboard.tsx
export default function Dashboard() {
  const { posts, stats } = useLoaderData<typeof loader>();
  return <DashboardLayout posts={posts} stats={stats} />;
}
```

## データフェッチング

### API クライアント

```typescript
// lib/api/client.ts
export class ApiClient {
  constructor(private baseUrl: string, private token?: string) {}

  async get<T>(path: string, params?: Record<string, string>) {
    // 実装
  }

  async post<T>(path: string, data: unknown) {
    // 実装
  }

  async put<T>(path: string, data: unknown) {
    // 実装
  }

  async delete(path: string) {
    // 実装
  }
}
```

### カスタムフック

```typescript
// hooks/usePosts.ts
export function usePosts(params: PostsParams) {
  return useQuery({
    queryKey: ["posts", params],
    queryFn: () => api.getPosts(params),
  });
}

// hooks/usePost.ts
export function usePost(slug: string) {
  return useQuery({
    queryKey: ["post", slug],
    queryFn: () => api.getPost(slug),
  });
}

// hooks/useCreatePost.ts
export function useCreatePost() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: CreatePostData) => api.createPost(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["posts"] });
    },
  });
}
```

## 状態管理

- React Query: サーバーステート
- React Context: グローバル UI 状態
- Form State: React Hook Form

```typescript
// contexts/AuthContext.tsx
interface AuthContextType {
  user: User | null;
  login: (credentials: Credentials) => Promise<void>;
  logout: () => Promise<void>;
}

// contexts/ThemeContext.tsx
interface ThemeContextType {
  theme: "light" | "dark";
  toggleTheme: () => void;
}
```

## バリデーション

```typescript
// models/post.ts
export const PostSchema = z.object({
  title: z.string().min(1).max(100),
  content: z.string().min(1),
  excerpt: z.string().max(200).optional(),
  status: z.enum(["draft", "published"]),
  categories: z.array(z.string().uuid()),
  tags: z.array(z.string()),
});

// models/comment.ts
export const CommentSchema = z.object({
  content: z.string().min(1).max(1000),
  postId: z.string().uuid(),
  parentId: z.string().uuid().optional(),
});
```

## エラーハンドリング

```typescript
// components/ErrorBoundary.tsx
export class ErrorBoundary extends React.Component<
  { children: React.ReactNode },
  { hasError: boolean }
> {
  // 実装
}

// utils/error.ts
export function handleApiError(error: unknown) {
  if (error instanceof ApiError) {
    // APIエラーの処理
  } else if (error instanceof NetworkError) {
    // ネットワークエラーの処理
  } else {
    // 予期せぬエラーの処理
  }
}
```

## パフォーマンス最適化

### 画像最適化

```typescript
// components/common/Image.tsx
interface OptimizedImageProps {
  src: string;
  alt: string;
  width: number;
  height: number;
  loading?: "lazy" | "eager";
}
```

### Code Splitting

```typescript
// routes/posts.$slug.tsx
const PostEditor = React.lazy(() => import("~/components/post/PostEditor"));
```

### キャッシュ戦略

```typescript
// lib/api/cache.ts
export const cacheConfig = {
  posts: {
    staleTime: 5 * 60 * 1000, // 5分
    cacheTime: 30 * 60 * 1000, // 30分
  },
  user: {
    staleTime: 60 * 1000, // 1分
    cacheTime: 5 * 60 * 1000, // 5分
  },
};
```

## セキュリティ対策

- XSS 対策: DOMPurify
- CSRF 対策: トークン
- Content Security Policy
- Secure HTTP Headers

## アクセシビリティ

- WAI-ARIA 対応
- キーボード操作
- スクリーンリーダー対応
- カラーコントラスト
