import {
    type Dispatch,
    type SetStateAction,
    useCallback,
    useContext,
} from "react";
import {
    LogLevel,
    TerminalSectionId,
    TerminalStatus,
    type UsageCode,
} from "../bt.types";
import { TerminalContext } from "../context/TerminalContext";
import { ShapingCommands } from "../utils/Command";
import { USAGE_EMPTY } from "../utils/usage/usageGeneral";
import {
    USAGE_SHAPING_REPAIR_FAILURE,
    USAGE_SHAPING_REPAIR_PROGRESS,
    USAGE_SHAPING_REPAIR_SUCCESS,
    USAGE_UNKNOWN_SHAPING_COMMAND,
} from "../utils/usage/usageShaping";

export const useShapingCommand = (
    addOutput: (usage: UsageCode, level?: LogLevel) => void,
    replaceOutput: (usage: UsageCode, level?: LogLevel) => void,
    setIsInputEnabled: Dispatch<SetStateAction<boolean>>,
) => {
    const context = useContext(TerminalContext);
    if (!context) {
        throw new Error("TerminalContext is not available");
    }
    const { terminals, updateTerminalStatus } = context;

    const repairExec = useCallback(async () => {
        const shapingTerminal = terminals.find(
            (terminal) => terminal.id === TerminalSectionId.Shaping,
        );

        if (
            shapingTerminal &&
            shapingTerminal.status === TerminalStatus.WRONG_BREAD_PRODUCED
        ) {
            let progress = 0;
            const spinner = ["／", "ー", "＼", "｜"];
            let spinnerIndex = 0;
            addOutput(USAGE_EMPTY);

            const updateProgress = () =>
                new Promise<void>((resolve) => {
                    const intervalId = setInterval(() => {
                        replaceOutput(
                            USAGE_SHAPING_REPAIR_PROGRESS(
                                spinner[spinnerIndex],
                            ),
                        );

                        progress += 10;
                        context.updateProgress(
                            TerminalSectionId.Shaping,
                            progress,
                        );
                        spinnerIndex = (spinnerIndex + 1) % spinner.length;

                        if (progress >= 100) {
                            context.updateProgress(
                                TerminalSectionId.Shaping,
                                100,
                            );
                            clearInterval(intervalId);
                            resolve();
                        }
                    }, 500);
                });

            await updateProgress();

            updateTerminalStatus(shapingTerminal.id, TerminalStatus.HEALTHY);
            addOutput(USAGE_SHAPING_REPAIR_SUCCESS);
        } else {
            addOutput(USAGE_SHAPING_REPAIR_FAILURE, LogLevel.WARN);
        }
    }, [terminals, updateTerminalStatus, addOutput, context, replaceOutput]);

    return useCallback(
        async (cmd: ShapingCommands) => {
            switch (cmd) {
                case ShapingCommands.REPAIR:
                    setIsInputEnabled(false);
                    await repairExec();
                    setIsInputEnabled(true);
                    break;
                default:
                    addOutput(
                        USAGE_UNKNOWN_SHAPING_COMMAND(cmd),
                        LogLevel.ERROR,
                    );
            }
        },
        [repairExec, addOutput, setIsInputEnabled],
    );
};
