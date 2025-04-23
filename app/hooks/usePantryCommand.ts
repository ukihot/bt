import { useCallback, useContext } from "react";
import { LogLevel, type UsageCode } from "../bt.types";
import { TerminalContext } from "../context/TerminalContext";
import { PantryCommands } from "../utils/Command";
import {
    USAGE_ERROR_TERMINAL_CONTEXT,
    USAGE_INGREDIENT_ITEM,
    USAGE_UNKNOWN_PANTRY_COMMAND,
} from "../utils/usage/usagePantry";

export const usePantryCommand = (
    addOutput: (usage: UsageCode, level?: LogLevel) => void,
) => {
    const terminalContext = useContext(TerminalContext);

    const ingredientExec = useCallback(() => {
        if (!terminalContext) {
            addOutput(USAGE_ERROR_TERMINAL_CONTEXT, LogLevel.ERROR);
            return;
        }

        const repository = terminalContext.repository;

        for (const [key, value] of Object.entries(repository)) {
            addOutput(USAGE_INGREDIENT_ITEM(key, value), LogLevel.INFO);
        }
    }, [terminalContext, addOutput]);

    const breadExec = useCallback(() => {
        if (!terminalContext) {
            addOutput(USAGE_ERROR_TERMINAL_CONTEXT, LogLevel.ERROR);
            return;
        }

        const breadStatus = terminalContext.overviewBreadStatus();
        addOutput({ ja: breadStatus }, LogLevel.INFO);
    }, [terminalContext, addOutput]);

    return (cmd: PantryCommands) => {
        switch (cmd) {
            case PantryCommands.INGREDIENT:
                ingredientExec();
                break;
            case PantryCommands.BREAD:
                breadExec();
                break;
            default:
                addOutput(USAGE_UNKNOWN_PANTRY_COMMAND(cmd), LogLevel.ERROR);
        }
    };
};
