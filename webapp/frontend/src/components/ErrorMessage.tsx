import { ExclamationCircle as ErrorIcon } from 'react-bootstrap-icons';
import { Alert } from '@mantine/core';
import type { JSX } from 'react';

export interface ErrorMessageProps {
    title: string;
    message: React.ReactNode;
}

interface AllErrorMessageProps extends ErrorMessageProps {
    width?: string;
}

export function ErrorMessage({ title, message, width }: AllErrorMessageProps): JSX.Element {
    return (
        <Alert variant="light" color="red" title={title} icon={<ErrorIcon />} style={{ width }}>
            {message}
        </Alert>
    );
}
