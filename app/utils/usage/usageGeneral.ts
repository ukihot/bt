import type { UsageCode } from "../../bt.types";

export const USAGE_TEXTS = {
    EMPTY: "",
    HELP_HEADER: {
        ja: "利用可能なｺﾏﾝﾄﾞ一覧:",
        en: "Here are the available commands:",
    },
    HELP_LS: {
        ja: "ls - すべてのｾｸｼｮﾝの名前とIDを一覧表示",
        en: "ls - List the names and IDs of all sections.",
    },
    HELP_MODE: {
        ja: "mode {id} - 別のｾｸｼｮﾝに切り替えます。",
        en: "mode {id} - Switch to a different section.",
    },
    HELP_TERM_OPEN: {
        ja: "term open {id} - 指定したIDのﾀｰﾐﾅﾙを開く。",
        en: "term open {id} - Open a terminal with the specified ID.",
    },
    HELP_TERM_FORMAT: {
        ja: "term format - 開いているﾀｰﾐﾅﾙを整理する。",
        en: "term format - Arrange the open terminals neatly.",
    },
    PURCHASING_TROUBLE: {
        ja: "購買設備に問題が発生しました！",
        en: "Purchasing terminal encountered a problem!",
    },
    PANTRY_TROUBLE: {
        ja: "パントリー設備に問題が発生しました！",
        en: "Pantry terminal encountered a problem!",
    },
    PACKAGING_TROUBLE: {
        ja: "包装場に問題が発生しました！",
        en: "Packaging terminal encountered a problem!",
    },
    SALES_FRONT_TROUBLE: {
        ja: "販売設備に問題が発生しました！",
        en: "Sales front terminal encountered a problem!",
    },
    WASTE_TROUBLE: {
        ja: "廃棄場に問題が発生しました！",
        en: "Waste terminal encountered a problem!",
    },
} as const;

export const USAGE_EMPTY: UsageCode = {
    ja: USAGE_TEXTS.EMPTY,
} as const;

export const USAGE_HELP_HEADER: UsageCode = USAGE_TEXTS.HELP_HEADER;

export const USAGE_HELP_LS: UsageCode = USAGE_TEXTS.HELP_LS;

export const USAGE_HELP_MODE: UsageCode = USAGE_TEXTS.HELP_MODE;

export const USAGE_HELP_TERM_OPEN: UsageCode = USAGE_TEXTS.HELP_TERM_OPEN;

export const USAGE_HELP_TERM_FORMAT: UsageCode = USAGE_TEXTS.HELP_TERM_FORMAT;

export const USAGE_TERM_FORMATTED: UsageCode = {
    ja: "ﾀｰﾐﾅﾙが整理されました。",
    en: "Terminals have been formatted.",
};

export const USAGE_UNKNOWN_COMMAND: UsageCode = {
    ja: "不明なｺﾏﾝﾄﾞです。",
    en: "Unknown command.",
};

export const USAGE_LS_ITEM = (
    sectionName: string,
    sectionId: string,
): UsageCode => ({
    ja: `-[ ${sectionName} ] ${sectionId}`,
    en: `-[ ${sectionName} ] ${sectionId}`,
});

export const USAGE_INVALID_TERMINAL_ID = (id: string): UsageCode => ({
    ja: `無効なﾀｰﾐﾅﾙID: ${id}。有効なTerminalSectionIdに対応する数値である必要があります。`,
    en: `Invalid Terminal ID: ${id}. Must be a number corresponding to a valid TerminalSectionId.`,
});

export const USAGE_ACTIVATING_TERMINAL = (terminalName: string): UsageCode => ({
    ja: `ﾀｰﾐﾅﾙを表示します: ${terminalName.toUpperCase()}`,
    en: `Activating Terminal: ${terminalName.toUpperCase()}`,
});

