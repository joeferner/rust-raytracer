import classes from './Header.module.scss';
import { userAtom } from '../store';
import { useAtomValue } from 'jotai';
import { useCallback, useState, type JSX } from 'react';
import { UnstyledButton } from '@mantine/core';
import type { User } from '../api';
import { LoginDialog } from './LoginDialog';

const PICTURE_SIZE = 35;

export function Header(): JSX.Element {
    const [loginDialogOpened, setLoginDialogOpened] = useState(false);
    const user = useAtomValue(userAtom);

    const onLoginClick = useCallback(() => {
        setLoginDialogOpened(true);
    }, [setLoginDialogOpened]);

    const onLoginDialogClose = useCallback(() => {
        setLoginDialogOpened(false);
    }, [setLoginDialogOpened]);

    return (
        <div className={classes.header}>
            <div className={classes.title}>
                <img src="/navbar-logo.png" height={40} width={110} />
            </div>
            <div className={classes.userInfo}>
                {user ? (
                    <UserInfo onClick={onLoginClick} user={user} />
                ) : (
                    <UnstyledButton onClick={onLoginClick}>Login</UnstyledButton>
                )}
                <LoginDialog opened={loginDialogOpened} onClose={onLoginDialogClose} />
            </div>
        </div>
    );
}

function UserInfo({ user, onClick }: { user: User; onClick: () => void }): JSX.Element {
    return (
        <UnstyledButton variant="outline" onClick={onClick}>
            <div className={classes.userInfo}>
                {user?.picture ? <img src={user.picture} width={PICTURE_SIZE} height={PICTURE_SIZE} /> : null}
                <div className={classes.details}>
                    <div className={classes.name}>{user?.name}</div>
                    <div className={classes.email}>{user?.email}</div>
                </div>
            </div>
        </UnstyledButton>
    );
}
