import type { TerminalContextType } from "@/app/context/TerminalContext";
import { handleBakingBatch } from "../components/normal/handleBakingBatch";
import { handleCoolingBatch } from "../components/normal/handleCoolingBatch";
import { handleMixingBatch as mixingBatchHandler } from "../components/normal/handleMixingBatch";
import { handlePackagingBatch } from "../components/normal/handlePackagingBatch";
import { handlePantryBatch } from "../components/normal/handlePantryBatch";
import { handlePurchasingBatch } from "../components/normal/handlePurchasingBatch";
import { handleSalesFrontBatch } from "../components/normal/handleSalesFrontBatch";
import { handleShapingBatch } from "../components/normal/handleShapingBatch";
import { handleUtilitiesBatch } from "../components/normal/handleUtilitiesBatch";
import { handleWasteBatch } from "../components/normal/handleWasteBatch";
import {} from "../utils/usage/usageMixing";

export const useNormalHandlers = (context: TerminalContextType) => {
    return {
        handlePurchasingBatch: () => handlePurchasingBatch(context),
        handlePantryBatch: () => handlePantryBatch(context),
        handleMixingBatch: () => mixingBatchHandler(context),
        handleCoolingBatch: () => handleCoolingBatch(context),
        handleShapingBatch: () => handleShapingBatch(context),
        handleBakingBatch: () => handleBakingBatch(context),
        handlePackagingBatch: () => handlePackagingBatch(context),
        handleSalesFrontBatch: () => handleSalesFrontBatch(context),
        handleWasteBatch: () => handleWasteBatch(context),
        handleUtilitiesBatch: () => handleUtilitiesBatch(context),
    };
};
