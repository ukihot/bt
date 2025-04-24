"use server";

import { signIn } from "@/auth";

export async function signInWithX() {
    await signIn("twitter", {
        redirectTo: "/",
    });
}
