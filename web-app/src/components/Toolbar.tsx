import { Button } from '@mantine/core';
import { useMyContext } from '../state';
import type { JSX } from 'react';

export function Toolbar(): JSX.Element {
    const { render } = useMyContext();

    return (
        <div>
            <Button
                onClick={() => {
                    void render();
                }}
            >
                Render
            </Button>
        </div>
    );
}
