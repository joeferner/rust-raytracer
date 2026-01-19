/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */

import {
    createConnection,
    BrowserMessageReader,
    BrowserMessageWriter,
    type InitializeParams,
    type InitializeResult,
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
        void lspServer.notify_client_message(JSON.stringify(msg));
    });

    const results = JSON.parse(await lspServer.initialize(JSON.stringify(params)));
    return results.result;
});

connection.listen();
