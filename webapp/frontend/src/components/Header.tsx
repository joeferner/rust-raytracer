import classes from './Header.module.scss';
import { store } from '../store';
import { useCallback, useState, type JSX } from 'react';
import { UnstyledButton } from '@mantine/core';
import type { User } from '../api';
import { LoginDialog } from './LoginDialog';

const PICTURE_SIZE = 35;

export function Header(): JSX.Element {
    const [loginDialogOpened, setLoginDialogOpened] = useState(false);

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
            <div className={classes.projectName}>{store.project.value && <div>{store.project.value.name}</div>}</div>
            <div className={classes.userInfo}>
                {store.user.value ? (
                    <UserInfo onClick={onLoginClick} user={store.user.value} />
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
                {user?.picture && <img src={user.picture} width={PICTURE_SIZE} height={PICTURE_SIZE} />}
                <div className={classes.details}>
                    <div className={classes.name}>{user?.name}</div>
                    <div className={classes.email}>{user?.email}</div>
                </div>
            </div>
        </UnstyledButton>
    );
}
