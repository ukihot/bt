import {
    DEFAULT_CASH,
    type TerminalContextType,
} from "../context/TerminalContext";

export const getCurrentLevel = (context: TerminalContextType): number => {
    const { cash } = context;
    const difference = cash - DEFAULT_CASH;
    if (difference <= 0) {
        return 1;
    }
    return Math.max(1, Math.floor(Math.log10(Math.abs(difference))) + 1);
};
