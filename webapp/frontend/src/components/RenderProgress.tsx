import { Progress } from '@mantine/core';
import { useEffect, useState, type JSX } from 'react';
import classes from './RenderProgress.module.scss';
import { formatDuration } from '../utils/time';

export interface RenderProgressOptions {
    progress: number;
    working: boolean;
    startTime?: Date;
}

export function RenderProgress(options: RenderProgressOptions): JSX.Element | null {
    const { progress, startTime, working } = options;
    const [endTime, setEndTime] = useState<Date | undefined>(undefined);

    useEffect(() => {
        if (!working) {
            return;
        }

        setTimeout(() => {
            setEndTime(new Date());
        });
        const interval = setInterval(() => {
            setEndTime(new Date());
        }, 1000);

        return (): void => {
            clearInterval(interval);
            setEndTime(new Date());
        };
    }, [working]);

    if (!startTime) {
        return null;
    }

    const progressPercentStr = (progress * 100.0).toFixed(0);

    let durationStr = '';
    let etaStr = '';
    if (endTime) {
        const duration = endTime.getTime() - startTime.getTime();
        durationStr = formatDuration(duration);

        if (working && progress > 0.0) {
            const estimatedTotalTime = duration / progress;
            const eta = estimatedTotalTime - duration;
            etaStr = `(eta ${formatDuration(eta)})`;
        }
    }

    const progressLabel = `${progressPercentStr}% ${durationStr} ${etaStr}`;

    return (
        <Progress.Root radius="xs" size={30} className={classes.progressRoot}>
            <Progress.Section value={progress * 100.0} />
            <Progress.Section value={0.0001} className={classes.label}>
                <Progress.Label>{progressLabel}</Progress.Label>
            </Progress.Section>
        </Progress.Root>
    );
}
