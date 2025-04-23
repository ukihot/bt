import type { Terminal } from "../bt.types";
import { TerminalSectionId } from "../context/TerminalContext";

export const TitleBar = ({
    isDragging,
    handleMouseDown,
    setIsMinimized,
    deactivateTerminal,
    terminal,
}: {
    isDragging: boolean;
    handleMouseDown: (e: React.MouseEvent<HTMLDivElement>) => void;
    setIsMinimized: React.Dispatch<React.SetStateAction<boolean>>;
    deactivateTerminal: () => void;
    terminal: Terminal;
}) => (
    <div
        className={`title-bar cursor-grab ${isDragging ? "cursor-grabbing" : ""}`}
        onMouseDown={handleMouseDown}
    >
        <div className="title-bar-text text-base">
            {terminal.id}&nbsp;:&nbsp;
            {TerminalSectionId[terminal.id].toUpperCase()}
        </div>
        <div className="title-bar-controls">
            <button
                type="button"
                aria-label="Minimize"
                onClick={() => setIsMinimized(true)}
            />
            <button
                type="button"
                aria-label="Maximize"
                onClick={() => setIsMinimized(false)}
            />
            <button
                type="button"
                aria-label="Close"
                onClick={deactivateTerminal}
            />
        </div>
    </div>
);
