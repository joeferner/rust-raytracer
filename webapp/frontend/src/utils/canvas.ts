export function renderEmpty(ctx: CanvasRenderingContext2D, blockSize: number): void {
    for (let row = 0; ; row++) {
        const y = row * blockSize;
        if (y > ctx.canvas.height) {
            break;
        }
        for (let col = 0; ; col++) {
            const x = col * blockSize;
            if (x > ctx.canvas.width) {
                break;
            }
            const isWhite = (row + col) % 2 === 0;
            ctx.fillStyle = isWhite ? '#ffffff' : '#cccccc';
            ctx.fillRect(x, y, blockSize, blockSize);
        }
    }
}
