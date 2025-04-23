import { useCallback, useContext } from "react";
import {
    LogLevel,
    TerminalSectionId,
    TerminalStatus,
    type UsageCode,
} from "../bt.types";
import { TerminalContext } from "../context/TerminalContext";
import { GeneralCommands } from "../utils/Command";
import {
    USAGE_ACTIVATING_TERMINAL,
    USAGE_DESCENT_INTO_ZANGE,
    USAGE_DISPOSE_FAILURE,
    USAGE_DISPOSE_SUCCESS,
    USAGE_EMPTY,
    USAGE_HELP_BAKING,
    USAGE_HELP_COOLING,
    USAGE_HELP_HEADER,
    USAGE_HELP_LS,
    USAGE_HELP_MIXING,
    USAGE_HELP_MODE,
    USAGE_HELP_PACKAGING,
    USAGE_HELP_PANTRY,
    USAGE_HELP_PURCHASING,
    USAGE_HELP_SALES_FRONT,
    USAGE_HELP_SHAPING,
    USAGE_HELP_TERM_FORMAT,
    USAGE_HELP_TERM_OPEN,
    USAGE_HELP_UTILITIES,
    USAGE_HELP_WASTE,
    USAGE_INVALID_MODE_ID,
    USAGE_INVALID_OR_MISSING_ID_MODE,
    USAGE_INVALID_TERMINAL_ID,
    USAGE_INVALID_TERM_SUBCOMMAND,
    USAGE_LANGUAGE_CHANGED,
    USAGE_MISSING_ID_TERM_OPEN,
    USAGE_MODE_CHANGED,
    USAGE_REST_FAILURE,
    USAGE_REST_SUCCESS,
    USAGE_TERM_FORMATTED,
    USAGE_TRAP_FAILURE,
    USAGE_TRAP_SET,
    USAGE_UNKNOWN_COMMAND,
    USAGE_UPDATED_TERMINAL_POSITION,
    USAGE_WORK_FAILURE,
    USAGE_WORK_SUCCESS,
} from "../utils/usage/usageGeneral";

