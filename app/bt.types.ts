export type UsageCode = {
    ja: string;
    en?: string;
};

export enum LogLevel {
    INFO = "info",
    WARN = "warn",
    ERROR = "error",
}

export enum TerminalSectionId {
    Purchasing = 0,
    Pantry = 1,
    Mixing = 2,
    Cooling = 3,
    Shaping = 4,
    Baking = 5,
    Packaging = 6,
    SalesFront = 7,
    Waste = 8,
    Utilities = 9,
    Zange = 999,
}

export enum BreadType {
    Anpan = "Anpan",
    Begguette = "Begguette",
    Croissant = "Croissant",
    Naan = "Naan",
}

export enum BreadCookingStatus {
    Raw = 0, // 混錬済
    FirstProofed = 1, // 一次発酵済
    Shaped = 2, // 成型済（二次発酵前）
    SecondProofed = 3, // 二次発酵済
    Baked = 4, // 焼成済
    Packaged = 5, // 包装済
    Shelved = 6, // 販売中
    Sold = 7, // 販売済
    Discarded = 8, // 廃棄済
}

export enum GameOverReason {
    TOO_MANY_UNHEALTHY_TERMINALS = "Too many unhealthy terminals.",
    TEMPERATURE_THRESHOLD_EXCEEDED = "Temperature threshold exceeded.",
    SHUTDOWN_BY_HEALTH_OFFICIALS = "Shut down by health officials.",
    EQUIPMENT_WEAR_MAXED = "Equipment wear maxed out.",
    WASTE_OVERFLOW = "Waste overflow.",
    INTRUDER_COUNT_EXCEEDED = "Intruder count exceeded.",
}

export interface TerminalPosition {
    x: number;
    y: number;
    z: number;
}

export interface News {
    id: string;
    datetime: Date;
    description: string;
    isOverbearing?: boolean;
}

export interface Ingredient {
    flour: number;
    yeast: number;
    salt: number;
    butter: number;
    sugar: number;
    milk: number;
    redBeanPaste: number;
    malt: number;
}

export interface Bread {
    id: string;
    kind: BreadType;
    cookStatus: BreadCookingStatus;
}

export type TransactionType = "income" | "expense";

export enum TerminalStatus {
    HEALTHY = "HEALTHY", // 健康な状態
    ON_BREAK = "ON_BREAK", // 休憩中
    POWER_FAILURE = "POWER_FAILURE", // 停電
    WATER_LEAK = "WATER_LEAK", // 水漏れ（配管関連）
    FIRE_HAZARD = "FIRE_HAZARD", // 火災
    GAS_LEAK = "GAS_LEAK", // ガス漏れ
    UNUSUAL_NOISE = "UNUSUAL_NOISE", // 異音
    CONTAMINATION_DETECTED = "CONTAMINATION_DETECTED", // 異物・汚染検知
    PEST_INFESTATION = "PEST_INFESTATION", // 害虫発生
    UNAUTHORIZED_ACCESS = "UNAUTHORIZED_ACCESS", // 不審者・侵入検知
    OVERHEATING = "OVERHEATING", // 温度異常
    WRONG_BREAD_PRODUCED = "WRONG_BREAD_PRODUCED", // 間違ったパンを作ってしまう
}

export interface TerminalStatusText {
    terminalStatus: TerminalStatus;
}

export interface Barometer {
    roomTemperature: number; // 室温
    rodentCount: number; // ﾈｽﾞﾐの数
    equipmentWear: number; // 設備の摩耗度
    wasteOverflow: number; // 廃棄物の溢れ
    intruderCount: number; // 乱入者の数
    trap: number; // 罠の数
}

export interface Terminal {
    id: TerminalSectionId;
    position: TerminalPosition;
    status: TerminalStatus;
    news: News[];
    visible: boolean;
    progress: number;
    troubleProbability: number;
    barometer: Barometer; // 各ｾｸｼｮﾝの重要な閾値
}

export enum OminousType {
    Sorryman = "Sorryman",
}
