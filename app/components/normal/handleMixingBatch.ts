import { BreadCookingStatus, BreadType, type Ingredient } from "@/app/bt.types";
import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import { breadRecipes } from "@/app/utils/breadRecipe";
import {
    USAGE_INGREDIENT_CONSUMPTION,
    USAGE_MIXING_ERROR,
    USAGE_MIXING_SUCCESS,
} from "@/app/utils/usage/usageMixing";
import { v4 as uuidv4 } from "uuid";

export const handleMixingBatch = async (context: TerminalContextType) => {
    const { addNews, updateRepository, updateBread, productionPlan } = context;

    const newBread = breadRecipes.flatMap((recipe) => {
        const breadKind = Object.keys(BreadType).find(
            (key) =>
                BreadType[key as keyof typeof BreadType] === recipe.kind &&
                Number.isNaN(Number(key)), // Enumの数値キーを除外
        ) as keyof typeof BreadType;

        const requiredQuantity = productionPlan[BreadType[breadKind]]; // 製造計画に基づく必要数

        if (requiredQuantity <= 0) return []; // 必要数が0以下の場合はスキップ

        const requiredIngredients = calculateRequiredIngredients(
            recipe,
            requiredQuantity,
        );

        const isSuccess = updateRepository(false, requiredIngredients);

        if (!isSuccess) {
            addNews(TerminalSectionId.Mixing, USAGE_MIXING_ERROR(breadKind));
            return [];
        }

        addIngredientConsumptionNews(addNews, requiredIngredients);
        addNews(TerminalSectionId.Mixing, USAGE_MIXING_SUCCESS(breadKind));

        return generateBread(recipe.kind as BreadType, requiredQuantity);
    });

    updateBread([...context.bread, ...newBread]);
};

// 必要な材料を計算
const calculateRequiredIngredients = (
    recipe: Record<string, number | string>,
    quantity: number,
) =>
    Object.entries(recipe).reduce(
        (acc, [key, value]) => {
            if (key !== "kind") {
                acc[key as keyof Ingredient] =
                    (typeof value === "number" ? value : 0) * quantity;
            }
            return acc;
        },
        {} as Partial<Ingredient>,
    );

// 材料消費のニュースを追加
const addIngredientConsumptionNews = (
    addNews: TerminalContextType["addNews"],
    requiredIngredients: Partial<Ingredient>,
) => {
    for (const [ingredient, amount] of Object.entries(requiredIngredients)) {
        if (amount > 0) {
            addNews(
                TerminalSectionId.Mixing,
                USAGE_INGREDIENT_CONSUMPTION(ingredient, amount),
            );
        }
    }
};

// パンを生成
const generateBread = (recipeKind: BreadType, quantity: number) =>
    Array.from({ length: quantity }, () => ({
        id: uuidv4(),
        kind: recipeKind,
        cookStatus: BreadCookingStatus.Raw,
    }));
