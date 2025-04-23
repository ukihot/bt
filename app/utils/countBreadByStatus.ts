import type { Bread, BreadCookingStatus, BreadType } from "@/app/bt.types";

export const countBreadByStatus = (
    bread: Bread[],
    targetStatus: BreadCookingStatus,
): Record<BreadType, number> => {
    return bread.reduce(
        (acc, b) => {
            if (b.cookStatus === targetStatus) {
                acc[b.kind] = (acc[b.kind] || 0) + 1;
            }
            return acc;
        },
        {} as Record<BreadType, number>,
    );
};
