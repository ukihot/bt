import { NextResponse } from "next/server";
import { neon } from "@neondatabase/serverless";

export async function POST(req: Request) {
    const dbUrl = process.env.DATABASE_URL;
    if (!dbUrl) {
        return NextResponse.json(
            { error: "DATABASE_URL is not set" },
            { status: 500 },
        );
    }

    const sql = neon(dbUrl);

    try {
        const body = await req.json();
        const incomingScore = body.score;

        if (typeof incomingScore !== "number") {
            return NextResponse.json(
                { error: "Invalid score" },
                { status: 400 },
            );
        }

        // 現在のランキングの最低スコアを取得
        const [lowest] = await sql`
            SELECT score
            FROM results
            ORDER BY score ASC
            LIMIT 1
        `;

        const lowestScore = lowest?.score ?? Number.NEGATIVE_INFINITY;

        const canEnter = incomingScore > lowestScore;

        return NextResponse.json({ canEnter });
    } catch (error) {
        return NextResponse.json(
            {
                error: "Internal server error",
                detail: (error as Error).message,
            },
            { status: 500 },
        );
    }
}
