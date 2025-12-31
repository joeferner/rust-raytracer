import { useEffect, type JSX } from 'react';

// Global Google Identity Services types
declare global {
    interface Window {
        google?: {
            accounts: {
                id: {
                    initialize: (config: GoogleIdentityConfiguration) => void;
                    renderButton: (parent: HTMLElement, options: GoogleButtonConfiguration) => void;
                    prompt: () => void;
                    disableAutoSelect: () => void;
                };
            };
        };
    }
}

interface GoogleIdentityConfiguration {
    client_id: string;
    callback: (response: GoogleCredentialResponse) => void;
    auto_select?: boolean;
    cancel_on_tap_outside?: boolean;
    context?: 'signin' | 'signup' | 'use';
    ux_mode?: 'popup' | 'redirect';
    login_uri?: string;
    native_callback?: (response: GoogleCredentialResponse) => void;
    itp_support?: boolean;
}

export interface GoogleCredentialResponse {
    credential: string;
    select_by: string;
    clientId?: string;
}

interface GoogleButtonConfiguration {
    type?: 'standard' | 'icon';
    theme?: 'outline' | 'filled_blue' | 'filled_black';
    size?: 'large' | 'medium' | 'small';
    text?: 'signin_with' | 'signup_with' | 'continue_with' | 'signin';
    shape?: 'rectangular' | 'pill' | 'circle' | 'square';
    logo_alignment?: 'left' | 'center';
    width?: number;
    locale?: string;
}

export interface GoogleLoginProps {
    onCredentialResponse: (response: GoogleCredentialResponse) => void;
    clientId: string;
    buttonConfig?: GoogleButtonConfiguration;
    autoSelect?: boolean;
}

export function GoogleLogin({
    onCredentialResponse: onCredentialResponse,
    clientId,
    buttonConfig = { theme: 'outline', size: 'large' },
    autoSelect = false,
}: GoogleLoginProps): JSX.Element {
    useEffect(() => {
        if (window.google) {
            window.google.accounts.id.initialize({
                client_id: clientId,
                callback: onCredentialResponse,
                auto_select: autoSelect,
            });

            const buttonDiv = document.getElementById('googleSignInButton');
            if (buttonDiv) {
                window.google.accounts.id.renderButton(buttonDiv, buttonConfig);
            }
        }
    }, [onCredentialResponse, clientId, buttonConfig, autoSelect]);

    return <div id="googleSignInButton" style={{ colorScheme: 'light' }}></div>;
}
