import type { Ingredient } from "@/app/bt.types";
import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import { USAGE_INGREDIENT_COST } from "@/app/utils/usage/usagePurchasing";

export const handlePurchasingBatch = (context: TerminalContextType) => {
    const { ingredientCost, addNews, updateIngredientCost, nigiwai } = context;

    // ランダムな原料を選択し、価格を変動させる
    const ingredients = Object.keys(ingredientCost);
    const randomIngredient = ingredients[
        Math.floor(Math.random() * ingredients.length)
    ] as keyof Ingredient;
    const currentCost = ingredientCost[randomIngredient];

    let newCost = currentCost * Math.log(Math.abs(nigiwai));

    // newCostが0.1を下回らないようにする
    if (newCost < 0.1) {
        newCost = 0.1;
    }

    updateIngredientCost({ [randomIngredient]: newCost });
    addNews(
        TerminalSectionId.Purchasing,
        USAGE_INGREDIENT_COST(ingredientCost),
    );
};
