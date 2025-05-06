import { useState, useEffect } from "react";

interface QA {
    text: string;
}

export default function RealNewsTrain() {
    // 問題＋正解の文字列リストと読み込み状態のステート
    const [qas, setQas] = useState<QA[]>([]);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        // Open Trivia Database から 8 問を取得
        const url = "https://opentdb.com/api.php?amount=8";

        fetch(url)
            .then((res) => {
                if (!res.ok) {
                    throw new Error(`HTTP error! status: ${res.status}`);
                }
                return res.json();
            })
            .then((data) => {
                interface TriviaItem {
                    category: string;
                    type: string;
                    difficulty: string;
                    question: string;
                    correct_answer: string;
                    incorrect_answers: string[];
                }

                const merged = data.results.map((item: TriviaItem) => {
                    const question = item.question;
                    const answer = item.correct_answer;
                    return { text: `${question}：${answer}` };
                });
                setQas(merged);
            })
            .catch((err) => {
                console.error("Failed to fetch trivia:", err);
            })
            .finally(() => {
                setLoading(false);
            });
    }, []);

    if (loading) {
        return null;
    }

    // 全テキストを一続きに結合し、文字ごとの配列に変換
    const decodeHtml = (html: string) => {
        const txt = document.createElement("textarea");
        txt.innerHTML = html;
        return txt.value;
    };

    const text = qas.map((qa) => decodeHtml(qa.text)).join(" ");
    const characters = text.split("");

    return (
        <div className="pointer-events-none fixed inset-0 z-50 overflow-hidden">
            {characters.map((char, index) => {
                const key = `${char}-${index}`;
                return (
                    <span
                        key={key}
                        className="train-scroll text-gray-100"
                        style={{ animationDelay: `${index * 0.1}s` }}
                    >
                        {char}
                    </span>
                );
            })}
        </div>
    );
}
