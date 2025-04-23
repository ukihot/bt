"use client";

import { useCallback, useContext, useEffect } from "react";
import { TerminalContext, TerminalSectionId } from "../context/TerminalContext";
import { useAbnormalHandlers } from "../hooks/useAbnormalHandlers";
import { useNormalHandlers } from "../hooks/useNormalHandlers";
import { USAGE_HEAL, USAGE_RODENT_ALERT } from "../utils/usage/usageGeneral";
import TerminalWindowUnit from "./TerminalWindowUnit";
import { type Terminal, TerminalStatus } from "../bt.types";

export const TerminalWindows = () => {
    const context = useContext(TerminalContext);
    if (!context) {
        return null;
    }
    const {
        terminals,
        addNews,
        maintainEquipment,
        productionSpeed,
        wearEquipment,
        accumulateWaste,
        increaseRodents,
        attemptTrapCatch,
        isGameOver,
        updateIsGameOver,
        nigiwai,
    } = context;

    const abnormalHandlers = useAbnormalHandlers(context);
    const normalHandlers = useNormalHandlers(context);

    const handleTrouble = useCallback(
        (terminal: Terminal) => {
            switch (terminal.id) {
                case 0:
                    abnormalHandlers.handlePurchasingTrouble();
                    break;
                case 1:
                    abnormalHandlers.handlePantryTrouble();
                    break;
                case 2:
                    abnormalHandlers.handleMixingTrouble();
                    break;
                case 3:
                    abnormalHandlers.handleCoolingTrouble();
                    break;
                case 4:
                    abnormalHandlers.handleShapingTrouble();
                    break;
                case 5:
                    abnormalHandlers.handleBakingTrouble();
                    break;
                case 6:
                    abnormalHandlers.handlePackagingTrouble();
                    break;
                case 7:
                    abnormalHandlers.handleSalesFrontTrouble();
                    break;
                case 8:
                    abnormalHandlers.handleWasteTrouble();
                    break;
                case 9:
                    abnormalHandlers.handleUtilitiesTrouble();
                    break;
                default:
            }
        },
        [abnormalHandlers],
    );

    const handleNormalBatch = useCallback(
        (terminal: Terminal) => {
            switch (terminal.id) {
                case 0:
                    normalHandlers.handlePurchasingBatch();
                    break;
                case 1:
                    normalHandlers.handlePantryBatch();
                    break;
                case 2:
                    normalHandlers.handleMixingBatch();
                    break;
                case 3:
                    normalHandlers.handleCoolingBatch();
                    break;
                case 4:
                    normalHandlers.handleShapingBatch();
                    break;
                case 5:
                    normalHandlers.handleBakingBatch();
                    break;
                case 6:
                    normalHandlers.handlePackagingBatch();
                    break;
                case 7:
                    normalHandlers.handleSalesFrontBatch();
                    break;
                case 8:
                    normalHandlers.handleWasteBatch();
                    break;
                case 9:
                    normalHandlers.handleUtilitiesBatch();
                    break;
                default:
            }
        },
        [normalHandlers],
    );

    const applyDamage = useCallback(
        (terminalId: TerminalSectionId) => {
            const damage = Math.random() * 3; // 0から3の乱数
            wearEquipment(terminalId, damage);
        },
        [wearEquipment],
    );

    const applyWaste = useCallback(
        (terminalId: TerminalSectionId) => {
            if (terminalId !== TerminalSectionId.Waste) {
                const wasteAmount = Math.random();
                accumulateWaste(terminalId, wasteAmount);
            }
        },
        [accumulateWaste],
    );

    const applyRodents = useCallback(
        (terminalId: TerminalSectionId) => {
            increaseRodents(terminalId, 1);
        },
        [increaseRodents],
    );

    // パン工房稼働
    useEffect(() => {
        const intervals = terminals.map((terminal) =>
            setInterval(() => {
                if (isGameOver) return;

                attemptTrapCatch(terminal.id);

                if (terminal.barometer.rodentCount > 0) {
                    addNews(terminal.id, USAGE_RODENT_ALERT);
                }

                switch (terminal.status) {
                    case TerminalStatus.ON_BREAK:
                        maintainEquipment(terminal.id, Math.abs(nigiwai));
                        addNews(terminal.id, USAGE_HEAL);
                        break;
                    case TerminalStatus.HEALTHY:
                        handleNormalBatch(terminal);
                        if (Math.random() < terminal.troubleProbability) {
                            handleTrouble(terminal);
                        }
                        if (Math.random() < terminal.troubleProbability) {
                            applyRodents(terminal.id);
                        }
                        if (Math.random() < terminal.troubleProbability) {
                            applyWaste(terminal.id);
                        }
                        if (Math.random() < terminal.troubleProbability) {
                            applyDamage(terminal.id);
                        }
                        break;
                    default:
                        applyDamage(terminal.id);
                        handleTrouble(terminal);
                        break;
                }

                updateIsGameOver();
            }, productionSpeed),
        );

        return () => intervals.forEach(clearInterval);
    }, [
        terminals,
        handleTrouble,
        nigiwai,
        handleNormalBatch,
        applyDamage,
        applyWaste,
        isGameOver,
        maintainEquipment,
        addNews,
        attemptTrapCatch,
        applyRodents,
        productionSpeed,
        updateIsGameOver,
    ]);

    return (
        <>
            {terminals.map((terminal) => (
                <TerminalWindowUnit
                    key={terminal.id}
                    {...terminal}
                    visible={false}
                    progress={0}
                />
            ))}
        </>
    );
};
