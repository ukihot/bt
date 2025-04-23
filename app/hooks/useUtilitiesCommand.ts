import { useCallback, useContext } from "react";
import { LogLevel, type UsageCode } from "../bt.types";
import { TerminalContext } from "../context/TerminalContext";
import { UtilitiesCommands } from "../utils/Command";
import {
    USAGE_INVALID_TEMPERATURE_OR_ID,
    USAGE_MISSING_TEMPERATURE_OR_ID,
    USAGE_TERMINAL_NOT_FOUND,
    USAGE_UNKNOWN_UTILITIES_COMMAND,
    USAGE_UPDATED_TEMPERATURE,
} from "../utils/usage/usageUtilities";

export const useUtilitiesCommand = (
    addOutput: (usage: UsageCode, level?: LogLevel) => void,
) => {
    const context = useContext(TerminalContext);
    if (!context) {
        throw new Error("TerminalContext is not available");
    }
    const { terminals, decreaseTemperature, increaseTemperature, addNews } =
        context;

    const adjustTemperature = useCallback(
        (
            terminalId: number,
            newTemperature: number,
            currentTemperature: number,
        ) => {
            const temperatureDifference = newTemperature - currentTemperature;
            temperatureDifference > 0
                ? increaseTemperature(terminalId, temperatureDifference)
                : decreaseTemperature(
                      terminalId,
                      Math.abs(temperatureDifference),
                  );
        },
        [increaseTemperature, decreaseTemperature],
    );

    const tempExec = useCallback(
        (terminalIdStr?: string, temperatureStr?: string) => {
            // 入力値の検証
            if (!terminalIdStr || !temperatureStr) {
                addOutput(USAGE_MISSING_TEMPERATURE_OR_ID, LogLevel.ERROR);
                return;
            }

            const terminalId = Number.parseInt(terminalIdStr, 10);
            const newTemperature = Number.parseFloat(temperatureStr);

            // 数値変換の検証
            if (Number.isNaN(terminalId) || Number.isNaN(newTemperature)) {
                addOutput(USAGE_INVALID_TEMPERATURE_OR_ID, LogLevel.ERROR);
                return;
            }

            // 対象ﾀｰﾐﾅﾙの検索
            const terminal = terminals.find((t) => t.id === terminalId);
            if (!terminal) {
                addOutput(USAGE_TERMINAL_NOT_FOUND(terminalId), LogLevel.ERROR);
                return;
            }

            // 温度調整処理
            adjustTemperature(
                terminalId,
                newTemperature,
                terminal.barometer.roomTemperature,
            );

            // 結果の通知
            const usageMessage = USAGE_UPDATED_TEMPERATURE(
                terminalId,
                newTemperature,
            );
            addNews(terminalId, usageMessage);
            addOutput(usageMessage, LogLevel.INFO);
        },
        [terminals, adjustTemperature, addNews, addOutput],
    );

    return (cmd: UtilitiesCommands, ...args: string[]) => {
        switch (cmd) {
            case UtilitiesCommands.TEMP:
                tempExec(args[0], args[1]);
                break;
            default:
                addOutput(USAGE_UNKNOWN_UTILITIES_COMMAND(cmd), LogLevel.ERROR);
        }
    };
};
