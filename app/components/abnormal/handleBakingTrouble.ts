import { TerminalStatus } from "@/app/bt.types";
import {
    type TerminalContextType,
    TerminalSectionId,
} from "@/app/context/TerminalContext";
import { USAGE_ALERT_GAS, USAGE_FIRE } from "@/app/utils/usage/usageBaking";

export const handleBakingTrouble = (context: TerminalContextType) => {
    const { addNews, updateTerminalStatus, terminals } = context;

    const bakingTerminal = terminals.find(
        (terminal) => terminal.id === TerminalSectionId.Baking,
    );

    if (bakingTerminal?.status === TerminalStatus.GAS_LEAK) {
        addNews(TerminalSectionId.Baking, USAGE_ALERT_GAS);
        if (Math.random() < 0.01) {
            updateTerminalStatus(
                TerminalSectionId.Baking,
                TerminalStatus.FIRE_HAZARD,
            );
        }
        return;
    }
    if (bakingTerminal?.status === TerminalStatus.FIRE_HAZARD) {
        addNews(TerminalSectionId.Baking, USAGE_FIRE);
    }

    updateTerminalStatus(TerminalSectionId.Baking, TerminalStatus.GAS_LEAK);
};
