import type { TerminalContextType } from "@/app/context/TerminalContext";
import { handleBakingTrouble } from "../components/abnormal/handleBakingTrouble";
import { handleCoolingTrouble } from "../components/abnormal/handleCoolingTrouble";
import { handleMixingTrouble } from "../components/abnormal/handleMixingTrouble";
import { handlePackagingTrouble } from "../components/abnormal/handlePackagingTrouble";
import { handlePantryTrouble } from "../components/abnormal/handlePantryTrouble";
import { handlePurchasingTrouble } from "../components/abnormal/handlePurchasingTrouble";
import { handleSalesFrontTrouble } from "../components/abnormal/handleSalesFrontTrouble";
import { handleShapingTrouble } from "../components/abnormal/handleShapingTrouble";
import { handleUtilitiesTrouble } from "../components/abnormal/handleUtilitiesTrouble";
import { handleWasteTrouble } from "../components/abnormal/handleWasteTrouble";

export const useAbnormalHandlers = (context: TerminalContextType) => {
    return {
        handlePurchasingTrouble: () => handlePurchasingTrouble(context),
        handlePantryTrouble: () => handlePantryTrouble(context),
        handleMixingTrouble: () => handleMixingTrouble(context),
        handleCoolingTrouble: () => handleCoolingTrouble(context),
        handleShapingTrouble: () => handleShapingTrouble(context),
        handleBakingTrouble: () => handleBakingTrouble(context),
        handlePackagingTrouble: () => handlePackagingTrouble(context),
        handleSalesFrontTrouble: () => handleSalesFrontTrouble(context),
        handleWasteTrouble: () => handleWasteTrouble(context),
        handleUtilitiesTrouble: () => handleUtilitiesTrouble(context),
    };
};
