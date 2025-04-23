import { BreadCookingStatus } from "@/app/bt.types";
import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import { countBreadByStatus } from "@/app/utils/countBreadByStatus";
import {
    USAGE_BAKING_ACTIVITY,
    USAGE_NO_SECOND_PROOFED_BREAD,
} from "@/app/utils/usage/usageBaking";

export const handleBakingBatch = async (context: TerminalContextType) => {
    const { bread, updateBread, addNews, updateProgress } = context;

    // SecondProofed 状態のパンを検索
    const secondProofedBread = bread.filter(
        (b) => b.cookStatus === BreadCookingStatus.SecondProofed,
    );
    if (secondProofedBread.length === 0) {
        addNews(TerminalSectionId.Baking, USAGE_NO_SECOND_PROOFED_BREAD);
        return; // SecondProofed 状態のパンがない場合は終了
    }

    let progress = 0;

    const updateProgressAsync = () =>
        new Promise<void>((resolve) => {
            const intervalId = setInterval(() => {
                updateProgress(TerminalSectionId.Baking, progress);
                progress += 10;

                if (progress >= 100) {
                    updateProgress(TerminalSectionId.Baking, 100);
                    clearInterval(intervalId);
                    resolve();
                }
            }, 500);
        });

    await updateProgressAsync();

    // Baked 状態に更新
    const bakedBread = bread.map((b) =>
        b.cookStatus === BreadCookingStatus.SecondProofed
            ? { ...b, cookStatus: BreadCookingStatus.Baked }
            : b,
    );
    updateBread(bakedBread);

    // 集計を計算
    const bakedCountByType = countBreadByStatus(
        bread,
        BreadCookingStatus.SecondProofed,
    );

    // ニュースを追加
    addNews(TerminalSectionId.Baking, USAGE_BAKING_ACTIVITY(bakedCountByType));
};
