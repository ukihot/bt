import { BreadType, type UsageCode } from "../../bt.types";

export const USAGE_UNKNOWN_SHAPING_COMMAND = (cmd: string): UsageCode => ({
    ja: `不明な成形ｺﾏﾝﾄﾞ: ${cmd}`,
    en: `Unknown shaping command: ${cmd}`,
});

export const USAGE_SHAPING_ACTIVITY = (
    shapedCountByType: Record<BreadType, number>,
): UsageCode => {
    const format = (lang: "ja" | "en") =>
        Object.entries(shapedCountByType)
            .map(
                ([kind, count]) =>
                    `${BreadType[kind as unknown as BreadType]}: ${count}${lang === "ja" ? "個" : ""}`,
            )
            .join(", ");

    return {
        ja: `成型処理が完了しました: ${format("ja")}`,
        en: `Shaping completed: ${format("en")}.`,
    };
};

export const USAGE_NO_FIRST_PROOFED_BREAD: UsageCode = {
    ja: "一次発酵済みのﾊﾟﾝがありません。",
    en: "No first-proofed bread available.",
};

export const USAGE_SHAPING_REPAIR_SUCCESS: UsageCode = {
    ja: "成形設備の修復に成功しました。",
    en: "Shaping terminal repair succeeded.",
};

export const USAGE_SHAPING_REPAIR_FAILURE: UsageCode = {
    ja: "成形設備の修復に失敗しました。",
    en: "Shaping terminal repair failed.",
};

export const USAGE_SHAPING_REPAIR_PROGRESS = (spinner: string): UsageCode => ({
    ja: `成型機を修理中... ${spinner}`,
    en: `Repairing shaping unit... ${spinner}`,
});

export const USAGE_SHAPING_SOFTWARE_ERROR: UsageCode = {
    ja: "成形設備の内部ソフトウェアで不具合が発生しました。",
    en: "An issue occurred in the shaping unit's internal software.",
};
