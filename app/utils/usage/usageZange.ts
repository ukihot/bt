import type { UsageCode } from "../../bt.types";

export const USAGE_UNKNOWN_ZANGE_COMMAND = (cmd: string): UsageCode => ({
    ja: `不明な懺悔ｺﾏﾝﾄﾞ: ${cmd}`,
    en: `Unknown command: ${cmd}`,
});

export const USAGE_ZANGE_ESCAPE_SUCCESS: UsageCode = {
    ja: "脱出成功",
    en: "Escape successful.",
};

export const USAGE_ZANGE_ESCAPE_FAILURE: UsageCode = {
    ja: "脱出失敗",
    en: "Escape failed.",
};
