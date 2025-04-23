import { v4 as uuidv4 } from "uuid";
import type { News, Terminal, TerminalSectionId } from "../bt.types";

/** JSON化したニュース配列のバイトサイズを計算 */
const calcSize = (news: News[]): number =>
    new TextEncoder().encode(JSON.stringify(news)).length;

/** ニュースリストを12KB以下にトリム */
const trimNews = (news: News[]): News[] => {
    let totalSize = calcSize(news);
    while (totalSize > 1024 * 12 && news.length > 1) {
        news.shift(); // 古いニュースから削除
        totalSize = calcSize(news);
    }
    return news;
};

/** 新しいニュースを追加し、サイズ制限を適用 */
const updateNewsList = (
    news: News[],
    description: string,
    isOverbearing?: boolean,
): News[] => {
    const newNews: News = {
        id: uuidv4(),
        datetime: new Date(),
        description,
        isOverbearing,
    };
    return trimNews([...news, newNews]);
};

/** 指定された Terminal の news を更新する */
export const mapAndUpdate = (
    terminals: Terminal[],
    id: TerminalSectionId,
    description: string,
    isOverbearing?: boolean,
): Terminal[] =>
    terminals.map((terminal) =>
        terminal.id !== id
            ? terminal
            : {
                  ...terminal,
                  news: updateNewsList(
                      terminal.news,
                      description,
                      isOverbearing,
                  ),
              },
    );
