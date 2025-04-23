import { useCallback, useContext } from "react";
import { LogLevel, type UsageCode } from "../bt.types";
import { TerminalContext } from "../context/TerminalContext";
import { SalesFrontCommands } from "../utils/Command";
import { USAGE_UNKNOWN_SALESFRONT_COMMAND } from "../utils/usage/usageSalesFront";

export const useSalesFrontCommand = (
    addOutput: (usage: UsageCode, level?: LogLevel) => void,
) => {
    const context = useContext(TerminalContext);
    if (!context) {
        throw new Error("TerminalContext is not available");
    }
    const salesExec = useCallback(() => {}, []);

    return (cmd: SalesFrontCommands) => {
        switch (cmd) {
            case SalesFrontCommands.SELL:
                salesExec();
                break;
            default:
                addOutput(
                    USAGE_UNKNOWN_SALESFRONT_COMMAND(cmd),
                    LogLevel.ERROR,
                );
        }
    };
};
