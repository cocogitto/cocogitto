/** @type LanguageFn */
export default function (hljs) {
    const BLOCK = {
        begin: '{%', end: '%}',
        contains: [
            {begin: '(?:for|endfor)', end: ' ', className: 'meta'},
            {begin: '(?:if|endif|elif|else)', end: ' ', className: 'number'},
            {begin: 'set', end: ' ', className: 'built_in'},
            {begin: '\"', end: '\"', className: 'string'},
        ]
    };

    const VAR = {
        begin: '{{',
        end: '}}',
        className: 'keyword'
    }

    return {
        name: 'tera',
        aliases: ['tera'],
        contains: [
            BLOCK,
            VAR
        ]
    };
}
