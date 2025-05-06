import fs from "node:fs";
import path from "node:path";
import { NextResponse } from "next/server";

export async function GET() {
    const gifsDirectory = path.join(process.cwd(), "public");
    const files = fs.readdirSync(gifsDirectory);
    const gifFiles = files
        .filter((file) => file.endsWith(".gif"))
        .map((file) => `/${file}`);

    return NextResponse.json(gifFiles);
}
