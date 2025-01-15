import type { MetaFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { PostCard } from "~/components/post/PostCard";
import { queries } from "~/lib/supabase/client";

export const meta: MetaFunction = () => {
  return [
    { title: "Blog Sample" },
    { name: "description", content: "Welcome to Blog Sample!" },
  ];
};

export const loader = async () => {
  try {
    const { data: posts, count } = await queries.posts.getAll();
    return { posts, count };
  } catch (error) {
    console.error("Error fetching posts:", error);
    return { posts: [], count: 0 };
  }
};

export default function Index() {
  const { posts } = useLoaderData<typeof loader>();

  return (
    <div className="space-y-8">
      <div className="text-center">
        <h1 className="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl md:text-6xl">
          <span className="block">Welcome to</span>
          <span className="block text-primary-600">Blog Sample</span>
        </h1>
        <p className="mt-3 max-w-md mx-auto text-base text-gray-500 sm:text-lg md:mt-5 md:text-xl md:max-w-3xl">
          Share your thoughts, ideas, and stories with the world.
        </p>
      </div>

      <div className="relative">
        <div className="absolute inset-0 flex items-center" aria-hidden="true">
          <div className="w-full border-t border-gray-300" />
        </div>
        <div className="relative flex justify-center">
          <span className="px-3 bg-gray-50 text-lg font-medium text-gray-900">
            Latest Posts
          </span>
        </div>
      </div>

      <div className="grid gap-6 lg:grid-cols-2">
        {posts.map((post) => (
          <PostCard key={post.id} post={post} />
        ))}
      </div>
    </div>
  );
}
