"use client";

type GreetingMessageProps = {
    status: "authenticated" | "unauthenticated" | "loading";
    userName?: string;
};

function getGreeting() {
    const hour = new Date().getHours();
    if (hour < 12) return "Good morning";
    if (hour < 18) return "Good afternoon";
    return "Good evening";
}

export function GreetingMessage({ status, userName }: GreetingMessageProps) {
    return (
        <span className="z-50 text-green-500 text-sm">
            {`${getGreeting()}, ${
                status === "authenticated" ? (userName ?? "user") : "guest"
            }.`}
        </span>
    );
}
