import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import { getCurrentLevel } from "@/app/utils/getCurrentLevel";
import { USAGE_TEXTS } from "@/app/utils/usage/usageGeneral";
import { USAGE_RODENT_EATING } from "@/app/utils/usage/usagePantry";

export const handlePantryTrouble = (context: TerminalContextType) => {
    const { addNews, terminals, updateRepository } = context;

    const pantry = terminals.find(
        (terminal) => terminal.id === TerminalSectionId.Pantry,
    );

    if ((pantry?.barometer.rodentCount ?? 0) > 0) {
        const rodentCount = pantry?.barometer.rodentCount;
        const level = getCurrentLevel(context);

        const consumptionRate = (rodentCount ?? 0) + level;

        const consumption = Object.fromEntries(
            Object.entries(context.repository).map(([key, value]) => [
                key,
                value * (consumptionRate / 100),
            ]),
        );

        const success = updateRepository(false, consumption);
        if (success) {
            addNews(TerminalSectionId.Pantry, USAGE_RODENT_EATING);
        }
    }

    addNews(TerminalSectionId.Pantry, USAGE_TEXTS.PANTRY_TROUBLE);
};
