import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import { USAGE_TEXTS } from "@/app/utils/usage/usageGeneral";

export const handlePackagingTrouble = (context: TerminalContextType) => {
    const { addNews } = context;

    addNews(TerminalSectionId.Packaging, USAGE_TEXTS.PACKAGING_TROUBLE);
};
