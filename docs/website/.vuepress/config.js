const {description} = require('../../../site/package.json')

module.exports = {
    /**
     * Ref：https://v1.vuepress.vuejs.org/config/#title
     */
    title: 'Cocogitto',

    /**
     * Ref：https://v1.vuepress.vuejs.org/config/#description
     */
    description: description,

    markdown: {
        code: {
            lineNumbers: false
        }
    },
    /**
     * Extra tags to be injected to the page HTML `<head>`
     *
     * ref：https://v1.vuepress.vuejs.org/config/#head
     */
    head: [
        ['meta', {name: 'theme-color', content: '#f86b6a'}],
        ['meta', {name: 'apple-mobile-web-app-capable', content: 'yes'}],
        ['meta', {name: 'apple-mobile-web-app-status-bar-style', content: 'black'}]
    ],

    /**
     * Theme configuration, here is the default theme configuration for VuePress.
     *
     * ref：https://v1.vuepress.vuejs.org/theme/default-theme-config.html
     */
    themeConfig: {
        repo: 'https://github.com/oknozor/cocogitto',
        editLinks: false,
        docsDir: '',
        editLinkText: '',
        lastUpdated: false,
        navbar: [
            {
                text: 'Guide',
                link: '/guide/',
            },
            {
                text: 'Config',
                link: '/config/'
            },
        ],
        // sidebar object
        // pages under different sub paths will use different sidebar
        sidebar: [
            {
                link: '/guide/',
                text: 'Guide',
                children: [
                    'README.md',
                    'initialization',
                    'conventional-commits',
                    'commit_history',
                    'versioning',
                    'github_action'
                ],
            },
            {
                link: '/config/',
                text: 'Configuration reference',
                children: ['/config/README.md'],
            }
        ],
        /**
         * Apply plugins，ref：https://v1.vuepress.vuejs.org/zh/plugin/
         */
        plugins: [
            '@vuepress/plugin-back-to-top',
            '@vuepress/plugin-medium-zoom',
        ]
    }
}
