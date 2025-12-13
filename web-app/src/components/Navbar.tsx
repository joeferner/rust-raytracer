import { Tooltip, UnstyledButton } from '@mantine/core';
import { useMyContext } from '../state';
import { useEffect, type JSX, type ReactNode } from 'react';
import { Play as RenderIcon } from 'react-bootstrap-icons';
import styles from './Navbar.module.scss';

const ICON_SIZE = 30;

export function Navbar(): JSX.Element {
    const { render } = useMyContext();

    useEffect(() => {
        const handleKeyPress = (event: KeyboardEvent): void => {
            if (event.key === 'F5') {
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
        <div className={styles.wrapper}>
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
            <UnstyledButton onClick={onClick} className={styles.link}>
                {icon}
            </UnstyledButton>
        </Tooltip>
    );
}