export const USAGE_MISSING_ID_TERM_OPEN: UsageCode = {
    ja: "'term open'のIDが不足しています。",
    en: "Missing ID for 'term open'.",
};

export const USAGE_UPDATED_TERMINAL_POSITION = (
    id: number,
    position: { x: number; y: number; z: number },
): UsageCode => ({
    ja: `ﾀｰﾐﾅﾙID: ${id} の位置を更新しました: (${position.x}, ${position.y}, ${position.z})`,
    en: `Updated Terminal ID: ${id} to Position: (${position.x}, ${position.y}, ${position.z})`,
});

export const USAGE_INVALID_TERM_SUBCOMMAND = (
    subCommand: string,
): UsageCode => ({
    ja: `無効なtermサブｺﾏﾝﾄﾞ: ${subCommand}`,
    en: `Invalid term subcommand: ${subCommand}`,
});

export const USAGE_INVALID_MODE_ID = (id: string): UsageCode => ({
    ja: `無効なモードID: ${id}。有効なTerminalSectionIdに対応する数値である必要があります。`,
    en: `Invalid Mode ID: ${id}. Must be a number corresponding to a valid TerminalSectionId.`,
});

export const USAGE_MODE_CHANGED = (modeName: string): UsageCode => ({
    ja: `モードが変更されました: ${modeName.toUpperCase()}`,
    en: `Mode changed to: ${modeName.toUpperCase()}`,
});

export const USAGE_INVALID_OR_MISSING_ID_MODE: UsageCode = {
    ja: "'mode'のIDが無効または不足しています。",
    en: "Invalid or missing ID for 'mode'.",
};

export const USAGE_LANGUAGE_CHANGED = (lang: "ja" | "en"): UsageCode => ({
    ja: `言語が変更されました: ${lang === "ja" ? "日本語" : "英語"}`,
    en: `Language changed to: ${lang === "ja" ? "Japanese" : "English"}`,
});

export const USAGE_COMMAND_NOT_ALLOWED = (cmd: string): UsageCode => ({
    ja: `ｺﾏﾝﾄﾞ "${cmd}" は現在のモードでは使用できません。`,
    en: `Command "${cmd}" is not allowed in the current mode.`,
});

export const USAGE_COMMAND_NOT_ALLOWED_ON_BREAK = (cmd: string): UsageCode => ({
    ja: `ｺﾏﾝﾄﾞ "${cmd}" は休憩中のﾀｰﾐﾅﾙでは使用できません。`,
    en: `Command "${cmd}" cannot be used while the terminal is on break.`,
});

export const USAGE_OPERATION_CEASED = (reason: string): UsageCode => ({
    ja: `営業停止しました。理由: ${reason}。ブラウザを再読み込みください。"`,
    en: `Operations have ceased. Reason: ${reason}. Please reload the browser.`,
});

export const USAGE_REST_SUCCESS = (name: string): UsageCode => ({
    ja: `ﾀｰﾐﾅﾙ: ${name} は休憩中になりました。"`,
    en: `Terminal: ${name} is now on break.`,
});

export const USAGE_REST_FAILURE = (name: string): UsageCode => ({
    ja: `ﾀｰﾐﾅﾙ: ${name} は休憩できません。"`,
    en: `Terminal: ${name} cannot go on break.`,
});

export const USAGE_WORK_SUCCESS = (name: string): UsageCode => ({
    ja: `ﾀｰﾐﾅﾙ: ${name} は作業を再開しました。"`,
    en: `Terminal: ${name} is now back to work.`,
});

export const USAGE_WORK_FAILURE = (name: string): UsageCode => ({
    ja: `ﾀｰﾐﾅﾙ: ${name} は作業を再開できません。"`,
    en: `Terminal: ${name} cannot return to work.`,
});

export const USAGE_HEAL = {
    ja: "設備の修繕をしました。",
    en: "The facilities have been repaired.",
};

