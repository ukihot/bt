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
import { CoolingCommands } from "../utils/Command";
import {
    USAGE_COOLING_REPAIR_FAILURE,
    USAGE_COOLING_REPAIR_PROGRESS,
    USAGE_COOLING_OVERHEAT_REPAIRED,
    USAGE_UNKNOWN_COOLING_COMMAND,
} from "../utils/usage/usageCooling";
import { USAGE_EMPTY } from "../utils/usage/usageGeneral";

export const useCoolingCommand = (
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
        const coolingTerminal = terminals.find(
            (terminal) => terminal.id === TerminalSectionId.Cooling,
        );

        if (
            coolingTerminal &&
            coolingTerminal.status === TerminalStatus.OVERHEATING
        ) {
            let progress = 0;
            const spinner = ["／", "ー", "＼", "｜"];
            let spinnerIndex = 0;
            addOutput(USAGE_EMPTY);

            const updateProgress = () =>
                new Promise<void>((resolve) => {
                    const intervalId = setInterval(() => {
                        replaceOutput(
                            USAGE_COOLING_REPAIR_PROGRESS(
                                spinner[spinnerIndex],
                            ),
                        );

                        progress += 10;
                        context.updateProgress(
                            TerminalSectionId.Cooling,
                            progress,
                        );
                        spinnerIndex = (spinnerIndex + 1) % spinner.length;

                        if (progress >= 100) {
                            context.updateProgress(
                                TerminalSectionId.Cooling,
                                100,
                            );
                            clearInterval(intervalId);
                            resolve();
                        }
                    }, 500);
                });

            await updateProgress();

            updateTerminalStatus(coolingTerminal.id, TerminalStatus.HEALTHY);
            addOutput(USAGE_COOLING_OVERHEAT_REPAIRED);
        } else {
            addOutput(USAGE_COOLING_REPAIR_FAILURE, LogLevel.WARN);
        }
    }, [terminals, updateTerminalStatus, addOutput, context, replaceOutput]);

    return useCallback(
        async (cmd: CoolingCommands) => {
            switch (cmd) {
                case CoolingCommands.REPAIR:
                    setIsInputEnabled(false);
                    await repairExec();
                    setIsInputEnabled(true);
                    break;
                default:
                    addOutput(
                        USAGE_UNKNOWN_COOLING_COMMAND(cmd),
                        LogLevel.ERROR,
                    );
            }
        },
        [repairExec, addOutput, setIsInputEnabled],
    );
};
