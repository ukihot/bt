import NextAuth from "next-auth";
import Twitter from "next-auth/providers/twitter";

export const { handlers, signIn, signOut, auth } = NextAuth({
    providers: [
        Twitter({
            clientId: process.env.AUTH_TWITTER_ID || "",
            clientSecret: process.env.AUTH_TWITTER_SECRET || "",
        }),
    ],
    pages: {
        signIn: "/",
    },
});
