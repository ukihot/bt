import { auth } from "@/auth";

export const middleware = auth;
export const config = { matcher: ["/we-are-open"] };
