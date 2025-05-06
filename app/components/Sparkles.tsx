import { useEffect, useState } from "react";

export function Sparkles() {
    const [sparkles, setSparkles] = useState<
        { id: number; top: string; left: string }[]
    >([]);

    useEffect(() => {
        const generatedSparkles = Array.from({ length: 62 }).map(
            (_, index) => ({
                id: index,
                top: `${Math.random() * 100}vh`,
                left: `${Math.random() * 100}vw`,
            }),
        );
        setSparkles(generatedSparkles);
    }, []);

    return (
        <>
            {sparkles.map((sparkle) => (
                <div
                    key={sparkle.id}
                    className="pointer-events-none absolute h-1.5 w-1.5 animate-sparkle rounded-full bg-transparent"
                    style={{
                        top: sparkle.top,
                        left: sparkle.left,
                        background:
                            "radial-gradient(circle, white, transparent)",
                    }}
                />
            ))}
        </>
    );
}
