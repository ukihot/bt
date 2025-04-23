import { TerminalStatus } from "@/app/bt.types";
import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import { USAGE_COOLING_OVERHEAT_DETECTED } from "@/app/utils/usage/usageCooling";

export const handleCoolingTrouble = (context: TerminalContextType) => {
    const { addNews, updateTerminalStatus, increaseTemperature } = context;

    updateTerminalStatus(TerminalSectionId.Cooling, TerminalStatus.OVERHEATING);
    increaseTemperature(TerminalSectionId.Cooling, 0.1);
    addNews(TerminalSectionId.Cooling, USAGE_COOLING_OVERHEAT_DETECTED);
};
