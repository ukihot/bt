import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import { USAGE_TEXTS } from "@/app/utils/usage/usageGeneral";

export const handleSalesFrontTrouble = (context: TerminalContextType) => {
    const { addNews } = context;
    addNews(TerminalSectionId.SalesFront, USAGE_TEXTS.SALES_FRONT_TROUBLE);
};
