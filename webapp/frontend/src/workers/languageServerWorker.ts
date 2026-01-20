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

connection.onInitialize(async (params: InitializeParams): Promise<InitializeResult> => {
    console.log('onInitialize');

    await initWasm();

    const lspServer = new WasmLspServer((msg: string) => {
        console.log('WasmLspServer message', msg);
        const json = JSON.parse(msg);
        void writer.write(json);
    });

    reader.listen((msg) => {
        console.log('WasmLspServer listen', msg);
        lspServer.notify_client_message(JSON.stringify(msg))
            .then(result => { console.log('notify_client_message result', result); })
            .catch((err: unknown) => { console.error('notify_client_message failed', err); });
    });

    const request: RequestMessage = {
        id: 0,
        jsonrpc: "2.0",
        method: "initialize",
        params
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

connection.listen();
