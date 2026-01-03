import { useCallback, type JSX } from 'react';
import { store } from '../store';
import { GoogleLogin, type GoogleCredentialResponse } from './GoogleLogin';
import { Button, Divider, Modal } from '@mantine/core';
import classes from './LoginDialog.module.scss';

export function LoginDialog({ opened, onClose }: { opened: boolean; onClose: () => void }): JSX.Element | null {
    const WIDTH = 300;

    const onCredentialResponse = useCallback(
        (response: GoogleCredentialResponse): void => {
            const run = async (): Promise<void> => {
                await store.handleGoogleCredentialResponse({ response });
                onClose();
            };
            void run();
        },
        [onClose]
    );

    const onLogOutClick = useCallback(() => {
        store.logOut();
        onClose();
    }, [onClose]);

    if (!store.settings.value) {
        return null;
    }

    return (
        <Modal opened={opened} onClose={onClose} title="Login" zIndex={2000}>
            <div className={classes.loginDialogOptions}>
                <GoogleLogin
                    clientId={store.settings.value.googleClientId}
                    onCredentialResponse={onCredentialResponse}
                    buttonConfig={{ width: WIDTH, theme: 'outline' }}
                />
                {store.user.value && (
                    <>
                        <Divider my="xs" label="OR" labelPosition="center" style={{ width: `${WIDTH}px` }} />
                        <Button onClick={onLogOutClick}>Log Out</Button>
                    </>
                )}
            </div>
        </Modal>
    );
}
