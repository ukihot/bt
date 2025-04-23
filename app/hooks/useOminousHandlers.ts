import type { TerminalSectionId } from "../bt.types";
import { handleSorryMan } from "../components/ominous/handleSorryMan";
import type { TerminalContextType } from "../context/TerminalContext";

export const useOminousHandlers = (context: TerminalContextType) => {
    return {
        handleSorryMan: (terminalId: TerminalSectionId) =>
            handleSorryMan(context, terminalId),
    };
};
