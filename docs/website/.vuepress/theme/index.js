import { getDirname, path } from '@vuepress/utils'

const __dirname = getDirname(import.meta.url)

const localTheme = (options) =>  {
    return {
        name: 'vuepress-theme-local',
        extends: '@vuepress/theme-default',
        layouts: {
            Layout: path.resolve(__dirname, 'layouts/Layout.vue'),
        },
    }
}
