import {defineConfig} from 'vite';
import {highlight} from './hljs/highlightjsPlugin'

import hljs from 'highlight.js/lib/core';
import toml from 'highlight.js/lib/languages/ini';
import markdown from 'highlight.js/lib/languages/markdown';
import yaml from 'highlight.js/lib/languages/yaml';

import bash from './hljs/bash_with_coco.js';
import git_log from './hljs/git_log.js';
import conventionalCommit from './hljs/conventional_commit_spec.js';
import editor from "./hljs/editor.js";
import tera from './hljs/tera.js';

hljs.registerLanguage('bash', bash);
hljs.registerLanguage('shell', bash);
hljs.registerLanguage('conventional', conventionalCommit);
hljs.registerLanguage('toml', toml);
hljs.registerLanguage('markdown', markdown);
hljs.registerLanguage('git', git_log);
hljs.registerLanguage('editor', editor);
hljs.registerLanguage('tera', tera);
hljs.registerLanguage('yaml', yaml);

function highlight(code, lang) {
    if (lang === "text" || lang === "") {
        return '<pre><code v-pre>' + code  + '</code></pre>';
    }
    else {
        return '<pre><code v-pre>' +
            hljs.highlight(code, { language: lang, ignoreIllegals: true }).value +
            '</code></pre>';
    }
}

export default defineConfig({
    title: "Cocogitto",
    description: "The conventional commit toolbox",
    head: [['link', { rel: 'icon', href: '/favicon.png' }]],
    themeConfig: {
        nav: [
            {text: 'Home', link: '/'},
            {text: 'User guide', link: '/guide/init'}
        ],

        sidebar: [
            {
                text: 'User guide',
                items: [
                    {text: 'Repository initialization', link: '/guide/init'},
                    {text: 'Conventional commits', link: '/guide/commit'},
                    {text: 'Check commit history', link: '/guide/check'},
                    {text: 'Managing git hooks', link: '/guide/git_hooks'},
                    {text: 'Sandbox', link: '/guide/verify'},
                    {text: 'Rewrite non-compliant commits', link: '/guide/edit'},
                    {text: 'Conventional commit log', link: '/guide/log'},
                    {text: 'Changelogs', link: '/guide/changelog'},
                    {text: 'Automatic versioning', link: '/guide/bump'},
                    {text: 'Automatic versioning for monorepo', link: '/guide/monorepo'},
                    {text: 'Tags prefix', link: '/guide/tag'},
                    {text: 'Bump hook recipes', link: '/guide/recipes'},
                    {text: 'Miscellaneous', link: '/guide/misc'},
                ]
            },
            {
                text: 'Reference',
                items: [
                    {text: 'Config reference', link: '/reference/config'},
                    {text: 'Template reference', link: 'reference/template'},
                ]
            },
            {
                text: 'CI/CD',
                items: [
                    {text: 'Github action', link: '/ci_cd/action'},
                    {text: 'Github Bot', link: '/ci_cd/bot'},
                    {text: 'Docker', link: '/ci_cd/docker'},
                ]
            },
        ],

        search: {
            provider: 'local',
            options: {
                detailedView: true
            }
        },

        socialLinks: [
            {icon: 'github', link: 'https://github.com/cocogitto/cocogitto'}
        ],

        editLink: {
            pattern: 'https://github.com/cocogitto/cocogitto/edit/main/website/:path'
        }
    },
    markdown: {
        highlight: (code, lang) => highlight(code, lang)
    }
});

