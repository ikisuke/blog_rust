import { type ActionFunctionArgs, json } from "@remix-run/node";
import { Form, useActionData } from "@remix-run/react";
import { Button } from "../components/common/Button";
import { Input } from "../components/common/Input";
import { Card } from "../components/common/Card";
import { supabase } from "../lib/supabase/client";
import { z } from "zod";

const LoginSchema = z.object({
  email: z.string().email("Invalid email address"),
  password: z.string().min(6, "Password must be at least 6 characters"),
});

type ActionData = {
  success?: boolean;
  errors?: {
    email?: string;
    password?: string;
    _form?: string;
  };
};

export const action = async ({ request }: ActionFunctionArgs) => {
  const formData = await request.formData();
  const email = formData.get("email")?.toString();
  const password = formData.get("password")?.toString();

  if (!email || !password) {
    return json<ActionData>(
      {
        success: false,
        errors: {
          _form: "Email and password are required",
        },
      },
      { status: 400 }
    );
  }

  const result = LoginSchema.safeParse({ email, password });

  if (!result.success) {
    const formattedErrors: ActionData["errors"] = {};
    result.error.issues.forEach((issue) => {
      if (issue.path[0] === "email" || issue.path[0] === "password") {
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

  const { error } = await supabase.auth.signInWithPassword({
    email: result.data.email,
    password: result.data.password,
  });

  if (error) {
    return json<ActionData>(
      {
        success: false,
        errors: {
          _form: "Invalid email or password",
        },
      },
      { status: 400 }
    );
  }

  return json<ActionData>({ success: true });
};

export default function Login() {
  const actionData = useActionData<typeof action>();

  return (
    <div className="mx-auto max-w-md">
      <Card>
        <div className="mb-8 text-center">
          <h1 className="text-2xl font-bold">Login</h1>
          <p className="mt-2 text-gray-600">
            Welcome back! Please enter your details.
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
            label="Password"
            name="password"
            type="password"
            autoComplete="current-password"
            required
            error={actionData?.errors?.password}
          />

          <Button type="submit" className="w-full">
            Sign in
          </Button>
        </Form>
      </Card>
    </div>
  );
}
