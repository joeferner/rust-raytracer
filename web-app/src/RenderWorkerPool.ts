import type { RenderOptions as StateRenderOptions } from './state';
import type { RenderDataInit, RenderDataWork, RenderResponse, RenderResponseData, RenderResponseInit } from './types';
import RenderWorker from './workers/renderWorker?worker';

export type RenderCallbackFn = (event: RenderResponse) => unknown;

export interface RenderOptions extends Required<StateRenderOptions> {
    width: number;
    height: number;
    callback: RenderCallbackFn;
}

export class RenderWorkerPool {
    private workers: Worker[] = [];
    private work: RenderDataWork[] = [];
    private callback?: RenderCallbackFn;

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
        this.callback?.(response);
        this.sendMoreWork(response.workerId);
    }

    private handleWorkerInitResponse(response: RenderResponseInit): void {
        const { workerId } = response;

        console.log(`[${workerId}] worker initialized`);
        this.callback?.(response);
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
        this.initializeAndBeginRender(threadCount, input);
    }

    private populateWorkQueue(options: RenderOptions): void {
        const { blockSize, width, height } = options;

        const work: RenderDataWork[] = [];
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
            const message: RenderDataInit = {
                type: 'init',
                workerId: i,
                input,
            };
            this.workers[i].postMessage(message);
        }
    }
}
