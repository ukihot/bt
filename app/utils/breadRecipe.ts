import { BreadType, type Ingredient } from "../bt.types";

export const DEFAULT_INGREDIENT_COST: Ingredient = {
    flour: 0.2,
    yeast: 15.0,
    salt: 0.1,
    butter: 2.4,
    sugar: 0.3,
    milk: 0.2,
    redBeanPaste: 1.1,
    malt: 2.3,
};

export const breadRecipes = [
    {
        kind: BreadType.Anpan,
        flour: 40,
        yeast: 0.6,
        salt: 0.6,
        butter: 4,
        sugar: 5,
        milk: 2,
        redBeanPaste: 30,
        malt: 0,
    },
    {
        kind: BreadType.Begguette,
        flour: 133,
        yeast: 0.2,
        salt: 2.7,
        butter: 0,
        sugar: 0,
        milk: 0,
        redBeanPaste: 0,
        malt: 1,
    },
    {
        kind: BreadType.Croissant,
        flour: 35.7,
        yeast: 1.4,
        salt: 0.7,
        butter: 18.6,
        sugar: 2.5,
        milk: 24.3,
        redBeanPaste: 0,
        malt: 0,
    },
    {
        kind: BreadType.Naan,
        flour: 50,
        yeast: 1,
        salt: 1,
        butter: 0,
        sugar: 2,
        milk: 0,
        redBeanPaste: 0,
        malt: 0,
    },
];
