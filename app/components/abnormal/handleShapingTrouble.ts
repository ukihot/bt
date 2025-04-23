import { TerminalStatus } from "@/app/bt.types";
import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import { USAGE_SHAPING_SOFTWARE_ERROR } from "@/app/utils/usage/usageShaping";

export const handleShapingTrouble = (context: TerminalContextType) => {
    const { addNews, updateTerminalStatus } = context;

    updateTerminalStatus(
        TerminalSectionId.Shaping,
        TerminalStatus.WRONG_BREAD_PRODUCED,
    );

    addNews(TerminalSectionId.Shaping, USAGE_SHAPING_SOFTWARE_ERROR);
};
