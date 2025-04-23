"use client";

import { useState, useCallback } from "react";
import { useRouter } from "next/navigation";
import { useDebounce } from "../hooks/useDebounce";
import { useNameCheck } from "../hooks/useNameCheck";

export function NameForm() {
    const [name, setName] = useState("");
    const debounced = useDebounce(name, 400);
    const { isUnique, checking } = useNameCheck(debounced);
    const router = useRouter();

    const onSubmit = useCallback(() => {
        router.push(`/we-are-open?name=${encodeURIComponent(name.trim())}`);
    }, [name, router]);

    return (
        <div className="flex flex-col items-center gap-4">
            <input
                className="!text-black rounded border"
                type="text"
                placeholder="Enter your name"
                value={name}
                onChange={(e) => setName(e.target.value)}
            />

            {/* Validation */}
            {name && (name.length < 3 || name.length > 10) && (
                <p className="text-red-400 text-sm">
                    Name must be between 3 and 10 characters.
                </p>
            )}
            {checking && (
                <p className="text-blue-300 text-sm">
                    Checking availability...
                </p>
            )}
            {!checking && isUnique === false && (
                <p className="text-sm text-yellow-400">
                    That name is already taken.
                </p>
            )}
            {!checking && isUnique === true && (
                <p className="text-green-400 text-sm">Name is available!</p>
            )}

            <button
                className="!px-6 !py-2 hover:!bg-green-600 transition disabled:opacity-40"
                onClick={onSubmit}
                type="button"
                disabled={
                    !isUnique ||
                    name.trim().length < 3 ||
                    name.trim().length > 10
                }
            >
                OPEN UP
            </button>
        </div>
    );
}
