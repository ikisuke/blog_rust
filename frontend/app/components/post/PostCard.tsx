import { Link } from "@remix-run/react";
import { Card } from "~/components/common/Card";
import type { Database } from "~/lib/supabase/types";

type Post = Database["public"]["Tables"]["posts"]["Row"] & {
  author: {
    username: string;
    avatar_url: string | null;
  };
  categories?: {
    category: Database["public"]["Tables"]["categories"]["Row"];
  }[];
  tags?: {
    tag: Database["public"]["Tables"]["tags"]["Row"];
  }[];
};

interface PostCardProps {
  post: Post;
}

export const PostCard = ({ post }: PostCardProps) => {
  return (
    <Card className="hover:shadow-lg transition-shadow duration-200">
      <div className="flex flex-col space-y-4">
        <div className="flex items-center space-x-4">
          <img
            src={
              post.author.avatar_url ||
              `https://ui-avatars.com/api/?name=${post.author.username}`
            }
            alt={post.author.username}
            className="h-10 w-10 rounded-full"
          />
          <div>
            <p className="text-sm font-medium text-gray-900">
              {post.author.username}
            </p>
            <p className="text-sm text-gray-500">
              {new Date(post.created_at).toLocaleDateString()}
            </p>
          </div>
        </div>
        <div>
          <Link
            to={`/posts/${post.slug}`}
            className="text-xl font-semibold text-gray-900 hover:text-primary-600"
          >
            {post.title}
          </Link>
          {post.excerpt && (
            <p className="mt-2 text-base text-gray-500">{post.excerpt}</p>
          )}
        </div>
        {(post.categories?.length ?? 0) > 0 && (
          <div className="flex flex-wrap gap-2">
            {post.categories?.map(({ category }) => (
              <Link
                key={category.id}
                to={`/categories/${category.slug}`}
                className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-primary-100 text-primary-800 hover:bg-primary-200"
              >
                {category.name}
              </Link>
            ))}
          </div>
        )}
        {(post.tags?.length ?? 0) > 0 && (
          <div className="flex flex-wrap gap-2">
            {post.tags?.map(({ tag }) => (
              <Link
                key={tag.id}
                to={`/tags/${tag.slug}`}
                className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800 hover:bg-gray-200"
              >
                #{tag.name}
              </Link>
            ))}
          </div>
        )}
      </div>
    </Card>
  );
};
