import { type PropsWithChildren } from "react";
import { Header } from "./Header";
import { Footer } from "./Footer";
import type { Database } from "~/lib/supabase/types";

interface LayoutProps extends PropsWithChildren {
  user?: Database["public"]["Tables"]["users"]["Row"] | null;
  showNewsletter?: boolean;
}

export const Layout = ({
  children,
  user,
  showNewsletter = false,
}: LayoutProps) => {
  return (
    <div className="min-h-screen flex flex-col">
      <Header user={user} />
      <main className="flex-1 bg-gray-50">
        <div className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">{children}</div>
      </main>
      <Footer showNewsletter={showNewsletter} />
    </div>
  );
};

export const DashboardLayout = ({
  children,
  user,
}: Omit<LayoutProps, "showNewsletter">) => {
  return (
    <div className="min-h-screen flex flex-col">
      <Header user={user} />
      <div className="flex-1 flex">
        <nav className="hidden md:block w-64 bg-white border-r border-gray-200">
          <div className="h-full flex flex-col">
            <div className="flex-1 py-6 overflow-y-auto">
              <div className="px-4 space-y-1">
                <a
                  href="/dashboard"
                  className="group flex items-center px-2 py-2 text-sm font-medium rounded-md text-gray-900 hover:text-gray-900 hover:bg-gray-50"
                >
                  <svg
                    className="mr-3 h-6 w-6 text-gray-400 group-hover:text-gray-500"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
                    />
                  </svg>
                  Dashboard
                </a>
                <a
                  href="/dashboard/posts"
                  className="group flex items-center px-2 py-2 text-sm font-medium rounded-md text-gray-600 hover:text-gray-900 hover:bg-gray-50"
                >
                  <svg
                    className="mr-3 h-6 w-6 text-gray-400 group-hover:text-gray-500"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9.5a2.5 2.5 0 00-2.5-2.5H15"
                    />
                  </svg>
                  Posts
                </a>
                <a
                  href="/dashboard/comments"
                  className="group flex items-center px-2 py-2 text-sm font-medium rounded-md text-gray-600 hover:text-gray-900 hover:bg-gray-50"
                >
                  <svg
                    className="mr-3 h-6 w-6 text-gray-400 group-hover:text-gray-500"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"
                    />
                  </svg>
                  Comments
                </a>
                <a
                  href="/dashboard/profile"
                  className="group flex items-center px-2 py-2 text-sm font-medium rounded-md text-gray-600 hover:text-gray-900 hover:bg-gray-50"
                >
                  <svg
                    className="mr-3 h-6 w-6 text-gray-400 group-hover:text-gray-500"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                    />
                  </svg>
                  Profile
                </a>
              </div>
            </div>
          </div>
        </nav>
        <main className="flex-1 overflow-y-auto bg-gray-50">
          <div className="py-6">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 md:px-8">
              {children}
            </div>
          </div>
        </main>
      </div>
    </div>
  );
};
