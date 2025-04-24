import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

// 保護対象パスのマッチ設定
export const config = {
    matcher: ["/we-are-open/:path*"],
};

export function middleware(req: NextRequest) {
    const token = req.cookies.get("x_access_token")?.value;

    // クッキーがなければルートへリダイレクト
    if (!token) {
        const url = req.nextUrl.clone();
        url.pathname = "/";
        return NextResponse.redirect(url);
    }

    // それ以外は通常通過
    return NextResponse.next();
}
