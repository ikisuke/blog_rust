import { type ActionFunctionArgs, json } from "@remix-run/node";
import { Form, useActionData } from "@remix-run/react";
import { Button } from "../components/common/Button";
import { Input } from "../components/common/Input";
import { Card } from "../components/common/Card";
import { supabase } from "../lib/supabase/client";
import { z } from "zod";

const RegisterSchema = z
  .object({
    email: z.string().email("Invalid email address"),
    username: z.string().min(3, "Username must be at least 3 characters"),
    password: z.string().min(6, "Password must be at least 6 characters"),
    confirmPassword: z.string(),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: "Passwords don't match",
    path: ["confirmPassword"],
  });

type ActionData = {
  success?: boolean;
  errors?: {
    email?: string;
    username?: string;
    password?: string;
    confirmPassword?: string;
    _form?: string;
  };
};

export const action = async ({ request }: ActionFunctionArgs) => {
  const formData = await request.formData();
  const email = formData.get("email")?.toString();
  const username = formData.get("username")?.toString();
  const password = formData.get("password")?.toString();
  const confirmPassword = formData.get("confirmPassword")?.toString();

  if (!email || !username || !password || !confirmPassword) {
    return json<ActionData>(
      {
        success: false,
        errors: {
          _form: "All fields are required",
        },
      },
      { status: 400 }
    );
  }

  const result = RegisterSchema.safeParse({
    email,
    username,
    password,
    confirmPassword,
  });

  if (!result.success) {
    const formattedErrors: ActionData["errors"] = {};
    result.error.issues.forEach((issue) => {
      if (
        issue.path[0] === "email" ||
        issue.path[0] === "username" ||
        issue.path[0] === "password" ||
        issue.path[0] === "confirmPassword"
      ) {
        formattedErrors[issue.path[0]] = issue.message;
      }
    });

    return json<ActionData>(
      {
        success: false,
        errors: formattedErrors,
      },
      { status: 400 }
    );
  }

  const { error: authError } = await supabase.auth.signUp({
    email: result.data.email,
    password: result.data.password,
  });

  if (authError) {
    return json<ActionData>(
      {
        success: false,
        errors: {
          _form: authError.message,
        },
      },
      { status: 400 }
    );
  }

  const { error: userError } = await supabase.from("users").insert([
    {
      email: result.data.email,
      username: result.data.username,
    },
  ]);

  if (userError) {
    return json<ActionData>(
      {
        success: false,
        errors: {
          _form: userError.message,
        },
      },
      { status: 400 }
    );
  }

  return json<ActionData>({ success: true });
};

export default function Register() {
  const actionData = useActionData<typeof action>();

  return (
    <div className="mx-auto max-w-md">
      <Card>
        <div className="mb-8 text-center">
          <h1 className="text-2xl font-bold">Register</h1>
          <p className="mt-2 text-gray-600">
            Create an account to get started.
          </p>
        </div>

        <Form method="post" className="space-y-6">
          {actionData?.errors?._form && (
            <div className="p-4 bg-red-50 rounded-md">
              <div className="flex">
                <div className="flex-shrink-0">
                  <svg
                    className="w-5 h-5 text-red-400"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                  >
                    <path
                      fillRule="evenodd"
                      d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z"
                      clipRule="evenodd"
                    />
                  </svg>
                </div>
                <div className="ml-3">
                  <h3 className="text-sm font-medium text-red-800">
                    {actionData.errors._form}
                  </h3>
                </div>
              </div>
            </div>
          )}

          <Input
            label="Email"
            name="email"
            type="email"
            autoComplete="email"
            required
            error={actionData?.errors?.email}
          />

          <Input
            label="Username"
            name="username"
            type="text"
            autoComplete="username"
            required
            error={actionData?.errors?.username}
          />

          <Input
            label="Password"
            name="password"
            type="password"
            autoComplete="new-password"
            required
            error={actionData?.errors?.password}
          />

          <Input
            label="Confirm Password"
            name="confirmPassword"
            type="password"
            autoComplete="new-password"
            required
            error={actionData?.errors?.confirmPassword}
          />

          <Button type="submit" className="w-full">
            Create Account
          </Button>
        </Form>
      </Card>
    </div>
  );
}
