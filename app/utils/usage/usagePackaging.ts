import type { UsageCode } from "../../bt.types";
import { BreadType } from "../../bt.types";

export const USAGE_UNKNOWN_PACKAGING_COMMAND = (cmd: string): UsageCode => ({
    ja: `不明な包装ｺﾏﾝﾄﾞ: ${cmd}`,
    en: `Unknown packaging command: ${cmd}`,
});

export const USAGE_PACKAGING_ACTIVITY = (
    packagedCountByType: Record<BreadType, number>,
): UsageCode => {
    const format = (lang: "ja" | "en") =>
        Object.entries(packagedCountByType)
            .map(
                ([kind, count]) =>
                    `${BreadType[kind as unknown as BreadType]}: ${count}${lang === "ja" ? "個" : ""}`,
            )
            .join(", ");

    return {
        ja: `包装が完了しました: ${format("ja")}を包装しました。`,
        en: `Packaging completed: Packaged ${format("en")}.`,
    };
};

export const USAGE_UNKNOWN_BREAD_TYPE = (bread: string): UsageCode => ({
    ja: `不明なパンの種類: ${bread}`,
    en: `Unknown bread type: ${bread}`,
});

export const USAGE_NO_BAKED_BREAD_OF_TYPE = (
    breadType: BreadType,
): UsageCode => ({
    ja: `焼成済みの${BreadType[breadType]}がありません。`,
    en: `No baked bread of type: ${BreadType[breadType]}.`,
});

export const USAGE_PACKAGING_SUCCESS = (
    count: number,
    breadType: BreadType,
): UsageCode => ({
    ja: `${count}個の${BreadType[breadType]}を包装しました。`,
    en: `Successfully sealed ${count} bread(s) of type: ${BreadType[breadType]}.`,
});
