import { Tooltip, UnstyledButton } from '@mantine/core';
import { renderAtom } from '../store';
import { useEffect, useState, type JSX, type ReactNode } from 'react';
import { Play as RenderIcon, Folder as OpenIcon } from 'react-bootstrap-icons';
import classes from './Navbar.module.scss';
import { useSetAtom } from 'jotai';
import { OpenProjectDialog } from './OpenProjectDialog';

const ICON_SIZE = 25;

export function Navbar(): JSX.Element {
    const render = useSetAtom(renderAtom);
    const [openProjectDialogOpened, setOpenProjectDialogOpened] = useState(false);

    useEffect(() => {
        const handleKeyPress = (event: KeyboardEvent): void => {
            if (event.key === 'F5' && !event.ctrlKey && !event.shiftKey && !event.altKey && !event.metaKey) {
                event.preventDefault();
                void render();
            }
        };

        document.addEventListener('keydown', handleKeyPress);
        return (): void => {
            document.removeEventListener('keydown', handleKeyPress);
        };
    }, [render]);

    return (
        <div className={classes.wrapper}>
            <OpenProjectDialog
                opened={openProjectDialogOpened}
                onClose={() => {
                    setOpenProjectDialogOpened(false);
                }}
            />
            <NavbarLink
                label="Open Project"
                icon={<OpenIcon width={ICON_SIZE} height={ICON_SIZE} />}
                onClick={() => {
                    setOpenProjectDialogOpened(true);
                }}
            />
            <NavbarLink
                label="Render (F5)"
                icon={<RenderIcon width={ICON_SIZE} height={ICON_SIZE} />}
                onClick={() => {
                    void render();
                }}
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
