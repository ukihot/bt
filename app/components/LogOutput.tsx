import { useRef, useLayoutEffect } from "react";
import { LogLevel } from "../bt.types";
import { v4 as uuidv4 } from "uuid";

export type LogProps = { message: string; level: LogLevel };

export const LogOutput = ({ output }: { output: LogProps[] }) => {
    const outputRef = useRef<HTMLDivElement>(null);
    const prevLengthRef = useRef(0);

    useLayoutEffect(() => {
        if (output.length > prevLengthRef.current) {
            outputRef.current?.scrollIntoView({ behavior: "smooth" });
        }
        prevLengthRef.current = output.length;
    }, [output.length]);

    return (
        <div className="flex-grow overflow-y-auto">
            <div className="h-full">
                {output.map(({ message, level }) => (
                    <div
                        key={uuidv4()}
                        className={`py-0.5 ${
                            level === LogLevel.INFO
                                ? "text-green-500"
                                : level === LogLevel.WARN
                                  ? "text-amber-200"
                                  : "text-red-500"
                        }`}
                    >
                        {message.split("\n").map((line) => (
                            <span key={uuidv4()}>
                                {line}
                                <br />
                            </span>
                        ))}
                    </div>
                ))}
                {/* スクロール用ダミー */}
                <div ref={outputRef} />
            </div>
        </div>
    );
};
