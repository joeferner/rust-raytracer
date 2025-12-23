export const ONE_SECOND = 1000;
export const ONE_MINUTE = 60 * ONE_SECOND;
export const ONE_HOUR = 60 * ONE_MINUTE;

export function formatDuration(duration: number): string {
    let result = '';
    let t = duration;

    if (t > ONE_HOUR) {
        const h = Math.floor(t / ONE_HOUR);
        result += `${h}h`;
        t -= h * ONE_HOUR;
    }

    if (t > ONE_MINUTE || result.length > 0) {
        const m = Math.floor(t / ONE_MINUTE);
        result += `${m}m`;
        t -= m * ONE_MINUTE;
    }

    if (t > ONE_SECOND || result.length > 0) {
        const s = Math.floor(t / ONE_SECOND);
        result += `${s}s`;
        t -= s * ONE_SECOND;
    }

    if (result.length == 0) {
        result += `${t.toFixed(0)}ms`;
    }

    return result;
}
