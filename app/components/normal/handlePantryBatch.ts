import { TerminalSectionId, TerminalStatus } from "@/app/bt.types";
import type { TerminalContextType } from "@/app/context/TerminalContext";
import { USAGE_FIRE_DESTROYED_ALL } from "@/app/utils/usage/usagePantry";

export const handlePantryBatch = (context: TerminalContextType) => {
    const { hasTerminalWithStatus, repository, updateRepository, addNews } =
        context;

    if (hasTerminalWithStatus(TerminalStatus.FIRE_HAZARD)) {
        // 焼失
        updateRepository(false, repository);
        addNews(TerminalSectionId.Pantry, USAGE_FIRE_DESTROYED_ALL);
    }
};
