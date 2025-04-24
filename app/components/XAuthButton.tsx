"use client";

import { useSession } from "next-auth/react";
import { useRouter } from "next/navigation";
import { signInWithX } from "@/app/actions/signInWithX";

export function XAuthButton() {
    const session = useSession();
    const router = useRouter();

    const handleClick = async () => {
        if (session.status === "authenticated") {
            // 認証済みの場合は/we-are-openに遷移
            router.push("/we-are-open");
        } else {
            // 未認証の場合はsignInWithXを呼び出し
            await signInWithX();
        }
    };

    return (
        <button
            type="button"
            onClick={handleClick}
            className={`px-6 py-2 transition ${
                session.status === "authenticated"
                    ? "hover:bg-green-600"
                    : "hover:bg-blue-600"
            }`}
        >
            {session.status === "authenticated" ? "OPEN UP" : "Sign in with X"}
        </button>
    );
}
