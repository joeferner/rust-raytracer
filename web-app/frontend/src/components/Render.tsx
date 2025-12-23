import React, { useCallback, useEffect, useRef, useState, type JSX } from 'react';
import { cameraInfoAtom, renderOptionsAtom, subscribeToDrawEvents } from '../store';
import { MiniMap, TransformComponent, TransformWrapper, type ReactZoomPanPinchHandlers } from 'react-zoom-pan-pinch';
import styles from './Render.module.scss';
import { Button, Tooltip } from '@mantine/core';
import { ZoomIn as ZoomInIcon, ZoomOut as ZoomOutIcon, X as ResetZoomIcon } from 'react-bootstrap-icons';
import type { RenderResult } from '../types';
import * as _ from 'radash';
import { RenderProgress } from './RenderProgress';
import { useAtomValue } from 'jotai';

export function Render(): JSX.Element {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    const canvasMiniRef = useRef<HTMLCanvasElement | null>(null);
    const [showMinimap, setShowMinimap] = useState(false);
    const cameraInfo = useAtomValue(cameraInfoAtom);
    const renderOptions = useAtomValue(renderOptionsAtom);
    const [progress, setProgress] = useState(1.0);
    const [working, setWorking] = useState(false);
    const [startTime, setStartTime] = useState<Date | undefined>(undefined);

    useEffect(() => {
        renderEmpty(canvasRef, renderOptions.blockSize);
        renderEmpty(canvasMiniRef, renderOptions.blockSize);
    }, [renderOptions]);

    useEffect(() => {
        const unsubscribe = subscribeToDrawEvents((event) => {
            if (event.type === 'init') {
                setProgress(0.0);
                setStartTime(event.startTime);
                setWorking(true);
                renderEmpty(canvasRef, renderOptions.blockSize);
                renderEmpty(canvasMiniRef, renderOptions.blockSize);
            } else if (event.type === 'renderResult') {
                setProgress(event.progress);
                if (event.progress >= 1.0) {
                    setWorking(false);
                }
                renderDrawEvent(canvasRef, event);
                renderDrawEvent(canvasMiniRef, event);
            }
        });

        return unsubscribe;
    }, [canvasRef, renderOptions, setProgress, setStartTime]);

    const handleOnZoom = useCallback(() => {
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
        setShowMinimap(offScreen);
    }, []);

    return (
        <div className={styles.wrapper}>
            <TransformWrapper onZoom={handleOnZoom}>
                {(utils) => (
                    <React.Fragment>
                        <div className={styles.miniMap} style={{ display: showMinimap ? 'block' : 'none' }}>
                            <MiniMap width={150} height={150}>
                                <canvas
                                    ref={canvasMiniRef}
                                    width={cameraInfo?.width ?? 500}
                                    height={cameraInfo?.height ?? 500}
                                />
                            </MiniMap>
                        </div>
                        <Controls {...utils} />
                        <TransformComponent>
                            <canvas
                                className={styles.canvas}
                                ref={canvasRef}
                                width={cameraInfo?.width ?? 500}
                                height={cameraInfo?.height ?? 500}
                            />
                        </TransformComponent>
                    </React.Fragment>
                )}
            </TransformWrapper>
            <RenderProgress progress={progress} startTime={startTime} working={working} />
        </div>
    );
}

function Controls(options: ReactZoomPanPinchHandlers): JSX.Element {
    return (
        <div className={styles.controls}>
            <Tooltip label="Zoom In">
                <Button
                    onClick={() => {
                        options.zoomIn();
                    }}
                >
                    <ZoomInIcon />
                </Button>
            </Tooltip>
            <Tooltip label="Zoom Out">
                <Button
                    onClick={() => {
                        options.zoomOut();
                    }}
                >
                    <ZoomOutIcon />
                </Button>
            </Tooltip>
            <Tooltip label="Reset Zoom">
                <Button
                    onClick={() => {
                        options.resetTransform();
                    }}
                >
                    <ResetZoomIcon />
                </Button>
            </Tooltip>
        </div>
    );
}

function getCanvasCtx(canvasRef: React.RefObject<HTMLCanvasElement | null>): CanvasRenderingContext2D | undefined {
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
    return ctx;
}

function renderEmpty(canvasRef: React.RefObject<HTMLCanvasElement | null>, blockSize: number): void {
    const ctx = getCanvasCtx(canvasRef);
    if (!ctx) {
        return;
    }

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

function renderDrawEvent(canvasRef: React.RefObject<HTMLCanvasElement | null>, event: RenderResult): void {
    const ctx = getCanvasCtx(canvasRef);
    if (!ctx) {
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
}
