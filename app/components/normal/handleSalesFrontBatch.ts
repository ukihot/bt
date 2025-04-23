import { BreadCookingStatus, BreadType } from "@/app/bt.types";
import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import {
    USAGE_NO_PACKAGED_BREAD,
    USAGE_SALESFRONT_ACTIVITY,
    USAGE_SALESFRONT_SOLD,
} from "@/app/utils/usage/usageSalesFront";

const BREAD_PRICES: Record<BreadType, number> = {
    [BreadType.Anpan]: 104,
    [BreadType.Begguette]: 55,
    [BreadType.Croissant]: 133,
    [BreadType.Naan]: 42,
};

export const handleSalesFrontBatch = async (context: TerminalContextType) => {
    const { bread, updateBread, addNews, updateProgress, updateCash, nigiwai } =
        context;

    const isSold =
        Math.random() * 100 <
        Math.min(Math.max((Math.abs(nigiwai) / 5.0) * 100, 0), 100);
    if (isSold) {
        const shelvedBread = bread.filter(
            (b) => b.cookStatus === BreadCookingStatus.Shelved,
        );

        const totalSales = shelvedBread.reduce(
            (sum, b) =>
                sum +
                Math.round((BREAD_PRICES[b.kind] ?? 0) * Math.abs(nigiwai)),
            0,
        );

        if (totalSales > 0) {
            updateCash("income", totalSales);
            for (const b of shelvedBread) {
                addNews(
                    TerminalSectionId.SalesFront,
                    USAGE_SALESFRONT_SOLD(
                        BreadType[b.kind],
                        Math.round(
                            (BREAD_PRICES[b.kind] ?? 0) * Math.abs(nigiwai),
                        ),
                    ),
                );
            }
        }

        updateBread(
            bread.map((b) =>
                b.cookStatus === BreadCookingStatus.Shelved
                    ? { ...b, cookStatus: BreadCookingStatus.Sold }
                    : b,
            ),
        );
    }

    const packagedBread = bread.filter(
        (b) => b.cookStatus === BreadCookingStatus.Packaged,
    );
    if (packagedBread.length === 0) {
        addNews(TerminalSectionId.SalesFront, USAGE_NO_PACKAGED_BREAD);
        return; // Packaged 状態のパンがない場合は終了
    }

    let progress = 0;

    const updateProgressAsync = () =>
        new Promise<void>((resolve) => {
            const intervalId = setInterval(() => {
                updateProgress(TerminalSectionId.SalesFront, progress);
                progress += 10;

                if (progress >= 100) {
                    updateProgress(TerminalSectionId.SalesFront, 100);
                    clearInterval(intervalId);
                    resolve();
                }
            }, 500);
        });

    await updateProgressAsync();

    // Shelved 状態に更新
    const shelvedBreadUpdated = bread.map((b) =>
        b.cookStatus === BreadCookingStatus.Packaged
            ? { ...b, cookStatus: BreadCookingStatus.Shelved }
            : b,
    );
    updateBread(shelvedBreadUpdated);

    // ニュースを追加
    addNews(TerminalSectionId.SalesFront, USAGE_SALESFRONT_ACTIVITY);
};
