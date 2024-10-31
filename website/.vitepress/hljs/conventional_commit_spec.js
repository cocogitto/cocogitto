/** @type LanguageFn */
export default function (hljs) {
    const regex = hljs.regex;
    const VAR = {};

    const BRACED_VAR = {
        begin: /\$\{/,
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

    const ARITHMETIC = {
        begin: /\$\(\(/,
        end: /\)\)/,
        contains: [
            {begin: /\d+#[0-9a-f]+/, className: "number"},
            hljs.NUMBER_MODE,
            VAR
        ]
    };

    const KEYWORDS = [
        "message",
    ];

    const OPTIONAL = [
        "optional",
    ];



    const MANDATORY = {
        className: 'string',
        begin: /</, end: />/,
        contains: [
            hljs.BACKSLASH_ESCAPE,
            VAR,
        ]
    };

    return {
        name: 'Conventional commit',
        aliases: ['conventional_commit'],
        keywords: {
            keyword: KEYWORDS,
            literal: OPTIONAL,
            built_in: []
        },
        contains: [
            MANDATORY,
            ARITHMETIC,
            VAR
        ]
    };
}
