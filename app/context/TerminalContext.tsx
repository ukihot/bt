"use client";

import type React from "react";
import { createContext, useCallback, useEffect, useState } from "react";
import { v4 as uuidv4 } from "uuid";
import {
    type Bread,
    BreadCookingStatus,
    BreadType,
    GameOverReason,
    type Ingredient,
    type OminousType,
    type Terminal,
    type TerminalPosition,
    TerminalSectionId,
    TerminalStatus,
    type TransactionType,
    type UsageCode,
} from "../bt.types";
import { DEFAULT_INGREDIENT_COST } from "../utils/breadRecipe";
import { mapAndUpdate } from "../utils/news";
import { USAGE_TRAP_SUCCESS } from "../utils/usage/usageGeneral";

export interface TerminalContextType {
    terminals: Terminal[];
    cash: number;
    repository: Ingredient;
    ingredientCost: Ingredient;
    language: "ja" | "en";
    bread: Bread[];
    nigiwai: number;
    productionSpeed: number;
    productionPlan: Record<BreadType, number>;
    ominous: OminousType | null;
    setLanguage: (lang: "ja" | "en") => void;
    updateLanguage: (lang: "ja" | "en") => void;
    updateCash: (type: TransactionType, amount: number) => void;
    updateTerminalPosition: (
        id: TerminalSectionId,
        position: TerminalPosition,
    ) => void;
    activateTerminal: (id: TerminalSectionId) => void;
    deactivateTerminal: (id: TerminalSectionId) => void;
    addNews: (
        id: TerminalSectionId,
        usage: UsageCode,
        isOverbearing?: boolean,
    ) => void;
    updateProgress: (id: TerminalSectionId, progress: number) => void;
    updateRepository: (
        isRestock: boolean,
        stock: Partial<Ingredient>,
    ) => boolean;
    updateIngredientCost: (cost: Partial<Ingredient>) => void;
    updateTerminalStatus: (
        id: TerminalSectionId,
        status: TerminalStatus,
    ) => void;
    resetGame: () => void;
    updateTroubleProbability: (
        id: TerminalSectionId,
        probability: number,
    ) => void;
    updateBread: (updatedBread: Bread[]) => void;
    overviewBreadStatus: () => string;
    countHealthyTerminals: () => number;
    updateNigiwai: (value: number) => void;
    setProductionSpeed: (speed: number) => void;
    updateProductionPlan: (breadType: BreadType, quantity: number) => void;
    increaseTemperature: (id: TerminalSectionId, amount: number) => void;
    decreaseTemperature: (id: TerminalSectionId, amount: number) => void;
    increaseRodents: (id: TerminalSectionId, amount: number) => void;
    decreaseRodents: (id: TerminalSectionId, amount: number) => void;
    maintainEquipment: (id: TerminalSectionId, amount: number) => void;
    wearEquipment: (id: TerminalSectionId, amount: number) => void;
    disposeWaste: (id: TerminalSectionId, amount: number) => void;
    accumulateWaste: (id: TerminalSectionId, amount: number) => void;
    addIntruder: (id: TerminalSectionId, amount: number) => void;
    removeIntruder: (id: TerminalSectionId, amount: number) => void;
    attemptTrapCatch: (id: TerminalSectionId) => void;
    updateTrap: (id: TerminalSectionId, quantity: number) => void;
    isGameOver: GameOverReason | null;
    updateIsGameOver: () => void;
    hasTerminalWithStatus: (status: TerminalStatus) => boolean;
    updateOmninous: (value: OminousType | null) => void;
}

export const DEFAULT_CASH = 10_000.0;

const DEFAULT_INGREDIENT: Ingredient = {
    flour: 2_500.0,
    yeast: 200.0,
    salt: 50.0,
    butter: 250.0,
    sugar: 100.0,
    milk: 300.0,
    redBeanPaste: 300.0,
    malt: 30.0,
};

const DEFAULT_PRODUCTION_SPEED = 3000; // デフォルトの生産スピード

const DEFAULT_PRODUCTION_PLAN: Record<BreadType, number> = {
    [BreadType.Anpan]: 0,
    [BreadType.Begguette]: 0,
    [BreadType.Croissant]: 1,
    [BreadType.Naan]: 0,
};

export const TerminalContext = createContext<TerminalContextType | undefined>(
    undefined,
);

