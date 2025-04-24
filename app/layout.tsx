import type { Metadata } from "next";
import "./globals.css";
import { DotGothic16 } from "next/font/google";
import { SpeedInsights } from "@vercel/speed-insights/next";
import { Providers } from "./providers";

const dotGothic16 = DotGothic16({
    subsets: ["latin"],
    weight: "400",
});

export const metadata: Metadata = {
    title: "Bakery Text",
    description: "文字だけでパンを焼き続けるミニゲームです。",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="ja">
            <head>
                <link
                    rel="stylesheet"
                    href="https://jdan.github.io/98.css/98.css"
                />
            </head>
            <body className={`${dotGothic16.className} antialiased`}>
                <Providers>
                    <main>{children}</main>
                    <SpeedInsights />
                </Providers>
            </body>
        </html>
    );
}
