import type { Ingredient, UsageCode } from "../../bt.types";

export const USAGE_INVALID_ITEM_BUY: UsageCode = {
    ja: "'buy'ｺﾏﾝﾄﾞで指定されたｱｲﾃﾑが無効です。有効な原料を指定してください。",
    en: "Invalid item specified for 'buy' command. Please specify a valid ingredient.",
};

export const USAGE_INSUFFICIENT_FUNDS = (
    item: string,
    quantity: number,
): UsageCode => ({
    ja: `${item}を${quantity}kg購入するのに十分な資金がありません。`,
    en: `Insufficient funds to purchase ${quantity} kg of ${item}.`,
});

export const USAGE_PURCHASED_ITEM = (
    item: string,
    quantity: number,
    subtotal: number,
): UsageCode => ({
    ja: `購入したｱｲﾃﾑ: ${item}, 数量: ${quantity}, ｺｽﾄ: $${subtotal.toFixed(
        2,
    )}`,
    en: `Purchased item: ${item}, Quantity: ${quantity}, Cost: $${subtotal.toFixed(
        2,
    )}`,
});

export const USAGE_PURCHASING_PROGRESS = (spinner: string): UsageCode => ({
    ja: `購入中... ${spinner}`,
    en: `Purchasing... ${spinner}`,
});

export const USAGE_PURCHASE_COMPLETED: UsageCode = {
    ja: "購入が完了しました。",
    en: "Purchase completed.",
};

export const USAGE_PURCHASED_STOCK = (
    item: string,
    quantity: number,
): UsageCode => ({
    ja: `${item}を購入し、${quantity}kgが在庫に追加されました。`,
    en: `Purchased ${item}, ${quantity}kg added to stock.`,
});

export const USAGE_UNKNOWN_PURCHASING_COMMAND = (
    command: string,
): UsageCode => ({
    ja: `不明な購入ｺﾏﾝﾄﾞ: ${command}`,
    en: `Unknown purchasing command: ${command}`,
});

export const USAGE_INGREDIENT_COST = (
    ingredientCosts: Ingredient,
): UsageCode => {
    const costDescriptions = Object.entries(ingredientCosts)
        .map(([ingredient, cost]) => `${ingredient}は$${cost.toFixed(2)}`)
        .join(", ");
    return {
        ja: `${costDescriptions}です。`,
        en: `${costDescriptions}.`,
    };
};
