import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import { USAGE_TEXTS } from "@/app/utils/usage/usageGeneral";

export const handleWasteTrouble = (context: TerminalContextType) => {
    const { addNews } = context;
    addNews(TerminalSectionId.Waste, USAGE_TEXTS.WASTE_TROUBLE);
};
