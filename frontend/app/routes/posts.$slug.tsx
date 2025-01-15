import { json, type LoaderFunctionArgs } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { Card } from "../components/common/Card";
import { queries } from "../lib/supabase/client";
import type { Database } from "../lib/supabase/types";

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

export const loader = async ({ params }: LoaderFunctionArgs) => {
  if (!params.slug) {
    throw new Response("Not Found", { status: 404 });
  }

  try {
    const post = await queries.posts.getBySlug(params.slug);

    if (!post) {
      throw new Response("Not Found", { status: 404 });
    }

    return json({ post });
  } catch (error) {
    console.error("Error fetching post:", error);
    throw new Response("Not Found", { status: 404 });
  }
};

export default function PostSlug() {
  const { post } = useLoaderData<typeof loader>();
  const typedPost = post as Post;

  return (
    <div className="mx-auto max-w-4xl">
      <Card>
        <article className="max-w-none prose lg:prose-xl">
          <header className="mb-8 not-prose">
            <div className="flex items-center mb-4 space-x-4">
              <img
                src={
                  typedPost.author.avatar_url ||
                  `https://ui-avatars.com/api/?name=${typedPost.author.username}`
                }
                alt={typedPost.author.username}
                className="w-12 h-12 rounded-full"
              />
              <div>
                <p className="text-sm font-medium text-gray-900">
                  {typedPost.author.username}
                </p>
                <p className="text-sm text-gray-500">
                  {new Date(typedPost.created_at).toLocaleDateString()}
                </p>
              </div>
            </div>
            <h1 className="text-3xl font-bold text-gray-900 sm:text-4xl">
              {typedPost.title}
            </h1>
            {typedPost.excerpt && (
              <p className="mt-2 text-xl text-gray-500">{typedPost.excerpt}</p>
            )}
          </header>

          <div
            className="mt-8"
            dangerouslySetInnerHTML={{ __html: typedPost.content }}
          />

          <footer className="mt-8">
            {typedPost.categories && typedPost.categories.length > 0 && (
              <div className="flex flex-wrap gap-2 mb-4">
                {typedPost.categories.map(({ category }) => (
                  <a
                    key={category.id}
                    href={`/categories/${category.slug}`}
                    className="inline-flex items-center px-3 py-0.5 rounded-full text-sm font-medium bg-primary-100 text-primary-800 hover:bg-primary-200"
                  >
                    {category.name}
                  </a>
                ))}
              </div>
            )}

            {typedPost.tags && typedPost.tags.length > 0 && (
              <div className="flex flex-wrap gap-2">
                {typedPost.tags.map(({ tag }) => (
                  <a
                    key={tag.id}
                    href={`/tags/${tag.slug}`}
                    className="inline-flex items-center px-3 py-0.5 rounded-full text-sm font-medium bg-gray-100 text-gray-800 hover:bg-gray-200"
                  >
                    #{tag.name}
                  </a>
                ))}
              </div>
            )}
          </footer>
        </article>
      </Card>
    </div>
  );
}
