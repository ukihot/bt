import { NextResponse } from "next/server";
import { neon } from "@neondatabase/serverless";

export async function GET() {
    const dbUrl = process.env.DATABASE_URL;
    if (!dbUrl) {
        return NextResponse.json(
            { error: "DATABASE_URL is not set" },
            { status: 500 },
        );
    }

    const sql = neon(dbUrl);

    const results = await sql`
        SELECT
          id,
          name,
          score,
          recorded_at
        FROM results
        ORDER BY score DESC
        LIMIT 10
    `;
    return NextResponse.json(results);
}
