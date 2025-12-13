import { useEffect, useRef } from "react";
import { useMyContext } from "../state";

export function Render() {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    const { cameraInfo, subscribeToDrawEvents } = useMyContext();

    useEffect(() => {
        const unsubscribe = subscribeToDrawEvents((event) => {
            const canvas = canvasRef.current;
            if (!canvas) {
                console.error('canvas not set');
                return;
            }
            const ctx = canvas.getContext('2d');
            if (!ctx) {
                console.error('could not get canvas context');
                return;
            }

            const { xmin, xmax, ymin, ymax, data } = event;
            let i = 0;
            for (let y = ymin; y < ymax; y++) {
                for (let x = xmin; x < xmax; x++) {
                    const color = data[i++];
                    const { r, g, b } = color;
                    ctx.fillStyle = `rgb(${r},${g},${b})`;
                    ctx.fillRect(x, y, 1, 1);
                }
            }
        });

        return unsubscribe;
    }, [subscribeToDrawEvents, canvasRef]);

    console.log('render');
    return (
        <div>
            <canvas ref={canvasRef} width={cameraInfo?.width ?? 100} height={cameraInfo?.height ?? 100} />
        </div>
    )
}
