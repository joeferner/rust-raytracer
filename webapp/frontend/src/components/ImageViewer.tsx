import React, { useImperativeHandle, type JSX } from 'react';
import { MiniMap, TransformComponent, TransformWrapper, type ReactZoomPanPinchHandlers } from 'react-zoom-pan-pinch';
import classes from './Render.module.scss';
import { Button, Tooltip } from '@mantine/core';
import { ZoomIn as ZoomInIcon, ZoomOut as ZoomOutIcon, X as ResetZoomIcon } from 'react-bootstrap-icons';
import * as _ from 'radash';
import { RenderProgress, type RenderProgressProps } from './RenderProgress';
import { useSignal, useSignalEffect } from '@preact/signals-react';
import { useSignalRef } from '@preact/signals-react/utils';
import { renderEmpty } from '../utils/canvas';

export const DEFAULT_BLOCK_SIZE = 128;

export type RenderFn = (ctx: CanvasRenderingContext2D) => void;

export interface ImageViewerHandle {
    render: (fn: RenderFn) => void;
}

export interface ImageViewerProps {
    progress?: RenderProgressProps;
    ref?: React.Ref<ImageViewerHandle>;
    blockSize?: number;
    width: number;
    height: number;
}

export const ImageViewer = ({ ref, progress, blockSize, width, height }: ImageViewerProps): JSX.Element => {
    const canvasRef = useSignalRef<HTMLCanvasElement | null>(null);
    const canvasMiniRef = useSignalRef<HTMLCanvasElement | null>(null);
    const showMinimap = useSignal(false);

    const getCanvasCtx = (canvasRef: React.RefObject<HTMLCanvasElement | null>): CanvasRenderingContext2D | undefined => {
        const canvas = canvasRef.current;
        if (!canvas) {
            return undefined;
        }
        const ctx = canvas.getContext('2d');
        if (!ctx) {
            console.error('could not get canvas context');
            return undefined;
        }
        return ctx;
    };

    const internalRender = (fn: RenderFn): void => {
        const ctx = getCanvasCtx(canvasRef)
        if (ctx) {
            fn(ctx);
        }

        const miniCtx = getCanvasCtx(canvasMiniRef)
        if (miniCtx) {
            fn(miniCtx);
        }
    };

    useImperativeHandle(ref, () => ({
        render: (fn: RenderFn): void => {
            internalRender(fn);
        }
    }));

    // update empty background if block size changes
    useSignalEffect(() => {
        internalRender((ctx): void => {
            renderEmpty(ctx, blockSize ?? DEFAULT_BLOCK_SIZE);
        });
    });

    const handleOnZoom = (): void => {
        const canvas = canvasRef.current;
        if (!canvas) {
            return;
        }

        const el = canvas.parentElement;
        const wrapperEl = el?.parentElement;
        if (!el || !wrapperEl) {
            return;
        }

        const elRect = el.getBoundingClientRect();
        const wrapperElRect = wrapperEl.getBoundingClientRect();

        const offScreen =
            elRect.y - wrapperElRect.y < 0 ||
            elRect.x - wrapperElRect.x < 0 ||
            wrapperElRect.right - elRect.right < 0 ||
            wrapperElRect.bottom - elRect.bottom < 0;
        showMinimap.value = offScreen;
    };

    return (
        <div className={classes.wrapper}>
            <TransformWrapper onZoom={handleOnZoom}>
                {(utils) => (
                    <React.Fragment>
                        <div className={classes.miniMap} style={{ display: showMinimap.value ? 'block' : 'none' }}>
                            <MiniMap width={150} height={150}>
                                <canvas
                                    ref={canvasMiniRef}
                                    width={width}
                                    height={height}
                                />
                            </MiniMap>
                        </div>
                        <Controls {...utils} />
                        <TransformComponent>
                            <canvas
                                className={classes.canvas}
                                ref={canvasRef}
                                width={width}
                                height={height}
                            />
                        </TransformComponent>
                    </React.Fragment>
                )}
            </TransformWrapper>
            {progress && <RenderProgress {...progress} />}
        </div>
    );
}

function Controls(options: ReactZoomPanPinchHandlers): JSX.Element {
    const handleZoomInClick = (): void => {
        options.zoomIn();
    };

    const handleZoomOutClick = (): void => {
        options.zoomOut();
    };

    const handleResetZoomClick = (): void => {
        options.resetTransform();
    };

    return (
        <div className={classes.controls}>
            <Tooltip label="Zoom In">
                <Button onClick={handleZoomInClick}>
                    <ZoomInIcon />
                </Button>
            </Tooltip>
            <Tooltip label="Zoom Out">
                <Button onClick={handleZoomOutClick}>
                    <ZoomOutIcon />
                </Button>
            </Tooltip>
            <Tooltip label="Reset Zoom">
                <Button onClick={handleResetZoomClick}>
                    <ResetZoomIcon />
                </Button>
            </Tooltip>
        </div>
    );
}
