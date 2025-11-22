const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

const worker = new Worker('worker.js', { type: 'module' });

function render(x, y) {
    worker.postMessage({ width: canvas.width, height: canvas.height, x, y });
}

worker.onmessage = (ev) => {
    const { x, y, color } = ev.data;
    const { r, g, b } = color;
    ctx.fillStyle = `rgb(${r},${g},${b})`;
    ctx.fillRect(x, y, 1, 1);
};

for (let y = 0; y < canvas.height; y++) {
    for (let x = 0; x < canvas.width; x++) {
        render(x, y);
    }
}
