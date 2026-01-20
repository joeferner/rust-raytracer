/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */

import {
    createConnection,
    BrowserMessageReader,
    BrowserMessageWriter,
    type InitializeParams,
    type InitializeResult,
    type RequestMessage,
} from 'vscode-languageserver/browser';
import { initWasm, WasmLspServer } from '../wasm';

const reader = new BrowserMessageReader(self);
const writer = new BrowserMessageWriter(self);
const connection = createConnection(reader, writer);

let lspServer: WasmLspServer | null = null;

connection.onInitialize(async (params: InitializeParams): Promise<InitializeResult> => {
    await initWasm();

    lspServer = new WasmLspServer((msg: string) => {
        const json = JSON.parse(msg);
        void writer.write(json);
    });

    const request: RequestMessage = {
        id: 0,
        jsonrpc: '2.0',
        method: 'initialize',
        params,
    };

    try {
        const result = await lspServer.notify_client_message(JSON.stringify(request));
        if (!result) {
            throw new Error('initialize failed');
        }
        return JSON.parse(result).result as InitializeResult;
    } catch (err) {
        console.error('initialize failed', err);
        throw err;
    }
});

// Proxy all other requests to the WASM server
connection.onRequest(async (method, params) => {
    if (!lspServer) {
        throw new Error('LSP server not initialized');
    }

    const request = {
        id: Math.floor(Date.now() + Math.random()),
        jsonrpc: '2.0',
        method,
        params,
    };

    const result = await lspServer.notify_client_message(JSON.stringify(request));
    if (!result) {
        return null;
    }

    const response = JSON.parse(result);
    console.log('LSP response', response);
    return response.result;
});

connection.listen();