export const USAGE_HELP_PURCHASING: UsageCode = {
    ja: "購買ｾｸｼｮﾝ: buy - 原料の調達",
    en: "Purchasing Section: buy - Procure ingredients.",
};

export const USAGE_HELP_PANTRY: UsageCode = {
    ja: "パントリーｾｸｼｮﾝ: ing - 原料の確認, bread - パンの確認",
    en: "Pantry Section: ing - Check ingredients, bread - Check bread.",
};

export const USAGE_HELP_MIXING: UsageCode = {
    ja: "混錬ｾｸｼｮﾝ: plan - 製造計画の確認, repair - 設備の改修",
    en: "Mixing Section: plan - Check production plan, repair - Repair equipment.",
};

export const USAGE_HELP_COOLING: UsageCode = {
    ja: "冷却ｾｸｼｮﾝ: repair - 設備の改修",
    en: "Cooling Section: repair - Repair equipment.",
};

export const USAGE_HELP_SHAPING: UsageCode = {
    ja: "成形ｾｸｼｮﾝ: repair - 設備の改修",
    en: "Shaping Section: repair - Repair equipment.",
};

export const USAGE_HELP_BAKING: UsageCode = {
    ja: "焼成ｾｸｼｮﾝ: repair - 設備の改修",
    en: "Baking Section: repair - Repair equipment.",
};

export const USAGE_HELP_PACKAGING: UsageCode = {
    ja: "包装ｾｸｼｮﾝ: wrap - 包装, seal - 密封",
    en: "Packaging Section: wrap - Wrap, seal - Seal.",
};

export const USAGE_HELP_SALES_FRONT: UsageCode = {
    ja: "販売ｾｸｼｮﾝ: sell - 販売, refund - 返金",
    en: "Sales Front Section: sell - Sell, refund - Refund.",
};

export const USAGE_HELP_WASTE: UsageCode = {
    ja: "廃棄ｾｸｼｮﾝ: dispose - 廃棄, recycle - リサイクル",
    en: "Waste Section: dispose - Dispose, recycle - Recycle.",
};

export const USAGE_HELP_UTILITIES: UsageCode = {
    ja: "ユーティリティｾｸｼｮﾝ: enable - 有効化, disable - 無効化",
    en: "Utilities Section: enable - Enable, disable - Disable.",
};

export const USAGE_DISPOSE_SUCCESS = (
    amount: number,
    terminalName: string,
): UsageCode => ({
    ja: `ﾀｰﾐﾅﾙ: ${terminalName} から ${amount.toFixed(2)} kg の廃棄物を廃棄ｾｸｼｮﾝに移動しました。`,
    en: `Disposed ${amount.toFixed(2)} kg waste from terminal: ${terminalName} to the Waste section.`,
});

export const USAGE_DISPOSE_FAILURE: UsageCode = {
    ja: "廃棄に失敗しました。廃棄ｾｸｼｮﾝがアクティブであることを確認してください。",
    en: "Failed to dispose waste. Ensure the Waste section is active.",
};

export const USAGE_RODENT_ALERT: UsageCode = {
    ja: "ﾈｽﾞﾐの影を見かけたかもしれません。",
    en: "Rodents might have appeared!",
};

export const USAGE_TRAP_SET: UsageCode = {
    ja: "わなを仕掛けました。",
    en: "Trap has been set.",
};

export const USAGE_TRAP_FAILURE = (errorMessage: string): UsageCode => ({
    ja: `トラップ設置に失敗しました: ${errorMessage}`,
    en: `Trap setup failed: ${errorMessage}`,
});

export const USAGE_TRAP_SUCCESS = {
    ja: "罠が成功し、ネズミを半分捕まえました！",
    en: "The trap succeeded, and half of the rodents were caught!",
};

export const USAGE_DESCENT_INTO_ZANGE: UsageCode = {
    ja: "懺悔室に入りました。",
    en: "Entered the confession chamber.",
};