export const useGeneralCommand = (
    addOutput: (usage: UsageCode, level?: LogLevel) => void,
    mode: TerminalSectionId,
    setMode: React.Dispatch<React.SetStateAction<TerminalSectionId>>,
) => {
    const context = useContext(TerminalContext);
    if (!context) {
        throw new Error("TerminalContext is not available");
    }

    const {
        terminals,
        updateTerminalPosition,
        activateTerminal,
        updateLanguage,
        updateTerminalStatus,
        disposeWaste,
        accumulateWaste,
        addNews,
        updateTrap,
    } = context;

    const helpExec = useCallback(() => {
        addOutput(USAGE_HELP_HEADER, LogLevel.WARN);
        addOutput(USAGE_EMPTY, LogLevel.WARN);
        addOutput(USAGE_HELP_LS, LogLevel.WARN);
        addOutput(USAGE_HELP_MODE, LogLevel.WARN);
        addOutput(USAGE_HELP_TERM_OPEN, LogLevel.WARN);
        addOutput(USAGE_HELP_TERM_FORMAT, LogLevel.WARN);
        addOutput(USAGE_EMPTY, LogLevel.WARN);

        switch (mode) {
            case TerminalSectionId.Purchasing:
                addOutput(USAGE_HELP_PURCHASING, LogLevel.WARN);
                break;
            case TerminalSectionId.Pantry:
                addOutput(USAGE_HELP_PANTRY, LogLevel.WARN);
                break;
            case TerminalSectionId.Mixing:
                addOutput(USAGE_HELP_MIXING, LogLevel.WARN);
                break;
            case TerminalSectionId.Cooling:
                addOutput(USAGE_HELP_COOLING, LogLevel.WARN);
                break;
            case TerminalSectionId.Shaping:
                addOutput(USAGE_HELP_SHAPING, LogLevel.WARN);
                break;
            case TerminalSectionId.Baking:
                addOutput(USAGE_HELP_BAKING, LogLevel.WARN);
                break;
            case TerminalSectionId.Packaging:
                addOutput(USAGE_HELP_PACKAGING, LogLevel.WARN);
                break;
            case TerminalSectionId.SalesFront:
                addOutput(USAGE_HELP_SALES_FRONT, LogLevel.WARN);
                break;
            case TerminalSectionId.Waste:
                addOutput(USAGE_HELP_WASTE, LogLevel.WARN);
                break;
            case TerminalSectionId.Utilities:
                addOutput(USAGE_HELP_UTILITIES, LogLevel.WARN);
                break;
            default:
                addOutput(USAGE_EMPTY, LogLevel.WARN); // デフォルトで空行を追加
        }
    }, [addOutput, mode]);

    const termExec = useCallback(
        (subCommand: string, id: string | undefined) => {
            switch (subCommand) {
                case "open":
                    if (id) {
                        const terminalId = Number(id);
                        if (
                            Number.isNaN(terminalId) ||
                            !(terminalId in TerminalSectionId)
                        ) {
                            addOutput(
                                USAGE_INVALID_TERMINAL_ID(id),
                                LogLevel.ERROR,
                            );
                            break;
                        }
                        addOutput(
                            USAGE_ACTIVATING_TERMINAL(
                                TerminalSectionId[terminalId],
                            ),
                            LogLevel.INFO,
                        );
                        activateTerminal(terminalId as TerminalSectionId);
                    } else {
                        addOutput(USAGE_MISSING_ID_TERM_OPEN, LogLevel.ERROR);
                    }
                    break;
                case "format": {
                    const spacing = 81; // ﾀｰﾐﾅﾙ間のスペース
                    let currentY = 10; // 現在のY座標を追跡

                    // 画面サイズを取得
                    const screenWidth = window.innerWidth;

                    // ﾀｰﾐﾅﾙをIDの昇順でソート
                    const sortedTerminals = [...terminals].sort(
                        (a, b) => a.id - b.id,
                    );

                    for (const terminal of sortedTerminals) {
                        const newPosition = {
                            x: screenWidth / 2,
                            y: currentY,
                            z: terminal.position.z,
                        }; // 中央に配置
                        updateTerminalPosition(terminal.id, newPosition);
                        addOutput(
                            USAGE_UPDATED_TERMINAL_POSITION(
                                terminal.id,
                                newPosition,
                            ),
                            LogLevel.INFO,
                        );
                        currentY += spacing;
                    }
                    addOutput(USAGE_TERM_FORMATTED, LogLevel.INFO);
                    break;
                }
                default:
                    addOutput(
                        USAGE_INVALID_TERM_SUBCOMMAND(subCommand),
                        LogLevel.ERROR,
                    );
            }
        },
        [addOutput, terminals, updateTerminalPosition, activateTerminal],
    );

    const modeExec = useCallback(
        (id: string | undefined) => {
            if (id) {
                const modeId = Number(id);
                if (Number.isNaN(modeId) || !(modeId in TerminalSectionId)) {
                    addOutput(USAGE_INVALID_MODE_ID(id), LogLevel.ERROR);
                    return;
                }
                // まれに異常：懺悔モードへ転落
                if (Math.random() < 1.005) {
                    setMode(TerminalSectionId.Zange);
                    addOutput(USAGE_DESCENT_INTO_ZANGE, LogLevel.ERROR);
                    return;
                }
                setMode(modeId); // モードを変更

                addOutput(
                    USAGE_MODE_CHANGED(TerminalSectionId[modeId]),
                    LogLevel.INFO,
                );
            } else {
                addOutput(USAGE_INVALID_OR_MISSING_ID_MODE, LogLevel.ERROR);
            }
        },
        [addOutput, setMode],
    );

    const changeLangExec = useCallback(
        (lang: "ja" | "en") => {
            updateLanguage(lang);
            addOutput(USAGE_LANGUAGE_CHANGED(lang), LogLevel.INFO);
        },
        [addOutput, updateLanguage],
    );

    const restExec = useCallback(() => {
        const terminalId = mode; // 現在のモードを使用
        const terminalName = TerminalSectionId[terminalId];
        const terminal = terminals.find((t) => t.id === terminalId);
        if (terminal && terminal.status === TerminalStatus.HEALTHY) {
            updateTerminalStatus(terminalId, TerminalStatus.ON_BREAK);
            addOutput(USAGE_REST_SUCCESS(terminalName), LogLevel.INFO);
        } else {
            addOutput(USAGE_REST_FAILURE(terminalName), LogLevel.ERROR);
        }
    }, [addOutput, terminals, updateTerminalStatus, mode]);

    const workExec = useCallback(() => {
        const terminalId = mode; // 現在のモードを使用
        const terminalName = TerminalSectionId[terminalId];
        const terminal = terminals.find((t) => t.id === terminalId);
        if (terminal && terminal.status === TerminalStatus.ON_BREAK) {
            updateTerminalStatus(terminalId, TerminalStatus.HEALTHY);
            addOutput(USAGE_WORK_SUCCESS(terminalName), LogLevel.INFO);
        } else {
            addOutput(USAGE_WORK_FAILURE(terminalName), LogLevel.ERROR);
        }
    }, [addOutput, terminals, updateTerminalStatus, mode]);

    const disposeExec = useCallback(() => {
        const terminalId = mode; // 現在のモードを使用
        const terminalName = TerminalSectionId[terminalId];
        const terminal = terminals.find((t) => t.id === terminalId);
        const wasteTerminal = terminals.find(
            (t) => t.id === TerminalSectionId.Waste,
        );

        if (terminal && wasteTerminal) {
            const wasteAmount = terminal.barometer.wasteOverflow;

            // Waste ﾀｰﾐﾅﾙに加算し、現在のﾀｰﾐﾅﾙの wasteOverflow をリセット
            disposeWaste(terminalId, wasteAmount);
            accumulateWaste(TerminalSectionId.Waste, wasteAmount);

            addOutput(
                USAGE_DISPOSE_SUCCESS(wasteAmount, terminalName),
                LogLevel.INFO,
            );
            addNews(
                TerminalSectionId.Waste,
                USAGE_DISPOSE_SUCCESS(wasteAmount, terminalName),
            );
        } else {
            addOutput(USAGE_DISPOSE_FAILURE, LogLevel.ERROR);
        }
    }, [addOutput, terminals, disposeWaste, accumulateWaste, mode, addNews]);

    const trapExec = useCallback(() => {
        const terminalId = mode;

        try {
            context.updateCash("expense", 30); // キャッシュを消費
            updateTrap(terminalId, 1); // トラップを1つ追加
            addOutput(USAGE_TRAP_SET, LogLevel.INFO);
        } catch (error) {
            if (error instanceof Error) {
                addOutput(USAGE_TRAP_FAILURE(error.message), LogLevel.ERROR);
            } else {
                addOutput(
                    USAGE_TRAP_FAILURE("An unknown error occurred"),
                    LogLevel.ERROR,
                );
            }
        }
    }, [updateTrap, mode, addOutput, context]);

    return useCallback(
        (command: GeneralCommands, ...args: string[]) => {
            switch (command) {
                case GeneralCommands.TERM:
                    termExec(args[0], args[1]);
                    break;
                case GeneralCommands.MODE:
                    modeExec(args[0]);
                    break;
                case GeneralCommands.HELP:
                    helpExec();
                    break;
                case GeneralCommands.EN:
                    changeLangExec("en");
                    break;
                case GeneralCommands.JA:
                    changeLangExec("ja");
                    break;
                case GeneralCommands.REST:
                    restExec();
                    break;
                case GeneralCommands.WORK:
                    workExec();
                    break;
                case GeneralCommands.DISPOSE:
                    disposeExec();
                    break;
                case GeneralCommands.TRAP:
                    trapExec();
                    break;
                default:
                    addOutput(USAGE_UNKNOWN_COMMAND, LogLevel.ERROR);
            }
        },
        [
            termExec,
            modeExec,
            helpExec,
            changeLangExec,
            restExec,
            workExec,
            disposeExec,
            trapExec,
            addOutput,
        ],
    );
};
