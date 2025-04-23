import {
    type Dispatch,
    type SetStateAction,
    useCallback,
    useContext,
} from "react";
import { type Ingredient, LogLevel, type UsageCode } from "../bt.types";
import { TerminalContext, TerminalSectionId } from "../context/TerminalContext";
import { PurchasingCommands } from "../utils/Command";
import { USAGE_EMPTY } from "../utils/usage/usageGeneral";
import {
    USAGE_INSUFFICIENT_FUNDS,
    USAGE_INVALID_ITEM_BUY,
    USAGE_PURCHASED_ITEM,
    USAGE_PURCHASED_STOCK,
    USAGE_PURCHASE_COMPLETED,
    USAGE_PURCHASING_PROGRESS,
    USAGE_UNKNOWN_PURCHASING_COMMAND,
} from "../utils/usage/usagePurchasing";

export const usePurchasingCommand = (
    addOutput: (usage: UsageCode, level?: LogLevel) => void,
    replaceOutput: (usage: UsageCode, level?: LogLevel) => void,
    setIsInputEnabled: Dispatch<SetStateAction<boolean>>,
) => {
    const context = useContext(TerminalContext);
    if (!context) {
        throw new Error("TerminalContext is not available");
    }

    const buyExec = useCallback(
        async (itemStr: string, quantity: number) => {
            // itemStr が Ingredient のキーであるかをチェック
            if (!Object.keys(context.ingredientCost).includes(itemStr)) {
                addOutput(USAGE_INVALID_ITEM_BUY, LogLevel.ERROR);
                return;
            }
            const item = itemStr as keyof Ingredient;

            // 小計
            const costPerUnit = context.ingredientCost[item] ?? 0;
            const subtotal = costPerUnit * quantity;

            // 資金チェックと更新
            try {
                context.updateCash("expense", subtotal);
            } catch (_) {
                addOutput(
                    USAGE_INSUFFICIENT_FUNDS(item, quantity),
                    LogLevel.ERROR,
                );
                return;
            }

            context.addNews(
                TerminalSectionId.Purchasing,
                USAGE_PURCHASED_ITEM(item, quantity, subtotal),
            );

            let progress = 0;
            const spinner = ["／", "ー", "＼", "｜"];
            let spinnerIndex = 0;
            addOutput(USAGE_EMPTY);
            const updateProgress = () =>
                new Promise<void>((resolve) => {
                    const intervalId = setInterval(() => {
                        replaceOutput(
                            USAGE_PURCHASING_PROGRESS(spinner[spinnerIndex]),
                        );

                        progress += 10;
                        context.updateProgress(
                            TerminalSectionId.Purchasing,
                            progress,
                        );
                        spinnerIndex = (spinnerIndex + 1) % spinner.length;

                        if (progress >= 100) {
                            context.updateProgress(
                                TerminalSectionId.Purchasing,
                                100,
                            );
                            clearInterval(intervalId);
                            resolve();
                        }
                    }, 500);
                });

            await updateProgress();

            // 在庫更新
            context.updateRepository(true, { [item]: quantity });
            // 完了
            addOutput(USAGE_PURCHASE_COMPLETED, LogLevel.INFO);

            context.addNews(
                TerminalSectionId.Pantry,
                USAGE_PURCHASED_STOCK(item, quantity),
            );
        },
        [addOutput, replaceOutput, context],
    );

    return useCallback(
        async (command: PurchasingCommands, ...args: string[]) => {
            switch (command) {
                case PurchasingCommands.BUY:
                    setIsInputEnabled(false);
                    await buyExec(args[0], Number.parseInt(args[1])); // 原料名と数量
                    setIsInputEnabled(true);
                    break;
                default:
                    addOutput(
                        USAGE_UNKNOWN_PURCHASING_COMMAND(command),
                        LogLevel.ERROR,
                    );
            }
        },
        [buyExec, setIsInputEnabled, addOutput],
    );
};
