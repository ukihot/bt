import { BreadCookingStatus, BreadType, TerminalStatus } from "@/app/bt.types";
import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import { countBreadByStatus } from "@/app/utils/countBreadByStatus";
import {
    USAGE_NO_FIRST_PROOFED_BREAD,
    USAGE_SHAPING_ACTIVITY,
} from "@/app/utils/usage/usageShaping";

export const handleShapingBatch = async (context: TerminalContextType) => {
    const { bread, updateBread, addNews, updateProgress, terminals } = context;

    // FirstProofed 状態のパンを検索
    const firstProofedBread = bread.filter(
        (b) => b.cookStatus === BreadCookingStatus.FirstProofed,
    );
    if (firstProofedBread.length === 0) {
        addNews(TerminalSectionId.Shaping, USAGE_NO_FIRST_PROOFED_BREAD);
        return; // FirstProofed 状態のパンがない場合は終了
    }

    let progress = 0;

    const updateProgressAsync = () =>
        new Promise<void>((resolve) => {
            const intervalId = setInterval(() => {
                updateProgress(TerminalSectionId.Shaping, progress);
                progress += 10;

                if (progress >= 100) {
                    updateProgress(TerminalSectionId.Shaping, 100);
                    clearInterval(intervalId);
                    resolve();
                }
            }, 500);
        });

    await updateProgressAsync();

    // ターミナルのステータスを確認
    const shapingTerminal = terminals.find(
        (t) => t.id === TerminalSectionId.Shaping,
    );
    const isWrongBreadProduced =
        shapingTerminal?.status === TerminalStatus.WRONG_BREAD_PRODUCED;

    // ランダムな BreadType を取得する関数
    const getRandomBreadType = (exclude: BreadType): BreadType => {
        const breadTypes = Object.values(BreadType).filter(
            (type) => type !== exclude,
        );
        return breadTypes[Math.floor(Math.random() * breadTypes.length)];
    };

    // Shaped 状態に更新
    const shapedBread = bread.map((b) => {
        if (b.cookStatus === BreadCookingStatus.FirstProofed) {
            const newBreadType = isWrongBreadProduced
                ? getRandomBreadType(b.kind) // 誤製造
                : b.kind;
            return {
                ...b,
                cookStatus: BreadCookingStatus.Shaped,
                kind: newBreadType,
            };
        }
        return b;
    });
    updateBread(shapedBread);

    // 集計を計算
    const shapedCountByType = countBreadByStatus(
        bread,
        BreadCookingStatus.FirstProofed,
    );

    addNews(
        TerminalSectionId.Shaping,
        USAGE_SHAPING_ACTIVITY(shapedCountByType),
    );
};
