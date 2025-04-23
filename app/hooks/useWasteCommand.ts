import { useCallback, useContext } from "react";
import { LogLevel, type UsageCode } from "../bt.types";
import { TerminalContext } from "../context/TerminalContext";
import { WasteCommands } from "../utils/Command";
import { USAGE_UNKNOWN_WASTE_COMMAND } from "../utils/usage/usageWaste";

export const useWasteCommand = (
    addOutput: (usage: UsageCode, level?: LogLevel) => void,
) => {
    const context = useContext(TerminalContext);
    if (!context) {
        throw new Error("TerminalContext is not available");
    }
    const wasteExec = useCallback(() => {}, []);

    return (cmd: WasteCommands) => {
        switch (cmd) {
            case WasteCommands.RECYCLE:
                wasteExec();
                break;
            default:
                addOutput(USAGE_UNKNOWN_WASTE_COMMAND(cmd), LogLevel.ERROR);
        }
    };
};
