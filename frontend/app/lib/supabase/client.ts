import { createClient } from "@supabase/supabase-js";
import type { Database } from "./types";

if (!process.env.SUPABASE_URL) {
  throw new Error("Missing environment variable: SUPABASE_URL");
}

if (!process.env.SUPABASE_ANON_KEY) {
  throw new Error("Missing environment variable: SUPABASE_ANON_KEY");
}

export const supabase = createClient<Database>(
  process.env.SUPABASE_URL,
  process.env.SUPABASE_ANON_KEY,
  {
    auth: {
      autoRefreshToken: true,
      persistSession: true,
      detectSessionInUrl: true,
    },
  }
);

// 型付きのクエリヘルパー関数
export const queries = {
  // 投稿関連
  posts: {
    getAll: async ({ page = 1, perPage = 10, status = "published" } = {}) => {
      const from = (page - 1) * perPage;
      const to = from + perPage - 1;

      const { data, error, count } = await supabase
        .from("posts")
        .select("*, author:users(username, avatar_url)", { count: "exact" })
        .eq("status", status)
        .order("created_at", { ascending: false })
        .range(from, to);

      if (error) throw error;
      return { data, count };
    },

    getBySlug: async (slug: string) => {
      const { data, error } = await supabase
        .from("posts")
        .select(
          `
          *,
          author:users(username, avatar_url, bio),
          categories:post_categories(category:categories(*)),
          tags:post_tags(tag:tags(*))
        `
        )
        .eq("slug", slug)
        .single();

      if (error) throw error;
      return data;
    },

    create: async (post: Database["public"]["Tables"]["posts"]["Insert"]) => {
      const { data, error } = await supabase
        .from("posts")
        .insert(post)
        .select()
        .single();

      if (error) throw error;
      return data;
    },

    update: async (
      id: string,
      post: Database["public"]["Tables"]["posts"]["Update"]
    ) => {
      const { data, error } = await supabase
        .from("posts")
        .update(post)
        .eq("id", id)
        .select()
        .single();

      if (error) throw error;
      return data;
    },
  },

  // ユーザー関連
  users: {
    getProfile: async (username: string) => {
      const { data, error } = await supabase
        .from("users")
        .select(
          `
          *,
          profile:profiles(*),
          posts:posts(*)
        `
        )
        .eq("username", username)
        .single();

      if (error) throw error;
      return data;
    },

    updateProfile: async (
      userId: string,
      profile: Database["public"]["Tables"]["profiles"]["Update"]
    ) => {
      const { data, error } = await supabase
        .from("profiles")
        .update(profile)
        .eq("user_id", userId)
        .select()
        .single();

      if (error) throw error;
      return data;
    },
  },

  // コメント関連
  comments: {
    getByPostId: async (postId: string, { page = 1, perPage = 20 } = {}) => {
      const from = (page - 1) * perPage;
      const to = from + perPage - 1;

      const { data, error, count } = await supabase
        .from("comments")
        .select("*, author:users(username, avatar_url)", { count: "exact" })
        .eq("post_id", postId)
        .eq("status", "approved")
        .order("created_at", { ascending: true })
        .range(from, to);

      if (error) throw error;
      return { data, count };
    },

    create: async (
      comment: Database["public"]["Tables"]["comments"]["Insert"]
    ) => {
      const { data, error } = await supabase
        .from("comments")
        .insert(comment)
        .select()
        .single();

      if (error) throw error;
      return data;
    },
  },

  // カテゴリ関連
  categories: {
    getAll: async () => {
      const { data, error } = await supabase
        .from("categories")
        .select("*")
        .order("name", { ascending: true });

      if (error) throw error;
      return data;
    },
  },

  // タグ関連
  tags: {
    getAll: async () => {
      const { data, error } = await supabase
        .from("tags")
        .select("*")
        .order("name", { ascending: true });

      if (error) throw error;
      return data;
    },
  },
};
