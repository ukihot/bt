import { BreadType, type UsageCode } from "../../bt.types";

export const USAGE_UNKNOWN_BAKING_COMMAND = (cmd: string): UsageCode => ({
    ja: `不明な焼成ｺﾏﾝﾄﾞ: ${cmd}`,
    en: `Unknown baking command: ${cmd}`,
});

export const USAGE_BAKING_ACTIVITY = (
    bakedCountByType: Record<BreadType, number>,
): UsageCode => {
    const format = (lang: "ja" | "en") =>
        Object.entries(bakedCountByType)
            .map(
                ([kind, count]) =>
                    `${BreadType[kind as unknown as BreadType]}: ${count}${lang === "ja" ? "個" : ""}`,
            )
            .join(", ");

    return {
        ja: `焼成が完了しました: ${format("ja")}`,
        en: `Baking completed: ${format("en")}.`,
    };
};

export const USAGE_NO_SECOND_PROOFED_BREAD: UsageCode = {
    ja: "二次発酵済みのﾊﾟﾝがありません。",
    en: "No second-proofed bread available.",
};

export const USAGE_ALERT_GAS: UsageCode = {
    ja: "異常なにおいを検知しました",
    en: "Unusual odor detected",
};

export const USAGE_GAS_REPAIR_SUCCESS: UsageCode = {
    ja: "ガス漏れが正常に修理されました。",
    en: "Gas leak has been successfully repaired.",
};

export const USAGE_GAS_REPAIR_FAILURE: UsageCode = {
    ja: "ガス漏れの修理に失敗しました。",
    en: "Failed to repair the gas leak.",
};

export const USAGE_BAKING_REPAIR_PROGRESS = (spinner: string): UsageCode => ({
    ja: `焼窯を修理中... ${spinner}`,
    en: `Repairing baking unit... ${spinner}`,
});

export const USAGE_FIRE: UsageCode = {
    ja: "火災が発生しました！",
    en: "Fire hazard detected!",
};
