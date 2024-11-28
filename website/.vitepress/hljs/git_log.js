/** @type LanguageFn */
export default function (hljs) {
    const regex = hljs.regex;
    const VAR = {};
    const BRACED_VAR = {
        begin: /\$\</,
        end: /\}/,
        contains: [
            "self",
            {
                begin: /:-/,
                contains: [VAR]
            } // default values
        ]
    };

    Object.assign(VAR, {
        className: 'variable',
        variants: [
            {
                begin: regex.concat(/\$[\w\d#@][\w\d_]*/,
                    // negative look-ahead tries to avoid matching patterns that are not
                    // Perl at all like $ident$, @ident@, etc.
                    `(?![\\w\\d])(?![$])`)
            },
            BRACED_VAR
        ]
    });

    const KEYWORDS = [
        "HEAD -> master",
    ];

    const TAG = {
        className: 'string',
        begin: /\(/, end: /\)/,
        contains: [
            hljs.BACKSLASH_ESCAPE,
            VAR,
        ]
    };

    const COMMIT_HASH = {
        begin: /\*/,
        end: /-/,
        contains: [
            "self",
            {
                className: 'title.function.invoke',
                begin: /\s/,
                end: /\s/,
            }
        ]
    };

    const COMMIT_TYPE = {
        className: 'keyword',
        begin: /\s(?:feat|fix|chore|docs|test|perf|ci|style)!?:/, end: /\s/,
        contains: [
            hljs.BACKSLASH_ESCAPE,
            VAR,
        ]
    };

    const AUTHOR = {
        begin: /</,
        end: />/,
        className: 'meta',
        contains: [
            hljs.BACKSLASH_ESCAPE,
            VAR,
        ]
    };



    return {
        name: 'Git log',
        aliases: ['git_log'],
        keywords: {
            keyword: KEYWORDS,
            built_in: []
        },
        contains: [
            COMMIT_TYPE,
            COMMIT_HASH,
            AUTHOR,
            TAG,
            VAR
        ]
    };
}
