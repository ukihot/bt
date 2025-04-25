"use client";

import { useSession } from "next-auth/react";
import { useRouter } from "next/navigation";
import { signInWithX } from "@/app/actions/signInWithX";

function getGreeting() {
    const hour = new Date().getHours();
    if (hour < 12) return "Good morning";
    if (hour < 18) return "Good afternoon";
    return "Good evening";
}

export function XAuthButton() {
    const { data: session, status } = useSession();
    const router = useRouter();

    const handleClick = async () => {
        if (status === "authenticated") {
            router.push("/we-are-open");
        } else {
            await signInWithX();
        }
    };

    return (
        <div className="flex flex-col items-center space-y-2">
            <span className="text-green-300 text-sm">
                {`${getGreeting()}, ${
                    status === "authenticated"
                        ? (session?.user?.name ?? "user")
                        : "guest"
                }.`}
            </span>
            <button
                type="button"
                onClick={handleClick}
                className={`rounded px-6 py-2 transition ${
                    status === "authenticated"
                        ? "bg-green-500 text-white hover:bg-green-600"
                        : "bg-blue-500 text-white hover:bg-blue-600"
                }`}
            >
                {status === "authenticated" ? "OPEN UP" : "Sign in with X"}
            </button>
        </div>
    );
}
