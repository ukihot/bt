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
        const incomingName = body.name;

        if (typeof incomingName !== "string" || incomingName.trim() === "") {
            return NextResponse.json(
                { error: "Invalid name" },
                { status: 400 },
            );
        }

        const result = await sql`
            SELECT COUNT(*) AS count
            FROM results
            WHERE name = ${incomingName}
        `;

        const isUnique = Number(result[0]?.count) === 0;

        return NextResponse.json({ isUnique });
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
