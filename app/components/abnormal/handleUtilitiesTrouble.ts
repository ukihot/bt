import type { TerminalContextType } from "@/app/context/TerminalContext";

export const handleUtilitiesTrouble = (context: TerminalContextType) => {
    const { increaseTemperature, decreaseTemperature, terminals } = context;

    const randomTerminal =
        terminals[Math.floor(Math.random() * terminals.length)];
    if (!randomTerminal) return;

    const randomTemperatureChange = Math.random();
    (Math.random() < 0.5 ? increaseTemperature : decreaseTemperature)(
        randomTerminal.id,
        randomTemperatureChange,
    );
};
