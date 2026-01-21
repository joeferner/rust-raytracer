import {
    BrowserMessageReader,
    BrowserMessageWriter,
    Message
} from 'vscode-languageserver/browser';
import { initWasm, WasmLspServer } from '../wasm';

const reader = new BrowserMessageReader(self);
const writer = new BrowserMessageWriter(self);

const lspServerPromise = new Promise<WasmLspServer>((resolve, reject) => {
    async function startServer(): Promise<WasmLspServer> {
        await initWasm();

        return new WasmLspServer((msg: string) => {
            console.log('LSP', msg);
            const json = JSON.parse(msg) as Message;
            void writer.write(json);
        });
    }

    startServer().then(resolve).catch(reject);
});

reader.listen((msg: Message) => {
    void processMessage(msg);
});

async function processMessage(msg: Message): Promise<void> {
    try {
        console.log('LSP request', JSON.stringify(msg));
        const lspServer = await lspServerPromise;
        const resultStr = await lspServer.notify_client_message(JSON.stringify(msg));
        if (resultStr) {
            const result = JSON.parse(resultStr) as Message;
            console.log('LSP result', JSON.stringify(result));
            await writer.write(result);
        }
    } catch (err: unknown) {
        console.error('failed to process message', err);
    }
}
