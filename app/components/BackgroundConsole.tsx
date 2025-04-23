"use client";

import { useCallback, useContext, useEffect, useRef, useState } from "react";
import {
    LogLevel,
    TerminalSectionId,
    TerminalStatus,
    type UsageCode,
} from "../bt.types";
import { TerminalContext } from "../context/TerminalContext";
import { useBakingCommand } from "../hooks/useBakingCommand";
import { useCoolingCommand } from "../hooks/useCoolingCommand";
import { useGeneralCommand } from "../hooks/useGeneralCommand";
import { useMixingCommand } from "../hooks/useMixingCommand";
import { usePackagingCommand } from "../hooks/usePackagingCommand";
import { usePantryCommand } from "../hooks/usePantryCommand";
import { usePurchasingCommand } from "../hooks/usePurchasingCommand";
import { useSalesFrontCommand } from "../hooks/useSalesFrontCommand";
import { useShapingCommand } from "../hooks/useShapingCommand";
import { useUtilitiesCommand } from "../hooks/useUtilitiesCommand";
import { useWasteCommand } from "../hooks/useWasteCommand";
import { useZangeCommand } from "../hooks/useZangeCommand";
import {
    BakingCommands,
    CoolingCommands,
    GeneralCommands,
    MixingCommands,
    PackagingCommands,
    PantryCommands,
    PurchasingCommands,
    SalesFrontCommands,
    ShapingCommands,
    UtilitiesCommands,
    WasteCommands,
    ZangeCommands,
} from "../utils/Command";
import { greeting } from "../utils/greeting";
import {
    USAGE_COMMAND_NOT_ALLOWED,
    USAGE_COMMAND_NOT_ALLOWED_ON_BREAK,
    USAGE_EMPTY,
    USAGE_OPERATION_CEASED,
    USAGE_UNKNOWN_COMMAND,
} from "../utils/usage/usageGeneral";
import { CommandInput } from "./CommandInput";
import { LogOutput, type LogProps } from "./LogOutput";

