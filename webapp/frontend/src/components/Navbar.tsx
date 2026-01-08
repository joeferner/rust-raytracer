import { Tooltip, UnstyledButton } from '@mantine/core';
import { useEffect, type JSX, type ReactNode } from 'react';
import { Play as RenderIcon, Folder as OpenIcon } from 'react-bootstrap-icons';
import classes from './Navbar.module.scss';
import { OpenProjectDialog } from './OpenProjectDialog';
import { projectStore } from '../stores/store';
import { useSignal } from '@preact/signals-react';
import { showNotification } from '@mantine/notifications';

const ICON_SIZE = 25;

export function Navbar(): JSX.Element {
    const openProjectDialogOpened = useSignal(false);

    useEffect(() => {
        const handleKeyPress = (event: KeyboardEvent): void => {
            if (event.key === 'F5' && !event.ctrlKey && !event.shiftKey && !event.altKey && !event.metaKey) {
                event.preventDefault();
                handleRenderClick();
            }
        };

        document.addEventListener('keydown', handleKeyPress);
        return (): void => {
            document.removeEventListener('keydown', handleKeyPress);
        };
    }, []);

    const handleRenderClick = (): void => {
        void (async (): Promise<void> => {
            try {
                await projectStore.render();
            } catch (err) {
                console.error('failed to render', err);
                // TODO add errors to an error pain
                showNotification({
                    title: 'Failed to render!',
                    message: `${err}`,
                    color: 'red',
                    autoClose: 5000,
                });
            }
        })();
    };

    return (
        <div className={classes.wrapper}>
            <OpenProjectDialog
                opened={openProjectDialogOpened}
                onClose={() => {
                    openProjectDialogOpened.value = false;
                }}
            />
            <NavbarLink
                label="Open Project"
                icon={<OpenIcon width={ICON_SIZE} height={ICON_SIZE} />}
                onClick={() => {
                    openProjectDialogOpened.value = true;
                }}
            />
            <NavbarLink
                label="Render (F5)"
                icon={<RenderIcon width={ICON_SIZE} height={ICON_SIZE} />}
                onClick={handleRenderClick}
            />
        </div>
    );
}

interface NavbarLinkProps {
    icon: ReactNode;
    label: string;
    onClick?: () => void;
}

function NavbarLink({ icon, label, onClick }: NavbarLinkProps): JSX.Element {
    return (
        <Tooltip label={label} position="right" transitionProps={{ duration: 0 }}>
            <UnstyledButton onClick={onClick} className={classes.link}>
                {icon}
            </UnstyledButton>
        </Tooltip>
    );
}
