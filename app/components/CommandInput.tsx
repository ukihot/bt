"use client";

import {
    useContext,
    useEffect,
    Suspense,
    useRef,
    type Dispatch,
    type SetStateAction,
    type RefObject,
} from "react";
import { useSession } from "next-auth/react";
import { TerminalContext, TerminalSectionId } from "../context/TerminalContext";

interface CommandInputProps {
    input: string;
    inputRef: RefObject<HTMLInputElement | null>;
    mode: TerminalSectionId;
    setInput: Dispatch<SetStateAction<string>>;
    handleKeyDown: (e: React.KeyboardEvent) => void;
    isInputEnabled: boolean;
}

const formatCash = (amount: number): string => {
    const units = ["", "K", "M", "B", "T"];
    let idx = 0;
    let val = amount;
    while (val >= 1000 && idx < units.length - 1) {
        val /= 1000;
        idx++;
    }
    return `${val.toFixed(3)}${units[idx]}`;
};

const sendScore = async (username: string, cash: number) => {
    const timestamp = new Date().toISOString();
    try {
        const res = await fetch("/api/ranking/submit", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "x-timestamp": timestamp,
            },
            body: JSON.stringify({ name: username, score: cash }),
        });
        if (!res.ok) {
            const err = await res.json();
            alert(`Failed to send score: ${err.error}`);
        }
    } catch (e) {
        alert(`Error sending score: ${e instanceof Error ? e.message : e}`);
    }
};

export const CommandInput = ({
    input,
    mode,
    setInput,
    handleKeyDown,
    isInputEnabled,
}: CommandInputProps) => {
    const session = useSession();
    const username = session?.data?.user?.name ?? null;
    const context = useContext(TerminalContext);
    if (!context) {
        throw new Error("TerminalContext is not available");
    }

    const { cash, isGameOver } = context;
    const inputRef = useRef<HTMLInputElement>(null);

    // ゲームオーバーでスコア送信
    useEffect(() => {
        if (isGameOver && username) {
            sendScore(username, cash);
        }
    }, [isGameOver, cash, username]);

    // クリックで input にフォーカス
    useEffect(() => {
        const handler = (e: MouseEvent) => {
            e.preventDefault();
            inputRef.current?.focus();
        };
        if (isInputEnabled) inputRef.current?.focus();
        document.addEventListener("click", handler);
        return () => document.removeEventListener("click", handler);
    }, [isInputEnabled]);

    return (
        <div className="mt-1.5 flex w-full font-bold">
            <Suspense fallback={<div>Loading prompt…</div>}>
                {username && (
                    <span className="text-white">
                        ~{username}&nbsp;&gt;&nbsp;{formatCash(cash)}
                        &nbsp;@&nbsp;
                        {TerminalSectionId[mode].toUpperCase()}&nbsp;$&nbsp;
                    </span>
                )}
            </Suspense>
            <input
                ref={inputRef}
                type="text"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={handleKeyDown}
                className="!shadow-none !bg-transparent flex-grow caret-green-500"
                disabled={!isInputEnabled || isGameOver !== null}
                placeholder="Type your command..."
            />
        </div>
    );
};
