"use client";

import dynamic from "next/dynamic";
import { useFetch } from "./hooks/useFetch";
import type { RankingItem } from "./home.types";
import { XAuthButton } from "./components/XAuthButton";

const RankingTable = dynamic(
    () => import("./components/RankingTable").then((mod) => mod.RankingTable),
    {
        ssr: false,
        loading: () => <p className="text-gray-400">Loading rankings…</p>,
    },
);

export default function RankingPage() {
    const { data: ranking, error } = useFetch<RankingItem[]>("/api/ranking");

    if (error) {
        return (
            <div className="min-h-screen bg-gray-900 text-gray-100">
                <p className="text-red-500">An error occurred: {error}</p>
            </div>
        );
    }

    return (
        <div className="flex min-h-screen flex-col items-center justify-center gap-20 p-4 text-gray-100">
            {ranking && <RankingTable items={ranking} />}
            <XAuthButton />
        </div>
    );
}
