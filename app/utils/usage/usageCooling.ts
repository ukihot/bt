import { BreadType, type UsageCode } from "../../bt.types";

export const USAGE_UNKNOWN_COOLING_COMMAND = (cmd: string): UsageCode => ({
    ja: `不明な冷却ｺﾏﾝﾄﾞ: ${cmd}`,
    en: `Unknown cooling command: ${cmd}`,
});

export const USAGE_COOLING_FIRST_PROOFING_COMPLETED = (
    firstProofedCountByType: Record<BreadType, number>,
): UsageCode => {
    const format = (lang: "ja" | "en") =>
        Object.entries(firstProofedCountByType)
            .map(
                ([kind, count]) =>
                    `${BreadType[kind as unknown as BreadType]}: ${count}${lang === "ja" ? "個" : ""}`,
            )
            .join(", ");

    return {
        ja: `一次発酵が完了しました: ${format("ja")}`,
        en: `First proofing completed: ${format("en")}.`,
    };
};

export const USAGE_COOLING_NO_DOUGH_AFTER_MIXING: UsageCode = {
    ja: "混錬後の生地はありません。",
    en: "No dough available after mixing.",
};

export const USAGE_COOLING_SECOND_PROOFING_COMPLETED = (
    secondProofedCountByType: Record<BreadType, number>,
): UsageCode => {
    const format = (lang: "ja" | "en") =>
        Object.entries(secondProofedCountByType)
            .map(
                ([kind, count]) =>
                    `${BreadType[kind as unknown as BreadType]}: ${count}${lang === "ja" ? "個" : ""}`,
            )
            .join(", ");

    return {
        ja: `二次発酵が完了しました: ${format("ja")}`,
        en: `Second proofing completed: ${format("en")}.`,
    };
};

export const USAGE_COOLING_NO_SHAPED_BREAD: UsageCode = {
    ja: "成型済みのﾊﾟﾝはありません。",
    en: "No shaped bread available.",
};

export const USAGE_COOLING_REPAIR_FAILURE: UsageCode = {
    ja: "冷却設備の修復に失敗しました。",
    en: "Cooling terminal repair failed.",
};

export const USAGE_COOLING_REPAIR_PROGRESS = (spinner: string): UsageCode => ({
    ja: `冷却器を修理中... ${spinner}`,
    en: `Repairing cooling unit... ${spinner}`,
});

export const USAGE_COOLING_OVERHEAT_DETECTED: UsageCode = {
    ja: "冷却器でオーバーヒートが検出されました。",
    en: "Overheat detected in the cooling unit.",
};

export const USAGE_COOLING_OVERHEAT_REPAIRED: UsageCode = {
    ja: "冷却器のオーバーヒートが修復されました。",
    en: "Cooling unit overheat has been repaired.",
};
