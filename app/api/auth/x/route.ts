import { type NextRequest, NextResponse } from "next/server";
import crypto from "node:crypto";
import { getRequiredEnv } from "@/app/utils/getRequiredEnv";

const CLIENT_ID = getRequiredEnv("X_CLIENT_ID");
const CLIENT_SECRET = getRequiredEnv("X_CLIENT_SECRET");
const REDIRECT_URI = getRequiredEnv("X_REDIRECT_URI");

// Base64 URL-safe エンコード
function base64url(buffer: Buffer): string {
    return buffer
        .toString("base64")
        .replace(/\+/g, "-")
        .replace(/\//g, "_")
        .replace(/=+$/, "");
}

// トークン交換関数
async function fetchToken(code: string, verifier: string) {
    console.log("[XAuth] fetchToken: code=", code, "verifier=", verifier);
    const params = new URLSearchParams({
        grant_type: "authorization_code",
        code,
        client_id: CLIENT_ID,
        client_secret: CLIENT_SECRET,
        redirect_uri: REDIRECT_URI,
        code_verifier: verifier,
    });
    const res = await fetch("https://api.x.com/2/oauth2/token", {
        method: "POST",
        headers: { "Content-Type": "application/x-www-form-urlencoded" },
        body: params.toString(),
    });
    console.log(
        "[XAuth] fetchToken: token endpoint response status=",
        res.status,
    );
    if (!res.ok) {
        const err = await res.text();
        console.error(`[XAuth] Token exchange failed: ${res.status} ${err}`);
        throw new Error(`Token exchange failed: ${res.status}`);
    }
    const json = await res.json();
    console.log("[XAuth] fetchToken: tokenData=", json);
    return json;
}

export async function GET(request: NextRequest) {
    console.log("[XAuth] GET invoked, request.url=", request.url); // ⚙️
    const url = new URL(request.url);
    const code = url.searchParams.get("code");
    const stateIn = url.searchParams.get("state");
    const cookies = request.cookies;

    // (1) 認可開始フェーズ
    if (!code) {
        console.log("[XAuth] No code found, start auth flow");
        const state = crypto.randomBytes(16).toString("hex");
        const verifier = base64url(crypto.randomBytes(32));
        const challenge = base64url(
            crypto.createHash("sha256").update(verifier).digest(),
        );
        console.log("[XAuth] Generated state=", state, "verifier=", verifier);

        const params = new URLSearchParams({
            response_type: "code",
            client_id: CLIENT_ID,
            redirect_uri: REDIRECT_URI,
            scope: "tweet.read users.read offline.access",
            state,
            code_challenge: challenge,
            code_challenge_method: "S256",
        }).toString();

        const authUrl = `https://x.com/i/oauth2/authorize?${params}`;
        console.log("[XAuth] Redirecting to authorize URL:", authUrl);

        const response = NextResponse.redirect(authUrl);
        response.cookies.set("x_oauth_state", state, {
            httpOnly: true,
            secure: true,
            sameSite: "lax",
            path: "/",
        });
        response.cookies.set("x_code_verifier", verifier, {
            httpOnly: true,
            secure: true,
            sameSite: "lax",
            path: "/",
        });
        return response;
    }

    // (2) コールバック検証フェーズ
    const savedState = cookies.get("x_oauth_state")?.value;
    console.log(
        "[XAuth] Callback phase: stateIn=",
        stateIn,
        "savedState=",
        savedState,
    );
    if (!stateIn || stateIn !== savedState) {
        console.error("[XAuth] CSRF check failed");
        return NextResponse.json({ error: "Invalid state" }, { status: 400 });
    }

    const verifierEntry = cookies.get("x_code_verifier")?.value;
    console.log("[XAuth] Retrieved code_verifier from cookie:", verifierEntry);
    if (!verifierEntry) {
        console.warn("[XAuth] Missing code_verifier, restarting auth");
        return NextResponse.redirect("/api/auth/x");
    }

    // (3) トークン取得フェーズ
    let accessToken: string;
    let refreshToken: string;
    try {
        const { access_token, refresh_token } = await fetchToken(
            code,
            verifierEntry,
        );
        accessToken = access_token;
        refreshToken = refresh_token;
    } catch (e) {
        console.error("[XAuth] Token fetch error:", e);
        return NextResponse.json(
            { error: "Token fetch failed" },
            { status: 500 },
        );
    }

    // (4) ユーザー情報取得フェーズ
    console.log("[XAuth] Fetching user info with accessToken");
    const userRes = await fetch("https://api.x.com/2/users/me", {
        headers: { Authorization: `Bearer ${accessToken}` },
    });
    console.log("[XAuth] user info response status=", userRes.status);
    if (!userRes.ok) {
        const err = await userRes.text();
        console.error(
            `[XAuth] Failed to fetch user info: ${userRes.status} ${err}`,
        );
        return NextResponse.json(
            { error: "User fetch failed" },
            { status: 500 },
        );
    }
    const userJson = await userRes.json();
    console.log("[XAuth] userJson=", userJson);
    const username = userJson.data?.username;
    if (!username) {
        console.error("[XAuth] Username missing in userJson");
        return NextResponse.json(
            { error: "Username missing" },
            { status: 500 },
        );
    }

    // (5) 最終リダイレクト & Cookie 設定
    console.log("[XAuth] Redirecting to /we-are-open");
    const final = NextResponse.redirect("/we-are-open");
    final.cookies.set("x_access_token", accessToken, {
        httpOnly: true,
        secure: true,
        sameSite: "lax",
        path: "/",
        maxAge: 60 * 60 * 2,
    });
    final.cookies.set("x_refresh_token", refreshToken, {
        httpOnly: true,
        secure: true,
        sameSite: "lax",
        path: "/",
        maxAge: 60 * 60 * 24 * 30,
    });
    final.cookies.delete("x_oauth_state");
    final.cookies.delete("x_code_verifier");
    return final;
}
