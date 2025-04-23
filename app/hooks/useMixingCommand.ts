import {
    type Dispatch,
    type SetStateAction,
    useCallback,
    useContext,
} from "react";
import {
    BreadType,
    LogLevel,
    TerminalSectionId,
    TerminalStatus,
    type UsageCode,
} from "../bt.types";
import { TerminalContext } from "../context/TerminalContext";
import { MixingCommands } from "../utils/Command";
import { findMatchingBread } from "../utils/findMatchingBread";
import { USAGE_EMPTY } from "../utils/usage/usageGeneral";
import {
    USAGE_FOREIGN_OBJECT_REMOVED,
    USAGE_INVALID_QUANTITY,
    USAGE_MISSING_MIXING_COMMAND,
    USAGE_MIXING_REPAIR_FAILURE,
    USAGE_MIXING_REPAIR_PROGRESS,
    USAGE_UNKNOWN_BREAD_TYPE,
    USAGE_UNKNOWN_MIXING_COMMAND,
    USAGE_UPDATED_PRODUCTION_PLAN,
} from "../utils/usage/usageMixing";

export const useMixingCommand = (
    addOutput: (usage: UsageCode, level?: LogLevel) => void,
    replaceOutput: (usage: UsageCode, level?: LogLevel) => void,
    setIsInputEnabled: Dispatch<SetStateAction<boolean>>,
) => {
    const context = useContext(TerminalContext);
    if (!context) {
        throw new Error("TerminalContext is not available");
    }
    const {
        terminals,
        updateTerminalStatus,
        productionPlan,
        updateProductionPlan,
    } = context;

    // 製造予定個数を変更するｺﾏﾝﾄﾞ
    const planExec = useCallback(
        (bread: string | undefined, plannedQuantity: string | undefined) => {
            if (!bread && !plannedQuantity) {
                const planSummary = Object.entries(productionPlan)
                    .map(
                        ([breadType, quantity]) =>
                            `${BreadType[breadType as unknown as BreadType]}: ${quantity}`,
                    )
                    .join("\n");
                addOutput(
                    {
                        ja: `現在の製造計画:\n${planSummary}`,
                        en: `Current production plan:\n${planSummary}`,
                    },
                    LogLevel.INFO,
                );
                return;
            }

            const breadType = findMatchingBread(bread);

            if (!bread || !plannedQuantity || !breadType) {
                addOutput(USAGE_MISSING_MIXING_COMMAND, LogLevel.ERROR);
                return;
            }

            const quantity = Number.parseInt(plannedQuantity, 10);

            if (Number.isNaN(quantity) || quantity < 0) {
                addOutput(
                    USAGE_INVALID_QUANTITY(plannedQuantity),
                    LogLevel.ERROR,
                );
                return;
            }

            if (!breadType) {
                addOutput(USAGE_UNKNOWN_BREAD_TYPE(breadType), LogLevel.ERROR);
                return;
            }

            if (breadType) {
                updateProductionPlan(breadType, quantity);
            }
            addOutput(
                USAGE_UPDATED_PRODUCTION_PLAN(breadType, quantity),
                LogLevel.INFO,
            );
        },
        [updateProductionPlan, addOutput, productionPlan],
    );

    const repairExec = useCallback(async () => {
        const mixingTerminal = terminals.find(
            (terminal) => terminal.id === TerminalSectionId.Mixing,
        );

        if (
            mixingTerminal &&
            mixingTerminal.status === TerminalStatus.CONTAMINATION_DETECTED
        ) {
            let progress = 0;
            const spinner = ["／", "ー", "＼", "｜"];
            let spinnerIndex = 0;
            addOutput(USAGE_EMPTY);

            const updateProgress = () =>
                new Promise<void>((resolve) => {
                    const intervalId = setInterval(() => {
                        replaceOutput(
                            USAGE_MIXING_REPAIR_PROGRESS(spinner[spinnerIndex]),
                        );

                        progress += 10;
                        context.updateProgress(
                            TerminalSectionId.Mixing,
                            progress,
                        );
                        spinnerIndex = (spinnerIndex + 1) % spinner.length;

                        if (progress >= 100) {
                            context.updateProgress(
                                TerminalSectionId.Mixing,
                                100,
                            );
                            clearInterval(intervalId);
                            resolve();
                        }
                    }, 500);
                });

            await updateProgress();

            updateTerminalStatus(mixingTerminal.id, TerminalStatus.HEALTHY);
            addOutput(USAGE_FOREIGN_OBJECT_REMOVED);
        } else {
            addOutput(USAGE_MIXING_REPAIR_FAILURE, LogLevel.WARN);
        }
    }, [terminals, updateTerminalStatus, addOutput, context, replaceOutput]);

    return useCallback(
        async (cmd: MixingCommands, ...args: string[]) => {
            switch (cmd) {
                case MixingCommands.REPAIR:
                    setIsInputEnabled(false);
                    await repairExec();
                    setIsInputEnabled(true);
                    break;
                case MixingCommands.PLAN:
                    planExec(args[0], args[1]);
                    break;
                default:
                    addOutput(
                        USAGE_UNKNOWN_MIXING_COMMAND(cmd),
                        LogLevel.ERROR,
                    );
            }
        },
        [repairExec, planExec, addOutput, setIsInputEnabled],
    );
};
