import type { ActionFunctionArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { supabase } from "../lib/supabase/client";

export const action = async ({ request }: ActionFunctionArgs) => {
  if (request.method !== "POST") {
    throw new Response("Method not allowed", { status: 405 });
  }

  const { error } = await supabase.auth.signOut();

  if (error) {
    console.error("Error signing out:", error.message);
  }

  return redirect("/");
};

export const loader = async () => {
  return redirect("/");
};
