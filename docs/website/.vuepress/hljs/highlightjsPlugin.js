import hljs from 'highlight.js/lib/core';
import toml from 'highlight.js/lib/languages/ini';
import markdown from 'highlight.js/lib/languages/markdown';
import yaml from 'highlight.js/lib/languages/yaml';
import bash from './bash_with_coco.js';
import git_log from './git_log.js';
import conventionalCommit from './conventional_commit_spec.js';
import editor from "./editor.js";
import tera from './tera.js';

hljs.registerLanguage('bash', bash);
hljs.registerLanguage('conventional', conventionalCommit);
hljs.registerLanguage('toml', toml);
hljs.registerLanguage('markdown', markdown);
hljs.registerLanguage('git', git_log);
hljs.registerLanguage('editor', editor);
hljs.registerLanguage('tera', tera);
hljs.registerLanguage('yaml', yaml);

export const highlightjsPlugin = () => ({
    name: '@vuepress/plugin-highlightjs',
    async extendsMarkdown(md) {
        md.options.highlight = (code, lang) => {
            if (lang === "text") {
                return code;
            } else {
                return hljs.highlight(code, {language: lang, ignoreIllegals: true}).value
            }
        }
    },
})
