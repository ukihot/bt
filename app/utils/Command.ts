export enum GeneralCommands {
    TERM = "term", // ﾀｰﾐﾅﾙの管理
    MODE = "mode", // モードの移動
    HELP = "help", // ヘルプ
    EN = "en", // 英語モード
    JA = "ja", // 日本語モード
    REST = "rest", // ﾀｰﾐﾅﾙの回復モード起動
    WORK = "work", // ﾀｰﾐﾅﾙの稼働モード起動
    DISPOSE = "dispose", // ごみ処分
    TRAP = "trap", // ﾈｽﾞﾐ捕り
}

export enum PurchasingCommands {
    BUY = "buy", // 原料の調達
}

export enum PantryCommands {
    INGREDIENT = "ing", // 原料の確認
    BREAD = "bread", // パンの確認
}

export enum MixingCommands {
    PLAN = "plan", // 製造計画の確認
    REPAIR = "repair", // 設備の改修
}

export enum CoolingCommands {
    REPAIR = "repair", // 設備の改修
}

export enum ShapingCommands {
    REPAIR = "repair", // 設備の改修
}

export enum BakingCommands {
    REPAIR = "repair", // 設備の改修
}

export enum PackagingCommands {
    SEAL = "seal",
}

export enum SalesFrontCommands {
    SELL = "sell",
    REFUND = "refund",
}

export enum WasteCommands {
    RECYCLE = "recycle",
}

export enum UtilitiesCommands {
    TEMP = "temp", // 室温の調整
}

export enum ZangeCommands {
    Zange = "zange",
}