export const BackgroundConsole = () => {
    const inputRef = useRef<HTMLInputElement>(null);
    const [input, setInput] = useState(""); // 現在の入力値
    const [output, setOutput] = useState<LogProps[]>(greeting); // 出力ログ
    const [buffer, setBuffer] = useState<string[]>([]); // ｺﾏﾝﾄﾞ履歴
    const [mode, setMode] = useState<TerminalSectionId>(
        TerminalSectionId.Purchasing,
    );
    const [isInputEnabled, setIsInputEnabled] = useState(true);
    const context = useContext(TerminalContext);
    if (!context) {
        throw new Error("TerminalContext is not available");
    }
    const { language, isGameOver } = context;
    const getLocalizedMessage = useCallback(
        (usage: UsageCode) =>
            language === "en" && usage.en ? usage.en : usage.ja,
        [language],
    );

    const addOutput = useCallback(
        (usage: UsageCode, level: LogLevel = LogLevel.INFO) =>
            setOutput((prev) => [
                ...prev,
                { message: getLocalizedMessage(usage), level },
            ]),
        [getLocalizedMessage],
    );

    const replaceOutput = useCallback(
        (usage: UsageCode, level: LogLevel = LogLevel.INFO) =>
            setOutput((prevOutput) =>
                prevOutput.map((log, i) =>
                    i === prevOutput.length - 1
                        ? { message: `| ${getLocalizedMessage(usage)}`, level }
                        : log,
                ),
            ),
        [getLocalizedMessage],
    );

    // 各ｾｸｼｮﾝのｺﾏﾝﾄﾞ実行関数を取得
    const generalCommandExecutor = useGeneralCommand(addOutput, mode, setMode);
    const purchasingCommandExecutor = usePurchasingCommand(
        addOutput,
        replaceOutput,
        setIsInputEnabled,
    );
    const pantryCommandExecutor = usePantryCommand(addOutput);
    const utilitiesCommandExecutor = useUtilitiesCommand(addOutput);
    const mixingCommandExecutor = useMixingCommand(
        addOutput,
        replaceOutput,
        setIsInputEnabled,
    );
    const coolingCommandExecutor = useCoolingCommand(
        addOutput,
        replaceOutput,
        setIsInputEnabled,
    );
    const shapingCommandExecutor = useShapingCommand(
        addOutput,
        replaceOutput,
        setIsInputEnabled,
    );
    const bakingCommandExecutor = useBakingCommand(
        addOutput,
        replaceOutput,
        setIsInputEnabled,
    );
    const packagingCommandExecutor = usePackagingCommand(addOutput);
    const salesFrontCommandExecutor = useSalesFrontCommand(addOutput);
    const wasteCommandExecutor = useWasteCommand(addOutput);
    const zangeCommandExecutor = useZangeCommand(addOutput, setMode);

    // ｺﾏﾝﾄﾞを実行
    const handleExecuteCommand = (command: string) => {
        if (command.trim() === "") {
            addOutput(USAGE_EMPTY); // 空行を追加
            return;
        }
        const [cmd, ...args] = command.split(/\s+/);

        // 現在のﾀｰﾐﾅﾙのステータスを取得
        const currentTerminal = context.terminals.find((t) => t.id === mode);
        if (currentTerminal?.status === TerminalStatus.ON_BREAK) {
            // ON_BREAKの場合はgeneralCommandExecutor以外を拒否
            if (
                !(Object.values(GeneralCommands) as string[]).includes(
                    cmd as keyof typeof GeneralCommands,
                )
            ) {
                addOutput(
                    USAGE_COMMAND_NOT_ALLOWED_ON_BREAK(cmd),
                    LogLevel.WARN,
                );
                return;
            }
        }

        // 各ｾｸｼｮﾝのｺﾏﾝﾄﾞと対応するモードをマッピング
        const commandMap = [
            {
                commands: GeneralCommands,
                modes: Object.values(TerminalSectionId).filter(
                    (id): id is TerminalSectionId =>
                        id !== TerminalSectionId.Zange,
                ),
                executor: generalCommandExecutor,
            },
            {
                commands: PurchasingCommands,
                modes: [TerminalSectionId.Purchasing] as TerminalSectionId[],
                executor: purchasingCommandExecutor,
            },
            {
                commands: PantryCommands,
                modes: [TerminalSectionId.Pantry] as TerminalSectionId[],
                executor: pantryCommandExecutor,
            },
            {
                commands: MixingCommands,
                modes: [TerminalSectionId.Mixing] as TerminalSectionId[],
                executor: mixingCommandExecutor,
            },
            {
                commands: CoolingCommands,
                modes: [TerminalSectionId.Cooling] as TerminalSectionId[],
                executor: coolingCommandExecutor,
            },
            {
                commands: ShapingCommands,
                modes: [TerminalSectionId.Shaping] as TerminalSectionId[],
                executor: shapingCommandExecutor,
            },
            {
                commands: BakingCommands,
                modes: [TerminalSectionId.Baking] as TerminalSectionId[],
                executor: bakingCommandExecutor,
            },
            {
                commands: PackagingCommands,
                modes: [TerminalSectionId.Packaging] as TerminalSectionId[],
                executor: packagingCommandExecutor,
            },
            {
                commands: SalesFrontCommands,
                modes: [TerminalSectionId.SalesFront] as TerminalSectionId[],
                executor: salesFrontCommandExecutor,
            },
            {
                commands: WasteCommands,
                modes: [TerminalSectionId.Waste] as TerminalSectionId[],
                executor: wasteCommandExecutor,
            },
            {
                commands: UtilitiesCommands,
                modes: [TerminalSectionId.Utilities] as TerminalSectionId[],
                executor: utilitiesCommandExecutor,
            },
            {
                commands: ZangeCommands,
                modes: [TerminalSectionId.Zange] as TerminalSectionId[],
                executor: zangeCommandExecutor,
            },
        ] as const;

        // ｺﾏﾝﾄﾞを検索して実行
        const matched = commandMap.find(({ commands }) =>
            Object.values(commands).includes(cmd as keyof typeof commands),
        );

        if (matched) {
            const { commands, modes, executor } = matched;

            // 現在のモードが許容されているか確認
            if (modes.length > 0 && !modes.includes(mode)) {
                // 現在のモードに基づいて適切なｺﾏﾝﾄﾞセットを選択
                const alternativeMatch = commandMap.find(
                    ({ commands: altCommands, modes: altModes }) =>
                        Object.values(altCommands).includes(
                            cmd as keyof typeof altCommands,
                        ) && altModes.includes(mode),
                );

                if (alternativeMatch) {
                    alternativeMatch.executor?.(
                        cmd as keyof typeof alternativeMatch.commands,
                        ...args,
                    );
                    return;
                }

                addOutput(USAGE_COMMAND_NOT_ALLOWED(cmd), LogLevel.WARN);
                return;
            }

            // 実行関数がある場合は実行
            if (executor) {
                executor(cmd as keyof typeof commands, ...args);
            }
        } else {
            addOutput(USAGE_UNKNOWN_COMMAND, LogLevel.ERROR);
        }
    };

    // ｺﾏﾝﾄﾞ履歴を上下に移動する
    const history = (direction: "up" | "down") =>
        setInput((prevInput) => {
            const currentIndex = buffer.indexOf(prevInput);
            const newIndex =
                direction === "up"
                    ? currentIndex === -1
                        ? buffer.length - 1
                        : Math.max(currentIndex - 1, 0)
                    : currentIndex === -1 || currentIndex === buffer.length - 1
                      ? -1
                      : currentIndex + 1;

            const newInput = newIndex === -1 ? "" : buffer[newIndex] || "";
            setTimeout(() =>
                inputRef.current?.setSelectionRange(
                    newInput.length,
                    newInput.length,
                ),
            );
            return newInput;
        });

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (
            !/^[a-z0-9\s]*$/.test(e.key) &&
            !["Enter", "ArrowUp", "ArrowDown", "Backspace"].includes(e.key)
        ) {
            e.preventDefault();
            return;
        }

        const actions: Record<string, () => void> = {
            Enter: () => {
                setOutput((prev) => [
                    ...prev,
                    {
                        message: `${TerminalSectionId[mode]} $ ${input}`,
                        level: LogLevel.INFO,
                    },
                ]);
                handleExecuteCommand(input);
                setBuffer((prev) => [...prev, input]);
                setInput("");
            },
            ArrowUp: () => history("up"),
            ArrowDown: () => history("down"),
        };

        actions[e.key]?.();
    };

    useEffect(() => {
        if (isGameOver) {
            addOutput(USAGE_OPERATION_CEASED(isGameOver), LogLevel.ERROR);
        }
    }, [isGameOver, addOutput]);

    return (
        <div className="crt-grid crt-effect crt-flicker crt-noise crt-glass flex max-h-[71vh] w-full flex-col rounded-2xl border-2 border-green-300 p-2 text-base text-white backdrop-blur-lg">
            {/* 出力ログを表示 */}
            <LogOutput output={output} />
            {/* ｺﾏﾝﾄﾞ入力を表示 */}
            <CommandInput
                handleKeyDown={handleKeyDown}
                input={input}
                inputRef={inputRef}
                mode={mode}
                setInput={setInput}
                isInputEnabled={isInputEnabled}
            />
        </div>
    );
};

export default BackgroundConsole;
