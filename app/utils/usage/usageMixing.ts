import type { UsageCode } from "../../bt.types";

export const USAGE_UNKNOWN_MIXING_COMMAND = (cmd: string): UsageCode => ({
    ja: `不明な混錬ｺﾏﾝﾄﾞ: ${cmd}`,
    en: `Unknown mixing command: ${cmd}`,
});

export const USAGE_MISSING_MIXING_COMMAND: UsageCode = {
    ja: "ﾊﾟﾝの種類または数量が不足しています。",
    en: "Bread type or quantity is missing.",
};

export const USAGE_MIXING_SUCCESS = (breadKind: string): UsageCode => ({
    ja: `${breadKind} の生地ができました。`,
    en: `The dough for ${breadKind} has been prepared.`,
});

export const USAGE_MIXING_ERROR = (breadKind: string): UsageCode => ({
    ja: `${breadKind} の材料が足りませんでした。`,
    en: `The ingredients for ${breadKind} were insufficient.`,
});

export const USAGE_MIXING_REPAIR_FAILURE: UsageCode = {
    ja: "混錬機の修復に失敗しました。",
    en: "Failed to repair the mixing terminal.",
};

export const USAGE_INGREDIENT_CONSUMPTION = (
    ingredient: string,
    amount: number,
): UsageCode => ({
    ja: `${ingredient} を ${amount.toFixed(1)} g使用した。`,
    en: `${amount.toFixed(1)} g of ${ingredient} was used.`,
});

export const USAGE_INVALID_QUANTITY = (quantity: string): UsageCode => ({
    ja: `無効な数量: ${quantity}`,
    en: `Invalid quantity: ${quantity}`,
});

export const USAGE_UNKNOWN_BREAD_TYPE = (bread: string): UsageCode => ({
    ja: `不明なﾊﾟﾝの種類: ${bread}`,
    en: `Unknown bread type: ${bread}`,
});

export const USAGE_UPDATED_PRODUCTION_PLAN = (
    bread: string,
    quantity: number,
): UsageCode => ({
    ja: `${bread} の製造計画を更新しました: ${quantity}`,
    en: `Updated production plan for ${bread}: ${quantity}`,
});

export const USAGE_MIXING_REPAIR_PROGRESS = (spinner: string): UsageCode => ({
    ja: `混錬機を修理中... ${spinner}`,
    en: `Repairing cooling unit... ${spinner}`,
});

export const USAGE_FOREIGN_OBJECT_DETECTED: UsageCode = {
    ja: "異物が検出されました",
    en: "Foreign object detected",
};

export const USAGE_FOREIGN_OBJECT_REMOVED: UsageCode = {
    ja: "異物が取り除かれました",
    en: "Foreign object removed",
};
