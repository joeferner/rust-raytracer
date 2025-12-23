import type {
    RenderRequest,
    RenderRequestInit,
    RenderRequestWork,
    RenderResponseData,
    RenderResponseInit,
} from '../types';
import { initWasm, loadOpenscad, renderBlock } from '../wasm';

let workerId = -1;

self.onmessage = (e: MessageEvent<RenderRequest>): void => {
    const { type } = e.data;

    if (type === 'init') {
        void init(e.data);
    } else if (type === 'work') {
        work(e.data);
    } else {
        throw new Error(`Unhandled message type: ${type}`);
    }
};

async function init(data: RenderRequestInit): Promise<void> {
    workerId = data.workerId;

    console.log(`[${workerId}] initializing worker`);
    await initWasm();
    loadOpenscad(data.input);

    const resultsMessage: RenderResponseInit = { type: 'init', workerId };
    self.postMessage(resultsMessage);
}

function work(data: RenderRequestWork): void {
    const { xmin, xmax, ymin, ymax } = data;

    const results = renderBlock(xmin, xmax, ymin, ymax);

    const resultsMessage: RenderResponseData = {
        type: 'data',
        workerId,
        xmin,
        xmax,
        ymin,
        ymax,
        data: results,
    };
    self.postMessage(resultsMessage);
}

export {};
