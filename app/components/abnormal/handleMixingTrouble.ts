import { TerminalStatus } from "@/app/bt.types";
import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import { USAGE_FOREIGN_OBJECT_DETECTED } from "@/app/utils/usage/usageMixing";

export const handleMixingTrouble = (context: TerminalContextType) => {
    const { addNews, updateTerminalStatus } = context;

    updateTerminalStatus(
        TerminalSectionId.Mixing,
        TerminalStatus.CONTAMINATION_DETECTED,
    );

    addNews(TerminalSectionId.Mixing, USAGE_FOREIGN_OBJECT_DETECTED);
};
