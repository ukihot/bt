"use client";

import { signInWithX } from "@/app/actions/signInWithX";
import { useSession } from "next-auth/react";
import { useRouter } from "next/navigation";
import { GreetingMessage } from "./GreetingMessage";

export function XAuthButton() {
    const { data: session, status } = useSession();
    const router = useRouter();
    console.log("session:: ", session);

    const handleClick = async () => {
        if (status === "authenticated") {
            router.push("/we-are-open");
        } else {
            await signInWithX();
        }
    };

    return (
        <div className="z-10 flex flex-col items-center space-y-2">
            <GreetingMessage
                status={status}
                userName={session?.user?.name ?? undefined}
            />
            <div className="flex items-center space-x-2">
                <span className="text-gray-500 text-sm">Here! →</span>
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
                <span className="text-gray-500 text-sm">← Here!</span>
            </div>
        </div>
    );
}
