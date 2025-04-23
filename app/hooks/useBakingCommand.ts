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
import { BakingCommands } from "../utils/Command";
import {
    USAGE_GAS_REPAIR_FAILURE,
    USAGE_BAKING_REPAIR_PROGRESS,
    USAGE_GAS_REPAIR_SUCCESS,
    USAGE_UNKNOWN_BAKING_COMMAND,
} from "../utils/usage/usageBaking";
import { USAGE_EMPTY } from "../utils/usage/usageGeneral";

export const useBakingCommand = (
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
        const bakingTerminal = terminals.find(
            (terminal) => terminal.id === TerminalSectionId.Baking,
        );

        if (
            bakingTerminal &&
            bakingTerminal.status === TerminalStatus.GAS_LEAK
        ) {
            let progress = 0;
            const spinner = ["／", "ー", "＼", "｜"];
            let spinnerIndex = 0;
            addOutput(USAGE_EMPTY);

            const updateProgress = () =>
                new Promise<void>((resolve) => {
                    const intervalId = setInterval(() => {
                        replaceOutput(
                            USAGE_BAKING_REPAIR_PROGRESS(spinner[spinnerIndex]),
                        );
                        progress += 10;
                        context.updateProgress(
                            TerminalSectionId.Baking,
                            progress,
                        );
                        spinnerIndex = (spinnerIndex + 1) % spinner.length;

                        if (progress >= 100) {
                            context.updateProgress(
                                TerminalSectionId.Baking,
                                100,
                            );
                            clearInterval(intervalId);
                            resolve();
                        }
                    }, 500);
                });

            await updateProgress();

            updateTerminalStatus(bakingTerminal.id, TerminalStatus.HEALTHY);
            addOutput(USAGE_GAS_REPAIR_SUCCESS);
        } else {
            addOutput(USAGE_GAS_REPAIR_FAILURE, LogLevel.WARN);
        }
    }, [terminals, updateTerminalStatus, addOutput, replaceOutput, context]);

    return useCallback(
        async (cmd: BakingCommands) => {
            switch (cmd) {
                case BakingCommands.REPAIR:
                    setIsInputEnabled(false);
                    await repairExec();
                    setIsInputEnabled(true);
                    break;
                default:
                    addOutput(
                        USAGE_UNKNOWN_BAKING_COMMAND(cmd),
                        LogLevel.ERROR,
                    );
            }
        },
        [repairExec, addOutput, setIsInputEnabled],
    );
};
