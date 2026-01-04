import classes from './Header.module.scss';
import { projectStore, userStore } from '../stores/store';
import { type JSX } from 'react';
import { UnstyledButton } from '@mantine/core';
import type { User } from '../api';
import { LoginDialog } from './LoginDialog';
import { useSignal } from '@preact/signals-react';
import { Github as GithubIcon } from 'react-bootstrap-icons';

const PICTURE_SIZE = 35;

export function Header(): JSX.Element {
    const loginDialogOpened = useSignal(false);

    const handleLoginClick = (): void => {
        loginDialogOpened.value = true;
    };

    const handleLoginDialogClose = (): void => {
        loginDialogOpened.value = false;
    };

    return (
        <div className={classes.header}>
            <div className={classes.title}>
                <img src="/navbar-logo.png" height={40} width={110} />
                <a href="https://github.com/joeferner/caustic/" target="_blank">
                    <GithubIcon width={25} height={25} />
                </a>
            </div>
            <div className={classes.projectName}>
                {projectStore.project.value && <div>{projectStore.project.value.name}</div>}
            </div>
            <div className={classes.userInfo}>
                {userStore.user.value ? (
                    <UserInfo onClick={handleLoginClick} user={userStore.user.value} />
                ) : (
                    <UnstyledButton onClick={handleLoginClick}>Login</UnstyledButton>
                )}
                <LoginDialog opened={loginDialogOpened} onClose={handleLoginDialogClose} />
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
