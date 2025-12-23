import type { RenderOptions as StateRenderOptions } from './store';
import type {
    RenderRequestInit,
    RenderRequestWork,
    RenderResponse,
    RenderResponseData,
    RenderResponseInit,
    RenderResult,
} from './types';
import RenderWorker from './workers/renderWorker?worker';

export interface RenderEventInit {
    type: 'init';
    blockSize: number;
    blockCount: number;
    startTime: Date;
}

export interface RenderEventRenderResult extends RenderResult {
    type: 'renderResult';
    progress: number;
}

export type RenderEvent = RenderEventInit | RenderEventRenderResult;

export type RenderCallbackFn = (event: RenderEvent) => unknown;

export interface RenderOptions extends Required<StateRenderOptions> {
    width: number;
    height: number;
    callback: RenderCallbackFn;
}

export class RenderWorkerPool {
    private workers: Worker[] = [];
    private work: RenderRequestWork[] = [];
    private callback?: RenderCallbackFn;
    private blockCount = 0;
    private receivedBlockCount = 0;

    private ensureWorkerCount(threadCount: number): void {
        if (this.workers.length < threadCount) {
            console.log(`increasing worker count from ${this.workers.length} to ${threadCount}`);
            while (this.workers.length < threadCount) {
                const worker = new RenderWorker();
                const workerId = this.workers.length;
                worker.onerror = (err): void => {
                    this.handleWorkerError(workerId, err);
                };
                worker.onmessage = (message): void => {
                    const response = message.data as RenderResponse;
                    const { type } = response;
                    if (type === 'init') {
                        this.handleWorkerInitResponse(response);
                    } else if (type === 'data') {
                        this.handleWorkerDataResponse(response);
                    } else {
                        throw new Error(`unhandled response type: ${type}`);
                    }
                };
                this.workers.push(worker);
            }
        }
    }

    private handleWorkerDataResponse(response: RenderResponseData): void {
        this.receivedBlockCount++;
        const progress = this.receivedBlockCount / this.blockCount;
        this.callback?.({
            ...response,
            type: 'renderResult',
            progress,
        });
        this.sendMoreWork(response.workerId);
    }

    private handleWorkerInitResponse(response: RenderResponseInit): void {
        const { workerId } = response;

        console.log(`[${workerId}] worker initialized`);
        this.sendMoreWork(workerId);
    }

    private sendMoreWork(workerId: number): void {
        const work = this.work.pop();
        if (work) {
            this.workers[workerId].postMessage(work);
        }
    }

    private handleWorkerError(workerId: number, err: ErrorEvent): void {
        console.error(`[${workerId}] worker error`, err);
    }

    public render(threadCount: number, input: string, options: RenderOptions): void {
        this.callback = options.callback;
        this.ensureWorkerCount(threadCount);
        this.populateWorkQueue(options);

        this.blockCount = this.work.length;
        this.receivedBlockCount = 0;
        this.callback?.({
            type: 'init',
            blockSize: options.blockSize,
            blockCount: this.blockCount,
            startTime: new Date(),
        });

        this.initializeAndBeginRender(threadCount, input);
    }

    private populateWorkQueue(options: RenderOptions): void {
        const { blockSize, width, height } = options;

        const work: RenderRequestWork[] = [];
        for (let y = 0; y < height; y += blockSize) {
            for (let x = 0; x < width; x += blockSize) {
                work.push({
                    type: 'work',
                    xmin: x,
                    xmax: Math.min(width, x + blockSize),
                    ymin: y,
                    ymax: Math.min(height, y + blockSize),
                });
            }
        }
        this.work = work.reverse();
        console.log(`work queue initialized with ${work.length} blocks`);
    }

    private initializeAndBeginRender(threadCount: number, input: string): void {
        for (let i = 0; i < threadCount; i++) {
            const message: RenderRequestInit = {
                type: 'init',
                workerId: i,
                input,
            };
            this.workers[i].postMessage(message);
        }
    }
}
