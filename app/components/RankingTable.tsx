import type { RankingItem } from "../home.types";

type Props = {
    items: RankingItem[];
};

export function RankingTable({ items }: Props) {
    return (
        <table className="w-[62%] table-auto border-green-400">
            <thead>
                <tr className="text-green-900">
                    {["Rank", "Name", "Score", "Date Achieved"].map((h) => (
                        <th key={h} className="border border-green-400">
                            {h}
                        </th>
                    ))}
                </tr>
            </thead>
            <tbody>
                {items.map(({ id, name, score, recorded_at }) => (
                    <tr
                        key={id}
                        className={id % 2 === 0 ? "bg-gray-800" : "bg-gray-700"}
                    >
                        <td className="border border-green-400">{id}</td>
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
