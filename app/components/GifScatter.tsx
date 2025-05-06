import Image from "next/image";
import { useEffect, useState } from "react";

interface GifScatterProps {
    count: number;
}

const GifScatter = ({ count }: GifScatterProps) => {
    const [gifs, setGifs] = useState<
        { id: number; src: string; top: string; left: string }[]
    >([]);

    useEffect(() => {
        const fetchGifs = async () => {
            const response = await fetch("/api/gifs");
            const gifFiles: string[] = await response.json();

            const scatteredGifs = Array.from({ length: count }).map(
                (_, index) => {
                    const randomGif =
                        gifFiles[Math.floor(Math.random() * gifFiles.length)];
                    return {
                        id: index,
                        src: randomGif,
                        top: `${Math.random() * 100}vh`,
                        left: `${Math.random() * 100}vw`,
                    };
                },
            );

            setGifs(scatteredGifs);
        };

        fetchGifs();
    }, [count]);

    return (
        <>
            {gifs.map((gif) => (
                <Image
                    key={gif.id}
                    src={gif.src}
                    alt="scattered gif"
                    className="pointer-events-none absolute h-16 w-16"
                    style={{
                        top: gif.top,
                        left: gif.left,
                    }}
                    width={64}
                    height={64}
                />
            ))}
        </>
    );
};

export default GifScatter;
