import { initialize } from '@codingame/monaco-vscode-api';
import getConfigurationServiceOverride from '@codingame/monaco-vscode-configuration-service-override';
import {
    BrowserMessageReader,
    BrowserMessageWriter,
    CloseAction,
    ErrorAction,
    type CloseHandlerResult,
    type ErrorHandlerResult,
} from 'vscode-languageclient/browser';
import { MonacoLanguageClient } from 'monaco-languageclient';
import 'vscode/localExtensionHost';
import LanguageServerWorker from './workers/languageServerWorker?worker';
import { Uri } from 'vscode';
import * as monaco from 'monaco-editor';
import { loader } from '@monaco-editor/react';

export const LANGUAGE_ID = 'openscad';

export async function initializeMonaco(): Promise<void> {
    await initialize({
        ...getConfigurationServiceOverride(),
    });

    registerOpenscadLanguage();

    // Create a web worker for the language server
    const worker = new LanguageServerWorker();

    // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-explicit-any
    (self as any).MonacoEnvironment = {
        getWorker: (): Worker => {
            return worker;
        },
    };

    loader.config({ monaco });
    await loader.init();

    // Set up message passing
    const reader = new BrowserMessageReader(worker);
    const writer = new BrowserMessageWriter(worker);

    // Create the language client
    const languageClient = new MonacoLanguageClient({
        name: 'OpenSCAD Language Client',
        clientOptions: {
            documentSelector: [{ language: LANGUAGE_ID, scheme: 'file' }],
            errorHandler: {
                error: (): ErrorHandlerResult => ({ action: ErrorAction.Continue }),
                closed: (): CloseHandlerResult => ({ action: CloseAction.DoNotRestart }),
            },
            synchronize: {},
            workspaceFolder: {
                uri: Uri.parse('file:///workspace'),
                name: 'workspace',
                index: 0,
            },
        },
        messageTransports: {
            reader,
            writer,
        },
    });

    await languageClient.start();
}

export function registerOpenscadLanguage(): void {
    const languages = monaco.languages;

    languages.register({
        id: LANGUAGE_ID,
        extensions: ['.scad'],
        aliases: ['OpenSCAD', 'openscad'],
        mimetypes: ['text/x-openscad'],
    });

    languages.setLanguageConfiguration(LANGUAGE_ID, {
        comments: {
            lineComment: '//',
            blockComment: ['/*', '*/'],
        },
        brackets: [
            ['{', '}'],
            ['[', ']'],
            ['(', ')'],
        ],
        autoClosingPairs: [
            { open: '{', close: '}' },
            { open: '[', close: ']' },
            { open: '(', close: ')' },
            { open: '"', close: '"' },
            { open: "'", close: "'" },
        ],
        surroundingPairs: [
            { open: '{', close: '}' },
            { open: '[', close: ']' },
            { open: '(', close: ')' },
            { open: '"', close: '"' },
            { open: "'", close: "'" },
        ],
    });

    languages.setMonarchTokensProvider(LANGUAGE_ID, {
        keywords: [
            'module',
            'function',
            'if',
            'else',
            'for',
            'let',
            'each',
            'true',
            'false',
            'undef',
            'include',
            'use',
        ],

        primitives: [
            'camera',
            'cube',
            'sphere',
            'cylinder',
            'polyhedron',
            'circle',
            'square',
            'polygon',
            'text',
            'linear_extrude',
            'rotate_extrude',
            'surface',
            'projection',
        ],

        transformations: [
            'translate',
            'rotate',
            'scale',
            'resize',
            'mirror',
            'multmatrix',
            'color',
            'offset',
            'hull',
            'minkowski',
        ],

        boolean: ['union', 'difference', 'intersection', 'render'],

        operators: ['=', '>', '<', '!', '==', '<=', '>=', '!=', '&&', '||', '+', '-', '*', '/', '%', '?', ':'],

        tokenizer: {
            root: [
                [
                    /[a-z_$][\w$]*/,
                    {
                        cases: {
                            '@keywords': 'keyword',
                            '@primitives': 'type',
                            '@transformations': 'keyword.control',
                            '@boolean': 'keyword.control',
                            '@default': 'identifier',
                        },
                    },
                ],

                { include: '@whitespace' },

                [/[{}()\[\]]/, '@brackets'],

                [/\d*\.\d+([eE][\-+]?\d+)?/, 'number.float'],
                [/\d+/, 'number'],

                [/"([^"\\]|\\.)*$/, 'string.invalid'],
                [/"/, 'string', '@string'],
            ],

            whitespace: [
                [/[ \t\r\n]+/, ''],
                [/\/\*/, 'comment', '@comment'],
                [/\/\/.*$/, 'comment'],
            ],

            comment: [
                [/[^\/*]+/, 'comment'],
                [/\*\//, 'comment', '@pop'],
                [/[\/*]/, 'comment'],
            ],

            string: [
                [/[^\\"]+/, 'string'],
                [/"/, 'string', '@pop'],
            ],
        },
    });
}
