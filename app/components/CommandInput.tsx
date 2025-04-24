"use client";

import { useContext, useEffect, Suspense } from "react";
import { TerminalContext, TerminalSectionId } from "../context/TerminalContext";
import { useSearchParams, useRouter } from "next/navigation";

interface CommandInputProps {
    input: string;
    mode: TerminalSectionId;
    setInput: React.Dispatch<React.SetStateAction<string>>;
    handleKeyDown: (e: React.KeyboardEvent) => void;
    inputRef: React.RefObject<HTMLInputElement | null>;
    isInputEnabled: boolean;
}

const formatCash = (amount: number): string => {
    const units = ["", "K", "M", "B", "T", "Q", "Qi", "Sx", "Sp", "Oc", "No"];
    let unitIndex = 0;
    let formattedAmount = amount;

    while (formattedAmount >= 1000 && unitIndex < units.length - 1) {
        formattedAmount /= 1000;
        unitIndex++;
    }

    return `${formattedAmount.toFixed(3)}${units[unitIndex]}`;
};

const PromptDisplay = ({
    cash,
    mode,
}: { cash: number; mode: TerminalSectionId }) => {
    const searchParams = useSearchParams();
    const username = searchParams.get("name");
    const router = useRouter();

    useEffect(() => {
        if (!username || username.trim() === "") {
            router.replace("/");
        }
    }, [username, router]);

    if (!username || username.trim() === "") {
        return null;
    }

    return (
        <span className="text-white">
            ~{username}&nbsp;&gt;&nbsp;{formatCash(cash)}&nbsp;@&nbsp;
            {TerminalSectionId[mode].toUpperCase()}
            &nbsp;$&nbsp;
        </span>
    );
};

export const CommandInput = ({
    input,
    mode,
    setInput,
    handleKeyDown,
    inputRef,
    isInputEnabled,
}: CommandInputProps) => {
    const context = useContext(TerminalContext);
    if (!context) {
        throw new Error("TerminalContext is not available");
    }

    const { cash, isGameOver } = context;

    useEffect(() => {
        const handleClick = (e: MouseEvent) => {
            e.preventDefault();
            inputRef.current?.focus();
        };

        if (isInputEnabled) {
            inputRef.current?.focus();
        }

        document.addEventListener("click", handleClick);
        return () => {
            document.removeEventListener("click", handleClick);
        };
    }, [inputRef, isInputEnabled]);

    // 修正されたスコア送信処理
    useEffect(() => {
        if (!isGameOver) return;

        const params = new URLSearchParams(window.location.search);
        const username = params.get("name");
        if (!username || username.trim() === "") return;

        const timestamp = new Date().toISOString();
        const payload = {
            name: username,
            score: cash,
        };

        const sendScore = async () => {
            try {
                const res = await fetch("/api/ranking/submit", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                        "x-timestamp": timestamp,
                    },
                    body: JSON.stringify(payload),
                });

                if (!res.ok) {
                    const error = await res.json();
                    alert(`Failed to send score: ${error.error}`);
                }
            } catch (err) {
                if (err instanceof Error) {
                    alert(`An error has occurred: ${err.message}`);
                } else {
                    alert("An unexpected error has occurred.");
                }
            }
        };

        void sendScore();
    }, [isGameOver, cash]);

    return (
        <div className="mt-1.5 flex w-full font-bold">
            <Suspense fallback={<div>Loading prompt…</div>}>
                <PromptDisplay cash={cash} mode={mode} />
            </Suspense>
            <input
                ref={inputRef}
                type="text"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={handleKeyDown}
                className="!shadow-none !bg-transparent flex-grow caret-green-500"
                disabled={!isInputEnabled || isGameOver !== null}
                title="Command Input"
                placeholder="Type your command here"
            />
        </div>
    );
};
