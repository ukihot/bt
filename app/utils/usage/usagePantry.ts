import type { UsageCode } from "../../bt.types";

export const USAGE_ERROR_TERMINAL_CONTEXT: UsageCode = {
    ja: "ｴﾗｰ: ﾀｰﾐﾅﾙｺﾝﾃｷｽﾄが利用できません。",
    en: "Error: Terminal context not available.",
};

export const USAGE_INGREDIENT_ITEM = (
    key: string,
    value: number,
): UsageCode => ({
    ja: `${key} = ${value.toFixed(3)} g`,
    en: `${key} = ${value.toFixed(3)} g`,
});

export const USAGE_UNKNOWN_PANTRY_COMMAND = (cmd: string): UsageCode => ({
    ja: `不明なﾊﾟﾝﾄﾘｰｺﾏﾝﾄﾞ: ${cmd}`,
    en: `Unknown pantry command: ${cmd}`,
});

export const USAGE_RODENT_EATING: UsageCode = {
    ja: "かじられた跡があります。在庫が減少しています。",
    en: "Gnaw marks found. Inventory has decreased.",
};

export const USAGE_FIRE_DESTROYED_ALL: UsageCode = {
    ja: "火災で原料がすべて焼失しました。",
    en: "All ingredients were destroyed in the fire.",
};
