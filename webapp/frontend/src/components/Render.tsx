import { type JSX } from 'react';
import type { RenderResult } from '../types';
import * as _ from 'radash';
import { useSignal, useSignalEffect } from '@preact/signals-react';
import { useSignalRef } from '@preact/signals-react/utils';
import { projectStore } from '../stores/store';
import { ImageViewer, type ImageViewerHandle } from './ImageViewer';
import { renderEmpty } from '../utils/canvas';

export function Render(): JSX.Element {
    const imageViewerRef = useSignalRef<ImageViewerHandle | null>(null);
    const progress = useSignal(1.0);
    const working = useSignal(false);
    const startTime = useSignal<Date | undefined>(undefined);

    const renderDrawEvent = (ctx: CanvasRenderingContext2D, event: RenderResult): void => {
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
    };

    // subscribe to draw events to render
    useSignalEffect(() => {
        const blockSize = projectStore.renderOptions.value.blockSize;
        const _imageViewerRef = imageViewerRef;

        const unsubscribe = projectStore.subscribeToDrawEvents((event) => {
            if (event.type === 'init') {
                progress.value = 0.0;
                startTime.value = event.startTime;
                working.value = true;
                _imageViewerRef.current?.render((ctx) => {
                    renderEmpty(ctx, blockSize);
                });
            } else if (event.type === 'renderResult') {
                progress.value = event.progress;
                if (event.progress >= 1.0) {
                    working.value = false;
                }
                _imageViewerRef.current?.render((ctx) => {
                    renderDrawEvent(ctx, event);
                });
            }
        });

        return unsubscribe;
    });

    return (<ImageViewer
        ref={imageViewerRef}
        progress={{
            progress,
            startTime,
            working
        }}
        width={projectStore.cameraInfo.value?.width ?? 500}
        height={projectStore.cameraInfo.value?.height ?? 500}
        blockSize={projectStore.renderOptions.value.blockSize} />);
}

