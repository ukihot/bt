import { type Terminal, TerminalStatus } from "../bt.types";

export const StatusBar = ({ terminal }: { terminal: Terminal }) => {
    const getTextColor = (value: number, max: number, isTemperature = false) =>
        isTemperature
            ? value <= 15 || value >= 35
                ? "text-rose-700"
                : value < 20 || value > 30
                  ? "text-orange-700"
                  : ""
            : (value / max) * 100 > 80
              ? "text-rose-700"
              : (value / max) * 100 > 60
                ? "text-orange-700"
                : "";

    return (
        <div className="status-bar text-base">
            {(() => {
                switch (terminal.status) {
                    case TerminalStatus.HEALTHY:
                        return (
                            <p className="status-bar-field">
                                <span className="mr-1 text-lime-300">☺️</span>
                                HEALTHY
                                <span className="ml-1 text-lime-300">☺️</span>
                            </p>
                        );
                    case TerminalStatus.ON_BREAK:
                        return (
                            <p className="status-bar-field">
                                <span className="mr-1 text-lime-900">❤️</span>
                                ON_BREAK
                                <span className="ml-1 text-lime-900">❤️</span>
                            </p>
                        );
                    default:
                        return (
                            <p className="status-bar-field">
                                <span className="mr-1 text-lime-900">👹</span>
                                ABNORMAL
                                <span className="ml-1 text-lime-900">👹</span>
                            </p>
                        );
                }
            })()}
            <p
                className={`status-bar-field ${getTextColor(
                    terminal.barometer.roomTemperature,
                    38,
                    true,
                )}`}
            >
                TEMP: {`${terminal.barometer.roomTemperature.toFixed(1)} ℃`}
            </p>
            <p
                className={`status-bar-field ${getTextColor(terminal.barometer.equipmentWear, 100)}`}
            >
                LOAD: {`${terminal.barometer.equipmentWear.toFixed(1)} %`}
            </p>
            <p
                className={`status-bar-field ${getTextColor(terminal.barometer.wasteOverflow, 100)}`}
            >
                WASTE: {`${terminal.barometer.wasteOverflow.toFixed(1)} kg`}
            </p>
            {terminal.barometer.trap > 0 && (
                <p className="status-bar-field">
                    TRAP: {terminal.barometer.trap}
                </p>
            )}
        </div>
    );
};
