"use client";

import { TerminalContext } from "@/app/context/TerminalContext";
import { useContext, useEffect, useRef } from "react";

const Rain = () => {
    const context = useContext(TerminalContext);
    if (!context) return null;

    const { nigiwai, ominous } = context;
    const canvasRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        const canvas = canvasRef.current;
        const ctx = canvas?.getContext("2d");
        if (!canvas || !ctx) return;

        let width = window.innerWidth;
        let height = window.innerHeight;
        canvas.width = width;
        canvas.height = height;

        const random = (min: number, max: number) =>
            min + Math.random() * (max - min);

        const createRaindrop = () => ({
            x: Math.random() * width,
            y: Math.random() * height,
            length: random(11, 55),
            speed: random(5, 10),
            angle: Math.PI / 2 + (Math.random() - 0.5) * 0.062,
            opacity: random(0.2, 1),
            lineWidth: random(0.5, 2),
        });

        const raindrops = Array.from(
            { length: Math.abs(nigiwai) * 350 },
            createRaindrop,
        );
        const ripples: {
            x: number;
            y: number;
            radius: number;
            maxRadius: number;
            alpha: number;
            decay: number;
            lineWidth: number;
        }[] = [];

        const isOminous = ominous !== null;
        const rainColor = isOminous ? "#8B0001" : "white";
        const rippleColor = isOminous ? "#8B0001" : "white";

        const draw = () => {
            ctx.clearRect(0, 0, width, height);

            for (const drop of raindrops) {
                const dx = drop.length * Math.cos(drop.angle);
                const dy = drop.length * Math.sin(drop.angle);

                ctx.beginPath();
                ctx.globalAlpha = drop.opacity;
                ctx.moveTo(drop.x, drop.y);
                ctx.lineTo(drop.x + dx, drop.y + dy);
                ctx.strokeStyle = rainColor;
                ctx.lineWidth = drop.lineWidth;
                ctx.stroke();

                drop.x += dx * drop.speed * 0.1;
                drop.y += dy * drop.speed * 0.1;

                const groundY = height * (0.62 + Math.random() * 0.38);
                if (drop.y > groundY) {
                    ripples.push({
                        x: drop.x,
                        y: groundY,
                        radius: 0,
                        maxRadius: drop.length,
                        alpha: drop.opacity,
                        decay: 0.02,
                        lineWidth: random(0.5, 2),
                    });
                    Object.assign(drop, createRaindrop());
                    drop.y = 0;
                }
            }

            for (let i = ripples.length - 1; i >= 0; i--) {
                const r = ripples[i];
                ctx.beginPath();
                ctx.globalAlpha = r.alpha;
                ctx.arc(r.x, r.y, r.radius, 0, Math.PI * 2);
                ctx.strokeStyle = rippleColor;
                ctx.lineWidth = r.lineWidth;
                ctx.stroke();
                r.radius += 1;
                r.alpha -= r.decay;
                if (r.alpha <= 0 || r.radius > r.maxRadius)
                    ripples.splice(i, 1);
            }

            ctx.globalAlpha = 1;
            requestAnimationFrame(draw);
        };

        draw();

        const handleResize = () => {
            width = window.innerWidth;
            height = window.innerHeight;
            canvas.width = width;
            canvas.height = height;
        };
        window.addEventListener("resize", handleResize);
        return () => window.removeEventListener("resize", handleResize);
    }, [nigiwai, ominous]);

    return (
        <div className="-z-50 pointer-events-none fixed inset-0 overflow-hidden">
            <canvas ref={canvasRef} className="size-full" />
        </div>
    );
};

export default Rain;
