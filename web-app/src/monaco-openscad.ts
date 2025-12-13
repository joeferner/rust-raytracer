import type { Monaco } from '@monaco-editor/react';
import * as monaco from 'monaco-editor';

export function registerOpenscadLanguage(m: Monaco): void {
    const languages = (m as typeof monaco).languages;

    languages.register({ id: 'openscad' });

    languages.setLanguageConfiguration('openscad', {
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

    languages.setMonarchTokensProvider('openscad', {
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
