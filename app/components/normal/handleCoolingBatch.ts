import { BreadCookingStatus } from "@/app/bt.types";
import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import { countBreadByStatus } from "@/app/utils/countBreadByStatus";
import {
    USAGE_COOLING_FIRST_PROOFING_COMPLETED,
    USAGE_COOLING_NO_DOUGH_AFTER_MIXING,
    USAGE_COOLING_NO_SHAPED_BREAD,
    USAGE_COOLING_SECOND_PROOFING_COMPLETED,
} from "@/app/utils/usage/usageCooling";

export const handleCoolingBatch = async (context: TerminalContextType) => {
    const { bread, updateBread, addNews, updateProgress } = context;

    // Raw 状態のパンを検索
    const rawBread = bread.filter(
        (b) => b.cookStatus === BreadCookingStatus.Raw,
    );
    if (rawBread.length === 0) {
        addNews(TerminalSectionId.Cooling, USAGE_COOLING_NO_DOUGH_AFTER_MIXING);
        return; // Raw 状態のパンがない場合は終了
    }

    let progress = 0;

    const updateProgressAsync = () =>
        new Promise<void>((resolve) => {
            const intervalId = setInterval(() => {
                updateProgress(TerminalSectionId.Cooling, progress);
                progress += 10;

                if (progress >= 100) {
                    updateProgress(TerminalSectionId.Cooling, 100);
                    clearInterval(intervalId);
                    resolve();
                }
            }, 500);
        });

    await updateProgressAsync();

    // 一次発酵済みの状態に更新
    const firstProofedBread = bread.map((b) =>
        b.cookStatus === BreadCookingStatus.Raw
            ? { ...b, cookStatus: BreadCookingStatus.FirstProofed }
            : b,
    );
    updateBread(firstProofedBread);

    // 集計を計算
    const firstProofedCountByType = countBreadByStatus(
        bread,
        BreadCookingStatus.Raw,
    );

    // ニュースを追加
    addNews(
        TerminalSectionId.Cooling,
        USAGE_COOLING_FIRST_PROOFING_COMPLETED(firstProofedCountByType),
    );

    // Shaped 状態のパンを検索
    const shapedBread = firstProofedBread.filter(
        (b) => b.cookStatus === BreadCookingStatus.Shaped,
    );
    if (shapedBread.length === 0) {
        addNews(TerminalSectionId.Cooling, USAGE_COOLING_NO_SHAPED_BREAD);
        return; // Shaped 状態のパンがない場合は終了
    }

    progress = 0; // プログレスをリセット
    await updateProgressAsync();

    // 二次発酵済みの状態に更新
    const secondProofedBread = firstProofedBread.map((b) =>
        b.cookStatus === BreadCookingStatus.Shaped
            ? { ...b, cookStatus: BreadCookingStatus.SecondProofed }
            : b,
    );
    updateBread(secondProofedBread);

    // 集計を計算
    const secondProofedCountByType = countBreadByStatus(
        bread,
        BreadCookingStatus.Shaped,
    );

    // ニュースを追加
    addNews(
        TerminalSectionId.Cooling,
        USAGE_COOLING_SECOND_PROOFING_COMPLETED(secondProofedCountByType),
    );
};
