import type { UsageCode } from "../../bt.types";

export const USAGE_UNKNOWN_SALESFRONT_COMMAND = (cmd: string): UsageCode => ({
    ja: `不明な販売ﾌﾛﾝﾄｺﾏﾝﾄﾞ: ${cmd}`,
    en: `Unknown sales front command: ${cmd}`,
});

export const USAGE_SALESFRONT_ACTIVITY: UsageCode = {
    ja: "販売ﾌﾛﾝﾄ処理が完了しました。",
    en: "Sales front process completed.",
};

export const USAGE_NO_PACKAGED_BREAD: UsageCode = {
    ja: "包装済みのﾊﾟﾝがありません。",
    en: "No packaged bread available.",
};

export const USAGE_SALESFRONT_SOLD = (
    kind: string,
    price: number,
): UsageCode => ({
    ja: `${kind} が ${price} ｼﾞｭﾛﾎﾟｩ で売れました。🎉`,
    en: `${kind} sold for ${price} julopow. 🎉`,
});
