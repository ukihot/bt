import { useCallback, useContext } from "react";
import {
    BreadCookingStatus,
    LogLevel,
    TerminalSectionId,
    type UsageCode,
} from "../bt.types";
import { TerminalContext } from "../context/TerminalContext";
import { PackagingCommands } from "../utils/Command";
import {
    USAGE_NO_BAKED_BREAD_OF_TYPE,
    USAGE_PACKAGING_SUCCESS,
    USAGE_UNKNOWN_BREAD_TYPE,
    USAGE_UNKNOWN_PACKAGING_COMMAND,
} from "../utils/usage/usagePackaging";
import { findMatchingBread } from "../utils/findMatchingBread";

export const usePackagingCommand = (
    addOutput: (usage: UsageCode, level?: LogLevel) => void,
) => {
    const context = useContext(TerminalContext);
    if (!context) {
        throw new Error("TerminalContext is not available");
    }

    const sealExec = useCallback(
        (bread: string) => {
            const { bread: breadList, updateBread, addNews } = context;

            // 0. パンの種類をパース
            const breadType = findMatchingBread(bread);
            if (!breadType) {
                addOutput(USAGE_UNKNOWN_BREAD_TYPE(bread), LogLevel.ERROR);
                return;
            }

            // 1. Baked 状態のパンを検索
            const bakedBread = breadList.filter(
                (b) =>
                    b.cookStatus === BreadCookingStatus.Baked &&
                    b.kind === breadType,
            );
            if (bakedBread.length === 0) {
                addOutput(
                    USAGE_NO_BAKED_BREAD_OF_TYPE(breadType),
                    LogLevel.WARN,
                );
                return;
            }

            // 2. Packaged 状態に更新
            const updatedBread = breadList.map((b) =>
                b.cookStatus === BreadCookingStatus.Baked &&
                b.kind === breadType
                    ? { ...b, cookStatus: BreadCookingStatus.Packaged }
                    : b,
            );
            updateBread(updatedBread);

            // 3. 完了メッセージを出力
            addNews(
                TerminalSectionId.Packaging,
                USAGE_PACKAGING_SUCCESS(bakedBread.length, breadType),
            );
        },
        [context, addOutput],
    );

    return (cmd: PackagingCommands, ...args: string[]) => {
        switch (cmd) {
            case PackagingCommands.SEAL:
                sealExec(args[0]);
                break;
            default:
                addOutput(USAGE_UNKNOWN_PACKAGING_COMMAND(cmd), LogLevel.ERROR);
        }
    };
};
