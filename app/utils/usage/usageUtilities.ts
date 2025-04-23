import type { UsageCode } from "../../bt.types";

// 不明なｺﾏﾝﾄﾞに関するメッセージ
export const USAGE_UNKNOWN_UTILITIES_COMMAND = (cmd: string): UsageCode => ({
    ja: `不明な設備ｺﾏﾝﾄﾞ: ${cmd}`,
    en: `Unknown utilities command: ${cmd}`,
});

// 温度更新に関するメッセージ
export const USAGE_UPDATED_TEMPERATURE = (
    terminalId: number,
    newTemperature: number,
): UsageCode => ({
    ja: `ﾀｰﾐﾅﾙ ${terminalId} の温度が ${newTemperature} 度に設定されました。`,
    en: `Terminal ${terminalId}'s temperature has been set to ${newTemperature} degrees.`,
});

// ﾀｰﾐﾅﾙIDまたは設定温度が不足している場合のメッセージ
export const USAGE_MISSING_TEMPERATURE_OR_ID: UsageCode = {
    ja: "ﾀｰﾐﾅﾙIDまたは設定温度が不足しています。",
    en: "Terminal ID or target temperature is missing.",
};

// ﾀｰﾐﾅﾙIDまたは設定温度が無効な場合のメッセージ
export const USAGE_INVALID_TEMPERATURE_OR_ID: UsageCode = {
    ja: "ﾀｰﾐﾅﾙIDまたは設定温度が無効です。",
    en: "Invalid Terminal ID or target temperature.",
};

// 指定されたﾀｰﾐﾅﾙが見つからない場合のメッセージ
export const USAGE_TERMINAL_NOT_FOUND = (terminalId: number): UsageCode => ({
    ja: `ﾀｰﾐﾅﾙ ${terminalId} が見つかりません。`,
    en: `Terminal ${terminalId} not found.`,
});
