import {
    OminousType,
    TerminalStatus,
    type TerminalSectionId,
} from "@/app/bt.types";
import type { TerminalContextType } from "@/app/context/TerminalContext";
import { USAGE_SORRY } from "@/app/utils/usage/usageOminous";

export const handleSorryMan = (
    context: TerminalContextType,
    terminalId: TerminalSectionId,
) => {
    const { addNews, updateOmninous, updateTerminalStatus } = context;

    updateOmninous(OminousType.Sorryman);
    updateTerminalStatus(terminalId, TerminalStatus.ON_BREAK);
    addNews(terminalId, USAGE_SORRY, true);
};
