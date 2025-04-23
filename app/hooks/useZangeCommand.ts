import { useCallback, useContext } from "react";
import { LogLevel, TerminalSectionId, type UsageCode } from "../bt.types";
import { TerminalContext } from "../context/TerminalContext";
import { ZangeCommands } from "../utils/Command";
import {
    USAGE_UNKNOWN_ZANGE_COMMAND,
    USAGE_ZANGE_ESCAPE_FAILURE,
    USAGE_ZANGE_ESCAPE_SUCCESS,
} from "../utils/usage/usageZange";

export const useZangeCommand = (
    addOutput: (usage: UsageCode, level?: LogLevel) => void,
    setMode: React.Dispatch<React.SetStateAction<TerminalSectionId>>,
) => {
    const context = useContext(TerminalContext);
    if (!context) {
        throw new Error("TerminalContext is not available");
    }
    const { nigiwai } = context;
    const zangeExec = useCallback(() => {
        if (Math.random() < nigiwai / 10) {
            setMode(TerminalSectionId.Waste);
            addOutput(USAGE_ZANGE_ESCAPE_SUCCESS);
        } else {
            addOutput(USAGE_ZANGE_ESCAPE_FAILURE, LogLevel.ERROR);
        }
    }, [addOutput, nigiwai, setMode]);

    return (cmd: ZangeCommands) => {
        switch (cmd) {
            case ZangeCommands.Zange:
                zangeExec();
                break;
            default:
                addOutput(USAGE_UNKNOWN_ZANGE_COMMAND(cmd), LogLevel.ERROR);
        }
    };
};
