"use client";

import { useState, useEffect, useCallback } from "react";
import { useRouter } from "next/navigation";
import Cookies from "js-cookie";

export function XAuthButton() {
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const router = useRouter();

    useEffect(() => {
        const token = Cookies.get("x_access_token") ?? "";
        setIsAuthenticated(token.length > 0);
    }, []);

    const handleClick = useCallback(() => {
        if (isAuthenticated) {
            router.push("/we-are-open");
        } else {
            window.location.href = "/api/auth/x";
        }
    }, [isAuthenticated, router]);

    return (
        <button
            type="button"
            onClick={handleClick}
            className={`!px-6 !py-2 transition ${
                isAuthenticated ? "hover:!bg-green-600" : "hover:!bg-blue-600"
            }`}
        >
            {isAuthenticated ? "OPEN UP" : "Sign in with X"}
        </button>
    );
}
