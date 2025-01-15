export type Json =
  | string
  | number
  | boolean
  | null
  | { [key: string]: Json | undefined }
  | Json[];

export interface Database {
  public: {
    Tables: {
      users: {
        Row: {
          id: string;
          email: string;
          username: string;
          display_name: string | null;
          bio: string | null;
          avatar_url: string | null;
          created_at: string;
          updated_at: string;
        };
        Insert: {
          id?: string;
          email: string;
          username: string;
          display_name?: string | null;
          bio?: string | null;
          avatar_url?: string | null;
          created_at?: string;
          updated_at?: string;
        };
        Update: {
          id?: string;
          email?: string;
          username?: string;
          display_name?: string | null;
          bio?: string | null;
          avatar_url?: string | null;
          created_at?: string;
          updated_at?: string;
        };
      };
      profiles: {
        Row: {
          user_id: string;
          website: string | null;
          location: string | null;
          social_links: Json | null;
          preferences: Json | null;
          created_at: string;
          updated_at: string;
        };
        Insert: {
          user_id: string;
          website?: string | null;
          location?: string | null;
          social_links?: Json | null;
          preferences?: Json | null;
          created_at?: string;
          updated_at?: string;
        };
        Update: {
          user_id?: string;
          website?: string | null;
          location?: string | null;
          social_links?: Json | null;
          preferences?: Json | null;
          created_at?: string;
          updated_at?: string;
        };
      };
      posts: {
        Row: {
          id: string;
          author_id: string;
          title: string;
          slug: string;
          content: string;
          excerpt: string | null;
          status: string;
          published_at: string | null;
          created_at: string;
          updated_at: string;
        };
        Insert: {
          id?: string;
          author_id: string;
          title: string;
          slug: string;
          content: string;
          excerpt?: string | null;
          status?: string;
          published_at?: string | null;
          created_at?: string;
          updated_at?: string;
        };
        Update: {
          id?: string;
          author_id?: string;
          title?: string;
          slug?: string;
          content?: string;
          excerpt?: string | null;
          status?: string;
          published_at?: string | null;
          created_at?: string;
          updated_at?: string;
        };
      };
      categories: {
        Row: {
          id: string;
          name: string;
          slug: string;
          description: string | null;
          parent_id: string | null;
          created_at: string;
        };
        Insert: {
          id?: string;
          name: string;
          slug: string;
          description?: string | null;
          parent_id?: string | null;
          created_at?: string;
        };
        Update: {
          id?: string;
          name?: string;
          slug?: string;
          description?: string | null;
          parent_id?: string | null;
          created_at?: string;
        };
      };
      post_categories: {
        Row: {
          post_id: string;
          category_id: string;
        };
        Insert: {
          post_id: string;
          category_id: string;
        };
        Update: {
          post_id?: string;
          category_id?: string;
        };
      };
      tags: {
        Row: {
          id: string;
          name: string;
          slug: string;
          created_at: string;
        };
        Insert: {
          id?: string;
          name: string;
          slug: string;
          created_at?: string;
        };
        Update: {
          id?: string;
          name?: string;
          slug?: string;
          created_at?: string;
        };
      };
      post_tags: {
        Row: {
          post_id: string;
          tag_id: string;
        };
        Insert: {
          post_id: string;
          tag_id: string;
        };
        Update: {
          post_id?: string;
          tag_id?: string;
        };
      };
      comments: {
        Row: {
          id: string;
          post_id: string;
          author_id: string | null;
          parent_id: string | null;
          content: string;
          status: string;
          created_at: string;
          updated_at: string;
        };
        Insert: {
          id?: string;
          post_id: string;
          author_id?: string | null;
          parent_id?: string | null;
          content: string;
          status?: string;
          created_at?: string;
          updated_at?: string;
        };
        Update: {
          id?: string;
          post_id?: string;
          author_id?: string | null;
          parent_id?: string | null;
          content?: string;
          status?: string;
          created_at?: string;
          updated_at?: string;
        };
      };
      follows: {
        Row: {
          follower_id: string;
          following_id: string;
          created_at: string;
        };
        Insert: {
          follower_id: string;
          following_id: string;
          created_at?: string;
        };
        Update: {
          follower_id?: string;
          following_id?: string;
          created_at?: string;
        };
      };
    };
    Views: {
      [_ in never]: never;
    };
    Functions: {
      [_ in never]: never;
    };
    Enums: {
      [_ in never]: never;
    };
  };
}
