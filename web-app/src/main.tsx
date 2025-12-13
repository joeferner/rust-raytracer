import React from 'react';
import ReactDOM from 'react-dom/client';
import { MantineProvider } from '@mantine/core';
import '@mantine/core/styles.css';
import { App } from './App.jsx';
import './index.scss';

const root = document.getElementById('root');
if (root) {
    ReactDOM.createRoot(root).render(
        <React.StrictMode>
            <MantineProvider defaultColorScheme="dark">
                <App />
            </MantineProvider>
        </React.StrictMode>
    );
}
