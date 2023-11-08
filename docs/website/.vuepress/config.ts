import {highlightjsPlugin} from "./hljs/highlightjsPlugin";
import {defaultTheme, defineUserConfig} from "vuepress";


export default defineUserConfig({
    lang: "en-US",
    title: ' ',
    base: "/",
    markdown: {
        code: {
            lineNumbers: false
        }
    },

    plugins: [
        highlightjsPlugin,
    ],

    head: [
        ['link', { rel: 'icon', href: '/favicon.png' }],
        ['meta', {name: 'theme-color', content: '#f86b6a'}],
        ['meta', {name: 'apple-mobile-web-app-capable', content: 'yes'}],
        ['meta', {name: 'apple-mobile-web-app-status-bar-style', content: 'black'}],
        ['meta', {property: 'og:title', content: 'Cocogitto'}],
        ['meta', {property: 'og:image', content: 'https://docs.cocogitto.io/logo.png'}],
        ['meta', {property: 'twitter:card', content: 'https://docs.cocogitto.io/logo.png'}],
        ['meta', {property: 'og:description', content: 'The Conventional Commits toolbox'}],
        ['meta', {property: 'og:width', content: '100'}],
    ],

    theme: defaultTheme({
        logo: 'logo_no_text.png',
        repo: 'https://github.com/cocogitto/cocogitto',
        docsRepo: 'https://github.com/cocogitto/website',
        docsDir: 'src',
        lastUpdated: true,
        navbar: [
            {
                link: '/guide/',
                text: 'User guide',
            },
            {
                link: '/ci_cd/',
                text: 'GitHub integration',
            },
            {
                link: '/config/',
                text: 'Configuration reference',
            },
            {
                link: '/template/',
                text: 'Template reference',
            }
        ],
        // sidebar object
        // pages under different sub paths will use different sidebar
        sidebar: [
            {
                link: '/guide/',
                text: 'User guide',
            },
            {
                link: '/ci_cd/',
                text: 'GitHub integration',
            },
            {
                link: '/config/',
                text: 'Configuration reference',
            },
            {
                link: '/template/',
                text: 'Changelog template reference',
            }
        ],
    }),
})