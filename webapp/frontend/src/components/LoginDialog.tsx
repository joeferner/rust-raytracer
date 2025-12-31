import { useAtomValue, useSetAtom } from 'jotai';
import { useCallback, type JSX } from 'react';
import { handleGoogleCredentialResponseAtom, handleLogOutAtom, settingsAtom, userAtom } from '../store';
import { GoogleLogin, type GoogleCredentialResponse } from './GoogleLogin';
import { Button, Divider, Modal } from '@mantine/core';
import classes from './LoginDialog.module.scss';

export function LoginDialog({ opened, onClose }: { opened: boolean; onClose: () => void }): JSX.Element | null {
    const WIDTH = 300;
    const user = useAtomValue(userAtom);
    const settings = useAtomValue(settingsAtom);
    const handleGoogleCredentialResponse = useSetAtom(handleGoogleCredentialResponseAtom);
    const handleLogOut = useSetAtom(handleLogOutAtom);

    const onCredentialResponse = useCallback(
        (response: GoogleCredentialResponse): void => {
            const run = async (): Promise<void> => {
                await handleGoogleCredentialResponse({ response });
                onClose();
            };
            void run();
        },
        [handleGoogleCredentialResponse, onClose]
    );

    const onLogOutClick = useCallback(() => {
        handleLogOut();
        onClose();
    }, [handleLogOut, onClose]);

    if (!settings) {
        return null;
    }

    return (
        <Modal opened={opened} onClose={onClose} title="Login" zIndex={2000}>
            <div className={classes.loginDialogOptions}>
                <GoogleLogin
                    clientId={settings.googleClientId}
                    onCredentialResponse={onCredentialResponse}
                    buttonConfig={{ width: WIDTH, theme: 'outline' }}
                />
                {user ? (
                    <>
                        <Divider my="xs" label="OR" labelPosition="center" style={{ width: `${WIDTH}px` }} />
                        <Button onClick={onLogOutClick}>Log Out</Button>
                    </>
                ) : null}
            </div>
        </Modal>
    );
}