export const TerminalProvider = ({
    children,
}: { children: React.ReactNode }) => {
    const [terminals, setTerminals] = useState<Terminal[]>([]);
    const [cash, setCash] = useState<number>(DEFAULT_CASH);
    const [repository, setRepository] =
        useState<Ingredient>(DEFAULT_INGREDIENT);
    const [ingredientCost, setIngredientCost] = useState<Ingredient>(
        DEFAULT_INGREDIENT_COST,
    );
    const [language, setLanguage] = useState<"ja" | "en">("ja");
    const [bread, setBread] = useState<Bread[]>([]); // 作業途中のパンの状態
    const [nigiwai, setNigiwai] = useState<number>(1.0);
    const [productionSpeed, setProductionSpeed] = useState<number>(
        DEFAULT_PRODUCTION_SPEED,
    );
    const [productionPlan, setProductionPlan] = useState<
        Record<BreadType, number>
    >(DEFAULT_PRODUCTION_PLAN);
    const [isGameOver, setIsGameOver] = useState<GameOverReason | null>(null);
    const [ominous, setOminous] = useState<OminousType | null>(null);
    const updateCash = useCallback(
        (type: TransactionType, amount: number) => {
            if (type === "expense" && cash < amount) {
                throw new Error("Not enough cash");
            }
            setCash((prevCash) =>
                type === "income" ? prevCash + amount : prevCash - amount,
            );
        },
        [cash],
    );

    const updateTerminalPosition = useCallback(
        (id: TerminalSectionId, position: TerminalPosition) => {
            setTerminals((prev) => {
                const maxZIndex = Math.max(0, ...prev.map((t) => t.position.z));
                return prev.map((terminal) =>
                    terminal.id === id
                        ? {
                              ...terminal,
                              position: { ...position, z: maxZIndex + 1 },
                          }
                        : terminal,
                );
            });
        },
        [],
    );

    const addTerminal = useCallback((id: TerminalSectionId) => {
        const getDescription = (id: TerminalSectionId): string => {
            switch (id) {
                case TerminalSectionId.Purchasing:
                    return "This is where you purchase ingredients!";
                case TerminalSectionId.Pantry:
                    return "This is where you store ingredients!";
                case TerminalSectionId.Mixing:
                    return "This is where you mix ingredients to make dough!";
                case TerminalSectionId.Cooling:
                    return "This is where dough is fermented and cooled!";
                case TerminalSectionId.Shaping:
                    return "This is where dough is shaped!";
                case TerminalSectionId.Baking:
                    return "This is where dough is baked into bread!";
                case TerminalSectionId.Packaging:
                    return "This is where bread is packaged!";
                case TerminalSectionId.SalesFront:
                    return "This is where bread is sold!";
                case TerminalSectionId.Waste:
                    return "This is where waste is managed!";
                case TerminalSectionId.Utilities:
                    return "This is where water, electricity, and gas are managed!";
                default:
                    return "This is an undefined area!";
            }
        };

        setTerminals((prev) => {
            if (prev.some((terminal) => terminal.id === id)) return prev;
            const maxZIndex = Math.max(0, ...prev.map((t) => t.position.z));
            return [
                ...prev,
                {
                    id,
                    position: {
                        x: 450 + id * 80,
                        y: 200 + id * 35,
                        z: maxZIndex + 1,
                    },
                    status: TerminalStatus.HEALTHY,
                    news: [
                        {
                            id: uuidv4(),
                            datetime: new Date(),
                            description: getDescription(id),
                        },
                    ],
                    visible: true,
                    progress: 0,
                    troubleProbability: 0.09,
                    barometer: {
                        rodentCount: 0,
                        roomTemperature: 25,
                        intruderCount: 0,
                        equipmentWear: 0,
                        wasteOverflow: 0,
                        trap: 0,
                    },
                },
            ];
        });
    }, []);

    const activateTerminal = useCallback((id: TerminalSectionId) => {
        setTerminals((prev) =>
            prev.map((terminal) =>
                terminal.id === id ? { ...terminal, visible: true } : terminal,
            ),
        );
    }, []);

    const deactivateTerminal = useCallback((id: TerminalSectionId) => {
        setTerminals((prev) =>
            prev.map((terminal) =>
                terminal.id === id ? { ...terminal, visible: false } : terminal,
            ),
        );
    }, []);

    const addNews = useCallback(
        (id: TerminalSectionId, usage: UsageCode, isOverbearing?: boolean) => {
            const localizedDescription =
                language === "en" && usage.en ? usage.en : usage.ja;
            setTerminals((prev) =>
                mapAndUpdate(prev, id, localizedDescription, isOverbearing),
            );
        },
        [language],
    );

    const updateProgress = useCallback(
        (id: TerminalSectionId, progress: number) => {
            setTerminals((prev) =>
                prev.map((terminal) =>
                    terminal.id === id
                        ? {
                              ...terminal,
                              progress: progress >= 100 ? 0 : progress,
                          }
                        : terminal,
                ),
            );
        },
        [],
    );

    const updateRepository = useCallback(
        (isRestock: boolean, stock: Partial<Ingredient>): boolean => {
            let hasInsufficientIngredients = false;

            // チェックフェーズ: 必要な量が存在するか確認
            if (!isRestock) {
                for (const [key, value] of Object.entries(stock)) {
                    const ingredientKey = key as keyof Ingredient;
                    if ((repository[ingredientKey] ?? 0) < (value ?? 0)) {
                        hasInsufficientIngredients = true;
                        break;
                    }
                }
                if (hasInsufficientIngredients) return false;
            }

            // 更新フェーズ: 在庫を更新
            setRepository((prev) => {
                const updatedRepository = { ...prev };

                for (const [key, value] of Object.entries(stock)) {
                    const ingredientKey = key as keyof Ingredient;
                    const newValue =
                        prev[ingredientKey] +
                        (isRestock ? (value ?? 0) : -(value ?? 0));

                    updatedRepository[ingredientKey] = Math.max(newValue, 0);
                }

                return updatedRepository;
            });

            return true;
        },
        [repository],
    );

    const updateIngredientCost = useCallback((cost: Partial<Ingredient>) => {
        setIngredientCost((prev) => ({ ...prev, ...cost }));
    }, []);

    const updateTerminalStatus = useCallback(
        (id: TerminalSectionId, status: TerminalStatus) => {
            setTerminals((prev) =>
                prev.map((terminal) =>
                    terminal.id === id ? { ...terminal, status } : terminal,
                ),
            );
        },
        [],
    );

    const updateLanguage = useCallback((lang: "ja" | "en") => {
        setLanguage(lang);
    }, []);

    const updateTroubleProbability = useCallback(
        (id: TerminalSectionId, probability: number) => {
            setTerminals((prev) =>
                prev.map((terminal) =>
                    terminal.id === id
                        ? { ...terminal, troubleProbability: probability }
                        : terminal,
                ),
            );
        },
        [],
    );

    const updateBread = useCallback((updatedBread: Bread[]) => {
        setBread(updatedBread);
    }, []);

    const updateNigiwai = useCallback((value: number) => {
        if (value === 0) {
            throw new Error("Nigiwai value must not be 0");
        }
        setNigiwai(value);
    }, []);

    const increaseTemperature = useCallback(
        (id: TerminalSectionId, amount: number) => {
            setTerminals((prev) =>
                prev.map((terminal) =>
                    terminal.id === id
                        ? {
                              ...terminal,
                              barometer: {
                                  ...terminal.barometer,
                                  roomTemperature:
                                      terminal.barometer.roomTemperature +
                                      amount,
                              },
                          }
                        : terminal,
                ),
            );
        },
        [],
    );

    const decreaseTemperature = useCallback(
        (id: TerminalSectionId, amount: number) => {
            setTerminals((prev) =>
                prev.map((terminal) =>
                    terminal.id === id
                        ? {
                              ...terminal,
                              barometer: {
                                  ...terminal.barometer,
                                  roomTemperature: Math.max(
                                      terminal.barometer.roomTemperature -
                                          amount,
                                      0,
                                  ),
                              },
                          }
                        : terminal,
                ),
            );
        },
        [],
    );

    const increaseRodents = useCallback(
        (id: TerminalSectionId, amount: number) => {
            setTerminals((prev) =>
                prev.map((terminal) =>
                    terminal.id === id
                        ? {
                              ...terminal,
                              barometer: {
                                  ...terminal.barometer,
                                  rodentCount:
                                      terminal.barometer.rodentCount + amount,
                              },
                          }
                        : terminal,
                ),
            );
        },
        [],
    );

    const decreaseRodents = useCallback(
        (id: TerminalSectionId, amount: number) => {
            setTerminals((prev) =>
                prev.map((terminal) =>
                    terminal.id === id
                        ? {
                              ...terminal,
                              barometer: {
                                  ...terminal.barometer,
                                  rodentCount: Math.max(
                                      terminal.barometer.rodentCount - amount,
                                      0,
                                  ),
                              },
                          }
                        : terminal,
                ),
            );
        },
        [],
    );

    const maintainEquipment = useCallback(
        (id: TerminalSectionId, amount: number) => {
            setTerminals((prev) =>
                prev.map((terminal) =>
                    terminal.id === id
                        ? {
                              ...terminal,
                              barometer: {
                                  ...terminal.barometer,
                                  equipmentWear: Math.max(
                                      terminal.barometer.equipmentWear - amount,
                                      0,
                                  ),
                              },
                          }
                        : terminal,
                ),
            );
        },
        [],
    );

    const wearEquipment = useCallback(
        (id: TerminalSectionId, amount: number) => {
            setTerminals((prev) =>
                prev.map((terminal) =>
                    terminal.id === id
                        ? {
                              ...terminal,
                              barometer: {
                                  ...terminal.barometer,
                                  equipmentWear:
                                      terminal.barometer.equipmentWear + amount,
                              },
                          }
                        : terminal,
                ),
            );
        },
        [],
    );

    const disposeWaste = useCallback(
        (id: TerminalSectionId, amount: number) => {
            setTerminals((prev) =>
                prev.map((terminal) =>
                    terminal.id === id
                        ? {
                              ...terminal,
                              barometer: {
                                  ...terminal.barometer,
                                  wasteOverflow: Math.max(
                                      terminal.barometer.wasteOverflow - amount,
                                      0,
                                  ),
                              },
                          }
                        : terminal,
                ),
            );
        },
        [],
    );

    const accumulateWaste = useCallback(
        (id: TerminalSectionId, amount: number) => {
            setTerminals((prev) =>
                prev.map((terminal) =>
                    terminal.id === id
                        ? {
                              ...terminal,
                              barometer: {
                                  ...terminal.barometer,
                                  wasteOverflow:
                                      terminal.barometer.wasteOverflow + amount,
                              },
                          }
                        : terminal,
                ),
            );
        },
        [],
    );

    const addIntruder = useCallback((id: TerminalSectionId, amount: number) => {
        setTerminals((prev) =>
            prev.map((terminal) =>
                terminal.id === id
                    ? {
                          ...terminal,
                          barometer: {
                              ...terminal.barometer,
                              intruderCount:
                                  terminal.barometer.intruderCount + amount,
                          },
                      }
                    : terminal,
            ),
        );
    }, []);

    const removeIntruder = useCallback(
        (id: TerminalSectionId, amount: number) => {
            setTerminals((prev) =>
                prev.map((terminal) =>
                    terminal.id === id
                        ? {
                              ...terminal,
                              barometer: {
                                  ...terminal.barometer,
                                  intruderCount: Math.max(
                                      terminal.barometer.intruderCount - amount,
                                      0,
                                  ),
                              },
                          }
                        : terminal,
                ),
            );
        },
        [],
    );

    const updateProductionPlan = useCallback(
        (breadType: BreadType, quantity: number) => {
            if (quantity < 0) {
                throw new Error("Quantity cannot be negative");
            }
            setProductionPlan((prev) => ({
                ...prev,
                [breadType]: quantity,
            }));
        },
        [],
    );

    const updateOmninous = useCallback((value: OminousType | null) => {
        setOminous(value);
    }, []);

    const overviewBreadStatus = useCallback(() => {
        const statusCount: Record<BreadCookingStatus, number> = {
            [BreadCookingStatus.Raw]: 0,
            [BreadCookingStatus.FirstProofed]: 0,
            [BreadCookingStatus.Shaped]: 0,
            [BreadCookingStatus.SecondProofed]: 0,
            [BreadCookingStatus.Baked]: 0,
            [BreadCookingStatus.Packaged]: 0,
            [BreadCookingStatus.Shelved]: 0,
            [BreadCookingStatus.Sold]: 0,
            [BreadCookingStatus.Discarded]: 0,
        };

        for (const b of bread) {
            statusCount[b.cookStatus]++;
        }

        return Object.entries(statusCount)
            .map(
                ([status, count]) =>
                    `${BreadCookingStatus[status as unknown as BreadCookingStatus]}: ${count}\n`,
            )
            .join("");
    }, [bread]);

    const countHealthyTerminals = useCallback(() => {
        return terminals.filter(
            (terminal) => terminal.status === TerminalStatus.HEALTHY,
        ).length;
    }, [terminals]);

    const updateIsGameOver = useCallback(() => {
        const unhealthyCount = terminals.filter(
            ({ status }) =>
                status !== TerminalStatus.HEALTHY &&
                status !== TerminalStatus.ON_BREAK,
        ).length;

        if (unhealthyCount > 4) {
            setIsGameOver(GameOverReason.TOO_MANY_UNHEALTHY_TERMINALS);
            return;
        }

        for (const terminal of terminals) {
            const { barometer, id } = terminal;
            if (
                barometer.roomTemperature > 38 ||
                barometer.roomTemperature < 10
            ) {
                setIsGameOver(GameOverReason.TEMPERATURE_THRESHOLD_EXCEEDED);
                return;
            }
            if (
                barometer.rodentCount > 5 &&
                Math.random() < 1 - Math.exp(-barometer.rodentCount * 0.002) &&
                barometer.trap === 0
            ) {
                setIsGameOver(GameOverReason.SHUTDOWN_BY_HEALTH_OFFICIALS);
                return;
            }

            if (barometer.equipmentWear >= 100) {
                setIsGameOver(GameOverReason.EQUIPMENT_WEAR_MAXED);
                return;
            }
            if (
                barometer.wasteOverflow >= 100 &&
                id !== TerminalSectionId.Waste
            ) {
                setIsGameOver(GameOverReason.WASTE_OVERFLOW);
                return;
            }
            if (barometer.intruderCount > 2) {
                setIsGameOver(GameOverReason.INTRUDER_COUNT_EXCEEDED);
                return;
            }
        }
    }, [terminals]);

    const updateTrap = useCallback(
        (id: TerminalSectionId, quantity: number) => {
            setTerminals((prev) =>
                prev.map((terminal) =>
                    terminal.id === id
                        ? {
                              ...terminal,
                              barometer: {
                                  ...terminal.barometer,
                                  trap: Math.max(
                                      terminal.barometer.trap + quantity,
                                      0,
                                  ),
                              },
                          }
                        : terminal,
                ),
            );
        },
        [],
    );

    const attemptTrapCatch = useCallback(
        (id: TerminalSectionId) => {
            setTerminals((prev) =>
                prev.map((terminal) => {
                    if (terminal.id !== id) return terminal;

                    const chance =
                        Math.abs(nigiwai) *
                        (terminal.barometer.rodentCount +
                            terminal.barometer.trap) *
                        0.02;
                    if (
                        terminal.barometer.rodentCount > 0 &&
                        terminal.barometer.trap > 0 &&
                        Math.random() < chance
                    ) {
                        updateTrap(terminal.id, -1); // 罠を減らす
                        decreaseRodents(terminal.id, 1); // ﾈｽﾞﾐを減らす
                        addNews(terminal.id, USAGE_TRAP_SUCCESS);
                    }
                    return terminal;
                }),
            );
        },
        [nigiwai, updateTrap, decreaseRodents, addNews],
    );

    const hasTerminalWithStatus = useCallback(
        (status: TerminalStatus) => {
            return terminals.some((terminal) => terminal.status === status);
        },
        [terminals],
    );

    const resetGame = useCallback(() => {
        setTerminals([]);
        setCash(DEFAULT_CASH);
        setRepository(DEFAULT_INGREDIENT);
        setIngredientCost(DEFAULT_INGREDIENT_COST);

        const terminalIds = Object.values(TerminalSectionId).filter(
            (id) => typeof id === "number" && id !== TerminalSectionId.Zange,
        ) as TerminalSectionId[];

        for (const id of terminalIds) {
            addTerminal(id);
        }
    }, [addTerminal]);

    useEffect(() => {
        resetGame();
    }, [resetGame]);

    return (
        <TerminalContext.Provider
            value={{
                terminals,
                cash,
                repository,
                ingredientCost,
                language,
                bread,
                nigiwai,
                productionSpeed,
                productionPlan,
                ominous,
                setLanguage,
                updateLanguage,
                updateCash,
                updateTerminalPosition,
                activateTerminal,
                deactivateTerminal,
                addNews,
                updateProgress,
                updateRepository,
                updateIngredientCost,
                updateTerminalStatus,
                resetGame,
                updateTroubleProbability,
                updateBread,
                overviewBreadStatus,
                countHealthyTerminals,
                updateNigiwai,
                setProductionSpeed,
                updateProductionPlan,
                increaseTemperature,
                decreaseTemperature,
                increaseRodents,
                decreaseRodents,
                maintainEquipment,
                wearEquipment,
                disposeWaste,
                accumulateWaste,
                addIntruder,
                removeIntruder,
                isGameOver,
                updateIsGameOver,
                updateTrap,
                attemptTrapCatch,
                hasTerminalWithStatus,
                updateOmninous,
            }}
        >
            {children}
        </TerminalContext.Provider>
    );
};
export { TerminalSectionId };
