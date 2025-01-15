import { Link } from "@remix-run/react";
import { Button } from "../common/Button";
import type { Database } from "../../lib/supabase/types";

interface HeaderProps {
  user?: Database["public"]["Tables"]["users"]["Row"] | null;
}

export const Header = ({ user }: HeaderProps) => {
  return (
    <header className="bg-white shadow">
      <div className="px-4 mx-auto max-w-7xl sm:px-6 lg:px-8">
        <div className="flex justify-between h-16">
          <div className="flex">
            <div className="flex flex-shrink-0 items-center">
              <Link to="/" className="text-2xl font-bold text-primary-600">
                Blog
              </Link>
            </div>
            <nav className="hidden sm:ml-6 sm:flex sm:space-x-8">
              <Link
                to="/"
                className="inline-flex items-center px-1 pt-1 text-sm font-medium text-gray-900"
              >
                Home
              </Link>
              <Link
                to="/posts"
                className="inline-flex items-center px-1 pt-1 text-sm font-medium text-gray-500 hover:text-gray-900"
              >
                Posts
              </Link>
              <Link
                to="/categories"
                className="inline-flex items-center px-1 pt-1 text-sm font-medium text-gray-500 hover:text-gray-900"
              >
                Categories
              </Link>
            </nav>
          </div>
          <div className="flex items-center">
            {user ? (
              <div className="flex items-center space-x-4">
                <Link
                  to="/dashboard"
                  className="inline-flex items-center px-1 pt-1 text-sm font-medium text-gray-500 hover:text-gray-900"
                >
                  Dashboard
                </Link>
                <div className="flex items-center space-x-2">
                  <img
                    className="w-8 h-8 rounded-full"
                    src={
                      user.avatar_url ||
                      `https://ui-avatars.com/api/?name=${user.username}`
                    }
                    alt={user.username}
                  />
                  <span className="text-sm font-medium text-gray-700">
                    {user.username}
                  </span>
                </div>
                <form action="/logout" method="post">
                  <Button variant="secondary" size="sm" type="submit">
                    Logout
                  </Button>
                </form>
              </div>
            ) : (
              <div className="flex items-center space-x-4">
                <Link to="/login">
                  <Button variant="secondary" size="sm">
                    Login
                  </Button>
                </Link>
                <Link to="/register">
                  <Button size="sm">Register</Button>
                </Link>
              </div>
            )}
          </div>
        </div>
      </div>
    </header>
  );
};
