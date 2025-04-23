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

    // username が null の場合、リダイレクト
    useEffect(() => {
        if (!username || username.trim() === "") {
            router.replace("/");
        }
    }, [username, router]);

    if (!username || username.trim() === "") {
        return null; // username がない場合、何も表示しない
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

    // 入力フィールドにフォーカス
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

    return (
        <div className="mt-1.5 flex w-full font-bold">
            <Suspense fallback={<div>Loading prompt...</div>}>
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
