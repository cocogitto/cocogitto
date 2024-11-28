/** @type LanguageFn */
export default function(hljs) {
    const regex = hljs.regex;
    const VAR = {};
    const BRACED_VAR = {
        begin: /\$\{/,
        end:/\}/,
        contains: [
            "self",
            {
                begin: /:-/,
                contains: [ VAR ]
            } // default values
        ]
    };
    Object.assign(VAR,{
        className: 'variable',
        variants: [
            {begin: regex.concat(/\$[\w\d#@][\w\d_]*/,
                    // negative look-ahead tries to avoid matching patterns that are not
                    // Perl at all like $ident$, @ident@, etc.
                    `(?![\\w\\d])(?![$])`) },
            BRACED_VAR
        ]
    });

    const SUBST = {
        className: 'subst',
        begin: /\$\(/, end: /\)/,
        contains: [hljs.BACKSLASH_ESCAPE]
    };
    const HERE_DOC = {
        begin: /<<-?\s*(?=\w+)/,
        starts: {
            contains: [
                hljs.END_SAME_AS_BEGIN({
                    begin: /(\w+)/,
                    end: /(\w+)/,
                    className: 'string'
                })
            ]
        }
    };
    const QUOTE_STRING = {
        className: 'string',
        begin: /"/, end: /"/,
        contains: [
            hljs.BACKSLASH_ESCAPE,
            VAR,
            SUBST
        ]
    };
    SUBST.contains.push(QUOTE_STRING);
    const ESCAPED_QUOTE = {
        className: '',
        begin: /\\"/

    };
    const APOS_STRING = {
        className: 'string',
        begin: /'/, end: /'/
    };

    const ARITHMETIC = {
        begin: /\$\(\(/,
        end: /\)\)/,
        contains: [
            { begin: /\d+#[0-9a-f]+/, className: "number" },
            hljs.NUMBER_MODE,
            VAR
        ]
    };

    // to consume paths to prevent keyword matches inside them
    const PATH_MODE = {
        match: /(\/[a-z._-]+)+/
    };

    const COMMANDS = [
        "cargo",
        "yay",
        "coco",
        "cog",
        "git",
        "mkdir",
        "docker",
        "pacman",
        "xbps-install",
        "brew",
        "nix-env",
        "choco",
    ];

    return {
        name: 'Bash',
        aliases: ['sh'],
        keywords: {
            $pattern: /\b[a-z._-]+\b/,
            built_in:[
                ...COMMANDS
            ]
        },
        contains: [
            ARITHMETIC,
            hljs.HASH_COMMENT_MODE,
            HERE_DOC,
            PATH_MODE,
            QUOTE_STRING,
            ESCAPED_QUOTE,
            APOS_STRING,
            VAR
        ]
    };
}
