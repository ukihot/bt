import { useEffect, useRef } from "react";
import { type News, TerminalStatus } from "../bt.types";

export const NewsContainer = ({
    news,
    terminalStatus,
}: {
    news: News[];
    terminalStatus: TerminalStatus;
}) => {
    const newsContainerRef = useRef<HTMLDivElement>(null);
    const prevNewsLength = useRef(news.length);

    useEffect(() => {
        if (!newsContainerRef.current) return;

        const isHealthy =
            terminalStatus === TerminalStatus.HEALTHY ||
            terminalStatus === TerminalStatus.ON_BREAK;
        if (news.length > prevNewsLength.current && !isHealthy) {
            const items = newsContainerRef.current.children;
            if (items.length > 0) {
                const lastItem = items[items.length - 1] as HTMLElement;
                lastItem.classList.add("glitch");

                setTimeout(() => {
                    lastItem.classList.remove("glitch");
                }, 200);
            }
        }

        newsContainerRef.current.scrollTo({
            top: newsContainerRef.current.scrollHeight,
            behavior: "smooth",
        });

        prevNewsLength.current = news.length;
    }, [news, terminalStatus]);

    return (
        <div
            ref={newsContainerRef}
            className="window-body max-h-[340px] w-[550px] overflow-y-scroll text-base"
        >
            {news.map((item) => (
                <div key={item.id} className="flex flex-row space-x-4">
                    <p className="min-w-max flex-shrink-0 whitespace-nowrap">
                        [&nbsp;{item.datetime.toISOString()}&nbsp;]
                    </p>
                    <strong
                        className={`flex-1 break-words ${
                            item.isOverbearing ? "text-red-900" : ""
                        }`}
                    >
                        {item.description}
                    </strong>
                </div>
            ))}
        </div>
    );
};
