import type { RankingItem } from "../home.types";

type Props = {
    items: RankingItem[];
};

export function RankingTable({ items }: Props) {
    const sortedItems = [...items].sort((a, b) => b.score - a.score);

    return (
        <table className="w-[62%] table-auto border-green-400">
            <thead>
                <tr className="text-green-900">
                    {["Rank", "Name", "Score", "Date Achieved"].map((h) => (
                        <th key={h}>{h}</th>
                    ))}
                </tr>
            </thead>
            <tbody>
                {sortedItems.map(({ id, name, score, recorded_at }, index) => (
                    <tr
                        key={id}
                        className={
                            index % 2 === 0 ? "bg-gray-800" : "bg-gray-700"
                        }
                    >
                        <td className="!p-2 border border-green-400">
                            {index + 1}
                        </td>
                        <td className="border border-green-400">{name}</td>
                        <td className="border border-green-400">{score}</td>
                        <td className="border border-green-400">
                            {new Date(recorded_at).toLocaleDateString("en-GB")}
                        </td>
                    </tr>
                ))}
            </tbody>
        </table>
    );
}
