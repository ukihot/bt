import { NextResponse } from "next/server";
import { neon } from "@neondatabase/serverless";
import { DEFAULT_CASH } from "@/app/context/TerminalContext";

const TIMESTAMP_TOLERANCE_MS = 5 * 60 * 1000;

const getHeader = (req: Request, key: string) => req.headers.get(key) ?? "";

const isFreshTimestamp = (timestamp: string) => {
    const time = Date.parse(timestamp);
    return (
        !Number.isNaN(time) &&
        Math.abs(Date.now() - time) <= TIMESTAMP_TOLERANCE_MS
    );
};

const withError = (message: string, status = 400, detail?: string) =>
    NextResponse.json(
        { error: message, ...(detail && { detail }) },
        { status },
    );

export async function POST(req: Request) {
    const SECRET = process.env.SUPER_SECRET_KEY;
    const DB_URL = process.env.DATABASE_URL;
    if (!SECRET) return withError("Server misconfiguration: missing key", 500);
    if (!DB_URL) return withError("DATABASE_URL is not set", 500);

    const [rawBody, timestamp] = await Promise.all([
        req.text(),
        getHeader(req, "x-timestamp"),
    ]);

    // タイムスタンプの鮮度チェック
    if (!isFreshTimestamp(timestamp)) {
        return withError("Invalid or expired timestamp", 401);
    }

    let body: { name: string; score: number };
    try {
        body = JSON.parse(rawBody);
        if (typeof body.name !== "string" || typeof body.score !== "number") {
            throw new Error("Invalid input types");
        }
    } catch (e) {
        return withError("Invalid JSON", 400, (e as Error).message);
    }

    const sql = neon(DB_URL);

    try {
        const [row] =
            await sql`SELECT score FROM results ORDER BY score ASC LIMIT 1`;
        const lowest = row?.score ?? Number.NEGATIVE_INFINITY;
        const currentScore = body.score - DEFAULT_CASH;
        const canEnter = currentScore > lowest;

        if (canEnter) {
            await sql`
                INSERT INTO results (name, score, recorded_at)
                VALUES (${body.name}, ${currentScore}, ${new Date().toISOString()})
            `;
        }

        return NextResponse.json({ canEnter });
    } catch (e) {
        return withError("Database error", 500, (e as Error).message);
    }
}
