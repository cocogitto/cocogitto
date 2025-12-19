# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -

## [6.5.0](https://github.com/cocogitto/cocogitto/compare/6.4.0..6.5.0) - 2025-11-02
#### Features
- (**bump**) add --include-packages option to allow manual bump for all monorepo versions - ([6f61a58](https://github.com/cocogitto/cocogitto/commit/6f61a58f06f57c793df26b5be8d2addecca10748)) - [@ba-lindner](https://github.com/ba-lindner)
- (**get-version**) add options to print full tag and include prerelease versions - ([55fb7f0](https://github.com/cocogitto/cocogitto/commit/55fb7f0dc0d2a8f44c2ca11db90bc1bc0275895b)) - [@ba-lindner](https://github.com/ba-lindner)
- add option to combine package and global changes into one changelog - ([12fe810](https://github.com/cocogitto/cocogitto/commit/12fe810c12ece8507ee0ffb629d960d6dc626308)) - [@ba-lindner](https://github.com/ba-lindner)
- add builtin macros for tera templates - ([cba4101](https://github.com/cocogitto/cocogitto/commit/cba4101b0da710656a9dfdf19f7135a4085ba8e4)) - [@oknozor](https://github.com/oknozor)
#### Bug Fixes
- set filetype for commit edit message - ([9d14c0b](https://github.com/cocogitto/cocogitto/commit/9d14c0b967598780d2acd9e281bcf2ee4d0e9fd7)) - [@oknozor](https://github.com/oknozor)
- correctly handle bumping from prerelease - ([5628b0c](https://github.com/cocogitto/cocogitto/commit/5628b0c0071acba95dbec603d171dc9c92cf5b19)) - [@ba-lindner](https://github.com/ba-lindner)
#### Documentation
- (**bump**) fix monorepo bump docs - ([3cf84ae](https://github.com/cocogitto/cocogitto/commit/3cf84ae84967d1381b1713970cfbfa985b328fc8)) - [@ba-lindner](https://github.com/ba-lindner)
- (**changelog**) update changelog docs to show current template output - ([2ee784e](https://github.com/cocogitto/cocogitto/commit/2ee784ea125c79f429bcc0eed38ba911488e4799)) - [@ba-lindner](https://github.com/ba-lindner)
#### Continuous Integration
- add dispatch release to cocogitto-action - ([9330f2b](https://github.com/cocogitto/cocogitto/commit/9330f2b0241746e425724cf4cc76675ce7a5104c)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- (**changelog**) add tera extension to template files - ([075d5a1](https://github.com/cocogitto/cocogitto/commit/075d5a1e49d71467ddf4bf5c9694b555b9f63a53)) - [@oknozor](https://github.com/oknozor)
- unify getting next version for bump - ([fa1dcce](https://github.com/cocogitto/cocogitto/commit/fa1dcce47c4fd5e50946ff63913d281e77be6d93)) - [@ba-lindner](https://github.com/ba-lindner)

- - -

## [6.4.0](https://github.com/cocogitto/cocogitto/compare/6.3.0..6.4.0) - 2025-10-18
#### Features
- add breaking changes badge in all built-in templates - ([1b7f965](https://github.com/cocogitto/cocogitto/commit/1b7f96576e98e89838ba7608904fad65983a9e9d)) - [@oknozor](https://github.com/oknozor)
- apply sort order in template - ([306da76](https://github.com/cocogitto/cocogitto/commit/306da76e540b6bc6736af11753b916806cd8c9e0)) - [@oknozor](https://github.com/oknozor)
- Allow sort order to be specified for commit types - ([1949b50](https://github.com/cocogitto/cocogitto/commit/1949b505ba47367145f8f55529c9d798e0745568)) - mikebender
- only read tags reachable from HEAD - ([aecd76b](https://github.com/cocogitto/cocogitto/commit/aecd76bd47a31644bb89158aa17f7246a8740765)) - [@ba-lindner](https://github.com/ba-lindner)
- allow specifying config path via command line (#466) - ([b5443e8](https://github.com/cocogitto/cocogitto/commit/b5443e8d8da0a4674ababbc0e031aedf0b795a85)) - Technofab
- resolve tilde in the signingkey to home (#460) - ([daa983b](https://github.com/cocogitto/cocogitto/commit/daa983b1b0507a326099fcbe50b1a80bdab8a296)) - Kristof Mattei
#### Bug Fixes
- respect disable_changelog setting for package changelogs - ([d6ac8bf](https://github.com/cocogitto/cocogitto/commit/d6ac8bf253b1f3dc6082af91c82039c13d5acc1a)) - [@ba-lindner](https://github.com/ba-lindner)
- allow packages without bump in changelog command - ([71dc621](https://github.com/cocogitto/cocogitto/commit/71dc6218bcba2c96bd622deefdf0a4136aa5c84f)) - [@ba-lindner](https://github.com/ba-lindner)
- don't reverse commit list - ([bec8de8](https://github.com/cocogitto/cocogitto/commit/bec8de8b1ef6618880fc556d7b53083f324a5432)) - [@ba-lindner](https://github.com/ba-lindner)
- save multiple tags per commit - ([cc0e64d](https://github.com/cocogitto/cocogitto/commit/cc0e64d2c1e075ac9b782258783212b4d7917892)) - [@ba-lindner](https://github.com/ba-lindner)
- cog changelog failing on single commit - ([a155a9d](https://github.com/cocogitto/cocogitto/commit/a155a9d3e331517398efefab0dbd0a426ff38f1b)) - [@oknozor](https://github.com/oknozor)
- set explicit binstall pkg-url, bin-dir and pkg-fmt - ([97fe60e](https://github.com/cocogitto/cocogitto/commit/97fe60ed25e6458b842afb52bc1ac30fdcef5c11)) - Kristof Mattei
- fix github pages build - ([c8e46aa](https://github.com/cocogitto/cocogitto/commit/c8e46aa2b28ff1bad94852b1abbec568adbff5aa)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- (**typo**) fix various doc typo (#458) - ([707a80a](https://github.com/cocogitto/cocogitto/commit/707a80acafac209d20e20fef19ecb3b539b791ef)) - Jx
- add commit order doc - ([7e4e6ef](https://github.com/cocogitto/cocogitto/commit/7e4e6ef80503f71748e369e8b92e4c8c135ad68a)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- remove field target of Tag - ([3a4472b](https://github.com/cocogitto/cocogitto/commit/3a4472beacf7679b6cbd6a8903693822b27297ea)) - [@ba-lindner](https://github.com/ba-lindner)

- - -

## [6.3.0](https://github.com/cocogitto/cocogitto/compare/6.2.0..6.3.0) - 2025-03-19
#### Features
- (**check**) Support commit scope checking - ([77f2acb](https://github.com/cocogitto/cocogitto/commit/77f2acb46efa55583ea2258099aa2bd6fd93b7f8)) - Thomas Böhler
- (**commit**) support custom hooksPath in gitconfig (#448) - ([f003e78](https://github.com/cocogitto/cocogitto/commit/f003e7853fd212f264af2fc2d024e189b7fce301)) - [@oknozor](https://github.com/oknozor)
- allow to partially override default commit type - ([7f59acc](https://github.com/cocogitto/cocogitto/commit/7f59acc161553b26a5429308027b5ca8c178c576)) - [@oknozor](https://github.com/oknozor)
- add bump order config for packages (#443) - ([e888f52](https://github.com/cocogitto/cocogitto/commit/e888f524f1e8c4456d0e2c1a5b77a4950a7f8773)) - [@oknozor](https://github.com/oknozor)
- ignore fixup commits - ([b246352](https://github.com/cocogitto/cocogitto/commit/b24635257988078e9a9a896498b4fd12506588d9)) - [@oknozor](https://github.com/oknozor)
#### Bug Fixes
- (**commit**) fixed order of commit type options (#431) - ([53c6f5c](https://github.com/cocogitto/cocogitto/commit/53c6f5ce46fec865ccc5858d2c1cff2d2b6755aa)) - Sertonix
- (**githooks**) direct exec with shebang or fallback to sh - ([7f3ab80](https://github.com/cocogitto/cocogitto/commit/7f3ab809fb102022953e316f466a21ae975af467)) - [@oknozor](https://github.com/oknozor)
- fix ssh signing - ([30ef4c5](https://github.com/cocogitto/cocogitto/commit/30ef4c5c62cc1942aec3f5c8034dd9bc85151ded)) - [@oknozor](https://github.com/oknozor)
- fixes broken link in project README - ([e8ea261](https://github.com/cocogitto/cocogitto/commit/e8ea2618353b77386bf2369facd25c0f915efb85)) - affantaufiqur
- set correct url for Getting Started - ([f2e2ee8](https://github.com/cocogitto/cocogitto/commit/f2e2ee82ab8e11f092d164688c1061ce3aeea3e9)) - Kristof Mattei
#### Performance Improvements
- use lazy loaded commit types - ([9eb4fc5](https://github.com/cocogitto/cocogitto/commit/9eb4fc5ca85d00b9a9baae1ee1957769f2fce657)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- generated config reference - ([4c2c991](https://github.com/cocogitto/cocogitto/commit/4c2c991fe7d4cb886f993793aaafb07fcc5890bf)) - [@oknozor](https://github.com/oknozor)
- fix capitalisation of NixOS - ([0683402](https://github.com/cocogitto/cocogitto/commit/06834021f50bb64b7507d5bdc595698e0073d5ff)) - txk2048
#### Continuous Integration
- fix changelog ommission until next release - ([2522799](https://github.com/cocogitto/cocogitto/commit/252279957b92116849542cbba5d01f1a0751512c)) - [@oknozor](https://github.com/oknozor)
- bump download artifact action - ([cff93d3](https://github.com/cocogitto/cocogitto/commit/cff93d365fdfec4faa52d9c9694e5cb5b4008ed7)) - [@oknozor](https://github.com/oknozor)
- fix bump deprecated upload-artifact action - ([9f3fee5](https://github.com/cocogitto/cocogitto/commit/9f3fee5a97b423d59027972b361bef11bb74128f)) - [@oknozor](https://github.com/oknozor)

- - -

## [6.2.0](https://github.com/cocogitto/cocogitto/compare/6.1.0..6.2.0) - 2024-11-28
#### Features
- (**settings**) allow to disable default commit types - ([b84247f](https://github.com/cocogitto/cocogitto/commit/b84247fd39c2bb1aff685293fe32cc4d43dfa090)) - [@oknozor](https://github.com/oknozor)
- allow to disable package tagging for monorepos - ([426223e](https://github.com/cocogitto/cocogitto/commit/426223e807e0e653afc85cde498a5db976babfa9)) - [@oknozor](https://github.com/oknozor)
- open repo at path - ([f496807](https://github.com/cocogitto/cocogitto/commit/f4968079684972c9d616f29e0cfa4aeb2d56b344)) - Finley Thomalla
- Add optional default version definition in the VersionDSL, the separator is a `|`, the default version is defined in a way that allows us to apply modifiers to it when a version is not available. The same applies for tags (we use the tag prefix defined in the settings). - ([07ae2ac](https://github.com/cocogitto/cocogitto/commit/07ae2acca7cca729693c3c56515f77f79865366c)) - Matjaz Domen Pecan
#### Bug Fixes
- fix ignore merge commit no longer honored - ([112bfcc](https://github.com/cocogitto/cocogitto/commit/112bfcc4a014b05dc43712cd4cb32846e5923251)) - [@oknozor](https://github.com/oknozor)
- populate monorepo changelog tera context - ([3875f65](https://github.com/cocogitto/cocogitto/commit/3875f65918f2710b335a872a433b5cef0a7c82b6)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- (**config**) add link to cog.toml config reference to README and cog.toml - ([ab1fccb](https://github.com/cocogitto/cocogitto/commit/ab1fccba029aa0c05345cb38c98a99c01a09e8ea)) - Emily Zall
- (**website**) clarify which commit types auto bump by default and how to change this config - ([13ab5a3](https://github.com/cocogitto/cocogitto/commit/13ab5a3e4112efb4705d7b82da9b95bba894a179)) - Emily Zall
- (**website**) fix some typos - ([7096430](https://github.com/cocogitto/cocogitto/commit/7096430d1852cecf8e85d842971563cb71dc8b2f)) - mroetsc
- (**website**) include MacOS install instructions on homepage - ([7f6b5f1](https://github.com/cocogitto/cocogitto/commit/7f6b5f1d3cb50db10b89b77c19a45ee2611df903)) - Ali Dowair
- migrate to vitepress - ([3ff98b2](https://github.com/cocogitto/cocogitto/commit/3ff98b20eeeb4e8b44000e23ddb77de53902f477)) - [@oknozor](https://github.com/oknozor)
- update tera link in README.md - ([24dd3da](https://github.com/cocogitto/cocogitto/commit/24dd3da91ee0427d7cc5ecfe36fff3cc0521566b)) - David LJ
- correct minor typos (#396) - ([72e1f86](https://github.com/cocogitto/cocogitto/commit/72e1f8624939a089414aa28418bd2249972fac23)) - tjmurray
#### Continuous Integration
- fix codedov upload - ([6f3a292](https://github.com/cocogitto/cocogitto/commit/6f3a292c7c52144c225c4901c01d5c1020a1f112)) - [@oknozor](https://github.com/oknozor)

- - -

## [6.1.0](https://github.com/cocogitto/cocogitto/compare/6.0.1..6.1.0) - 2024-03-14
#### Features
- (**bump**) disable bump commit - ([e6b5468](https://github.com/cocogitto/cocogitto/commit/e6b5468c698ab15db6129a3edb7c8ec48895cdcc)) - ABWassim
- (**commit**) add gitsign support - ([56a8f32](https://github.com/cocogitto/cocogitto/commit/56a8f32f2591d0b6985fb8173c8305d9883e5579)) - [@oknozor](https://github.com/oknozor)
- (**commit**) add and update files - ([0666ffe](https://github.com/cocogitto/cocogitto/commit/0666ffed9c93942751258622c4fbf08ba2e779c2)) - ABWassim
- add build version to command - ([1680042](https://github.com/cocogitto/cocogitto/commit/1680042edd9f811fee2f6232a7573e456d3a65b9)) - David Arnold
- ssh signing for commits - ([3cd580e](https://github.com/cocogitto/cocogitto/commit/3cd580e6b6cfb500e1d14fa84486f26d0b0523db)) - [@DaRacci](https://github.com/DaRacci)
- complete rework of revspec and revwalk - ([6a3b2db](https://github.com/cocogitto/cocogitto/commit/6a3b2dbabeaf243050902628810e684ff278590d)) - [@oknozor](https://github.com/oknozor)
- add additional package path filtering options - ([dde8ffe](https://github.com/cocogitto/cocogitto/commit/dde8ffe38ac4b3395e55f542ff7d1d57f1f89a43)) - Greg Fairbanks
#### Bug Fixes
- (**bump**) use commit meta to determine no bump commits - ([f9d3dd3](https://github.com/cocogitto/cocogitto/commit/f9d3dd35a1adeffc465b5aa7e3645bdbcdff7bbf)) - Maksym Kondratenko
#### Performance Improvements
- (**changelog**) build cache once - ([1ca130f](https://github.com/cocogitto/cocogitto/commit/1ca130f8e0c034f1cadc18bedffe48c23779886c)) - [@oknozor](https://github.com/oknozor)
- (**revwalk**) cache every tags and ref - ([99eef16](https://github.com/cocogitto/cocogitto/commit/99eef1603c58b5d15a7be24e484838d182d13431)) - [@oknozor](https://github.com/oknozor)
- add flamegraph and massif scripts to justfile - ([3f1d2cf](https://github.com/cocogitto/cocogitto/commit/3f1d2cf6ae5c91de363c2bacc5a5e4a0dcf4aea1)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- (**README**) fix typo - ([64fc19c](https://github.com/cocogitto/cocogitto/commit/64fc19cef4e3625d7dc15f4104871a3813002ae5)) - Oluf Lorenzen
- (**bump**) disable bump commit - ([12df7a2](https://github.com/cocogitto/cocogitto/commit/12df7a23d6bea841d00314f7c4fa07ad1e2c6f57)) - ABWassim
- (**commits-types**) bump minor and patch options - ([dd5517b](https://github.com/cocogitto/cocogitto/commit/dd5517b4a1d079c38565ac7b3c223ea25d9a8310)) - ABWassim
- update docs with semver build meta - ([aec74df](https://github.com/cocogitto/cocogitto/commit/aec74df67038732c872c45334b38141a0ce6303a)) - David Arnold
#### Continuous Integration
- fixed copy wrong path - ([facdefb](https://github.com/cocogitto/cocogitto/commit/facdefbae670f4b2ca245a1106dc9d07345ac993)) - ABWassim
#### Refactoring
- (**bump**) wrap bump and commit args into dedicated opts structs - ([452097d](https://github.com/cocogitto/cocogitto/commit/452097dd152542e3783a08ecd5ee842a8dcb7810)) - [@oknozor](https://github.com/oknozor)
- try from impl for release - ([06ec52b](https://github.com/cocogitto/cocogitto/commit/06ec52bbc29f08d3937149c522d575d736eec010)) - [@oknozor](https://github.com/oknozor)

- - -

## [6.0.1](https://github.com/cocogitto/cocogitto/compare/6.0.0..6.0.1) - 2023-11-30
#### Bug Fixes
- gh pages build path - ([52b8307](https://github.com/cocogitto/cocogitto/commit/52b8307f23d76cf7d853ff60631e0872a1fb268e)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- fix gh-pages deploy - ([4c79de9](https://github.com/cocogitto/cocogitto/commit/4c79de930fd120e6cee12fa49c3f96e4a52d2faa)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- fix the format of release assets - ([8a1df55](https://github.com/cocogitto/cocogitto/commit/8a1df55ed99d05ca9443a348f446dfdab42fd2fa)) - Shunsuke Suzuki

- - -

## [6.0.0](https://github.com/cocogitto/cocogitto/compare/5.6.0..6.0.0) - 2023-11-30
#### Features
- ![BREAKING](https://img.shields.io/badge/BREAKING-red) (**bump**) skip-ci rework - ([3a42e68](https://github.com/cocogitto/cocogitto/commit/3a42e685d9182bbd3148cca8ab532116da6a12d8)) - ABWassim
- add docker image - ([c28d934](https://github.com/cocogitto/cocogitto/commit/c28d934fd60e75637902e3eb0543d36a88674b42)) - [@oknozor](https://github.com/oknozor)
- allow bumping with other commit types - ([49d586a](https://github.com/cocogitto/cocogitto/commit/49d586ac1d70057ea0a83a124daba4f6d8b6e1ff)) - [@oknozor](https://github.com/oknozor)
- version access substitution - ([3346a84](https://github.com/cocogitto/cocogitto/commit/3346a84b5f2a8dd7b2482a868599a6985ae2635a)) - Ezekiel Warren
- Allow users to disable changelog generation - ([17f98dc](https://github.com/cocogitto/cocogitto/commit/17f98dc3ea2ebf233fb19f8e25700fab3664058f)) - Billie Thompson
- add option to overwrite existing git-hooks - ([ec2c4c4](https://github.com/cocogitto/cocogitto/commit/ec2c4c43870bc81839b02bd581a62010b2dfb55a)) - marcbull
#### Bug Fixes
- shortened commit when parsing into oidof - ([7ed4fa7](https://github.com/cocogitto/cocogitto/commit/7ed4fa738bec4821b8873344b123ca1a70087e9d)) - ABWassim
- correctly handle bump with pre-release tags - ([891d55b](https://github.com/cocogitto/cocogitto/commit/891d55b2293d10d556455ad4cadb75e19b2ac819)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- remove coco deprecation notice - ([6ce82f4](https://github.com/cocogitto/cocogitto/commit/6ce82f42a82273333aa55018f56b8ec896896a96)) - [@oknozor](https://github.com/oknozor)
- add docker to readme and link to github app page - ([6931c79](https://github.com/cocogitto/cocogitto/commit/6931c790641b98b7065fcd71615a3ea2656a5dba)) - [@oknozor](https://github.com/oknozor)
- fix indentation for gh action doc - ([809b386](https://github.com/cocogitto/cocogitto/commit/809b3867f5d7722096620e628473c6f26500bc30)) - [@oknozor](https://github.com/oknozor)
- add docs for the docker image - ([bedfaa8](https://github.com/cocogitto/cocogitto/commit/bedfaa81775dd5b8aae1967adbba6720bed05377)) - [@oknozor](https://github.com/oknozor)
- skip-ci breaking change - ([8e05d9c](https://github.com/cocogitto/cocogitto/commit/8e05d9c01f23e3dbdfa390cea0f7268179c10718)) - ABWassim
- migrate docs to main repo - ([fb73056](https://github.com/cocogitto/cocogitto/commit/fb73056b0968c13773865099f2f744d9336af109)) - [@oknozor](https://github.com/oknozor)
- add installation instructions for MacOS - ([b64ce0f](https://github.com/cocogitto/cocogitto/commit/b64ce0f6b68bbcf3ee4f5b6acfdf71d037660f65)) - Gianluca Recchia
#### Continuous Integration
- check tags for release job instead of steps - ([fb23340](https://github.com/cocogitto/cocogitto/commit/fb233402864134304e1ef4c730638062c7125895)) - [@oknozor](https://github.com/oknozor)
- update docker image and gh release - ([377c2c2](https://github.com/cocogitto/cocogitto/commit/377c2c2481c4403657f825d3c4934d5a8d614098)) - [@oknozor](https://github.com/oknozor)
- docker build - ([b1ca37b](https://github.com/cocogitto/cocogitto/commit/b1ca37bbefdba3eaefa7a9139ef3fe4b4ac07f85)) - [@oknozor](https://github.com/oknozor)
- add docker multiarch build - ([9fea07f](https://github.com/cocogitto/cocogitto/commit/9fea07fadd47874b4d8f0ae938b6473daeb5670b)) - [@oknozor](https://github.com/oknozor)
- run tests on all platforms and add a windows binary to release - ([5987707](https://github.com/cocogitto/cocogitto/commit/59877075e0d90e9e36c5e1b301f00fd7507641d8)) - [@oknozor](https://github.com/oknozor)

- - -

## [5.6.0](https://github.com/cocogitto/cocogitto/compare/5.5.0..5.6.0) - 2023-09-27
#### Features
- (**bump**) added skip untracked as cli argument - ([fb6a7e6](https://github.com/cocogitto/cocogitto/commit/fb6a7e667cdcddc185e02894b03b3b623610ca77)) - [@Wassim-AB](https://github.com/Wassim-AB)
#### Bug Fixes
- (**bump**) option to disable untracked changes error - ([da459de](https://github.com/cocogitto/cocogitto/commit/da459decff9a84bb5bde471f5d64f577df28df11)) - [@Wassim-AB](https://github.com/Wassim-AB)
#### Documentation
- fix discord invite link - ([5d11d5a](https://github.com/cocogitto/cocogitto/commit/5d11d5aec58b4d88b2606c3f8b2ca4e7688590c2)) - [@oknozor](https://github.com/oknozor)

- - -

## [5.5.0](https://github.com/cocogitto/cocogitto/compare/5.4.0..5.5.0) - 2023-08-17
#### Features
- (**bump**) option to specify skip-ci pattern for bump commit - ([92736aa](https://github.com/cocogitto/cocogitto/commit/92736aaea536916b0650c6a26176bb1e23b7fa18)) - [@Wassim-AB](https://github.com/Wassim-AB)
- implement `TryFrom` support for `Settings` for `String` and `&Repository` - ([b2d36aa](https://github.com/cocogitto/cocogitto/commit/b2d36aa39d37bf6491fe3074ada401480d7832a6)) - Mark S
#### Documentation
- (**README**) fix links to cocogitto docs - ([d62a83c](https://github.com/cocogitto/cocogitto/commit/d62a83c5613dc51a5b8201a0d2747493f148a758)) - [@lukehsiao](https://github.com/lukehsiao)
- update cog.toml example and contributing guidelines - ([ee09f87](https://github.com/cocogitto/cocogitto/commit/ee09f87c8b76b616279894d75c88b3400cf87762)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- (**revspec**) use git describe for finding latest tag - ([9a49999](https://github.com/cocogitto/cocogitto/commit/9a499990c66b4472d14a4a1e2f1ba9b6131d36ef)) - [@lukehsiao](https://github.com/lukehsiao)
- (**revspec**) raise error instead of panicking when parsing `RevspecPattern` - ([71ee6c6](https://github.com/cocogitto/cocogitto/commit/71ee6c67fb170ec535ee18aefd51cf142fc19911)) - [@SanchithHegde](https://github.com/SanchithHegde)
#### Style
- rename 'Cog' to 'cog' internally - ([9b0830e](https://github.com/cocogitto/cocogitto/commit/9b0830ef456a2713e31934a304339d36231b1df0)) - [@tranzystorek-io](https://github.com/tranzystorek-io)

- - -

## [5.4.0](https://github.com/cocogitto/cocogitto/compare/5.3.1..5.4.0) - 2023-06-23
#### Features
- (**bump**) support annotated tags - ([363387d](https://github.com/cocogitto/cocogitto/commit/363387df62050d96bd2988a11e827cb720487395)) - [@bitfehler](https://github.com/bitfehler)
- (**check**) allow running check on commit range - ([5168f75](https://github.com/cocogitto/cocogitto/commit/5168f75323ed14d34f497a7d14c7f2fa71db1693)) - [@SanchithHegde](https://github.com/SanchithHegde)
- (**cli**) add file parameter to verify command - ([5e02aef](https://github.com/cocogitto/cocogitto/commit/5e02aefc6a77302f6bbe505425f731ebbc5214c6)) - sebasnabas
- (**cli**) Added get-version feature (#248) - ([5670cd8](https://github.com/cocogitto/cocogitto/commit/5670cd81a4127f2a125fde4b4eb9d38da3e121ed)) - [@andre161292](https://github.com/andre161292)
- (**commit**) execute commit hooks when running `cog commit` - ([bf38fa6](https://github.com/cocogitto/cocogitto/commit/bf38fa6454d038c4d175943f7c47aeea2a433ec8)) - [@oknozor](https://github.com/oknozor)
- (**monorepo**) add config opt to disable global global-tag-opt (#264) - ([96aa3b6](https://github.com/cocogitto/cocogitto/commit/96aa3b678845548cb50ef56c40948a86450d0ad0)) - [@oknozor](https://github.com/oknozor)
- Add configurable changelog omission for custom commit types - ([88f8742](https://github.com/cocogitto/cocogitto/commit/88f874220e058709ad4a1f2f2c35eb4a0d67dc4c)) - Mark S
- add custom git-hook installation - ([39cba74](https://github.com/cocogitto/cocogitto/commit/39cba74ba21c9679a03667e0fa8cbc6057bdf967)) - [@oknozor](https://github.com/oknozor)
- reorganize manpages generation - ([1509583](https://github.com/cocogitto/cocogitto/commit/1509583ca058d8e43e4d02e603d10d13248723b0)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- add {{package}} to hook version dsl - ([af08a7e](https://github.com/cocogitto/cocogitto/commit/af08a7e86c714fcdd0977d283d4887f3e98b6aa2)) - [@oknozor](https://github.com/oknozor)
- add version_tag and latest_tag to hook version dsl - ([9eaee5a](https://github.com/cocogitto/cocogitto/commit/9eaee5abcc16bd8fa06c50b83e1892dde126f38f)) - [@oknozor](https://github.com/oknozor)
#### Bug Fixes
- (**bump**) bump don't throw error on no bump types commits - ([4a6a8b3](https://github.com/cocogitto/cocogitto/commit/4a6a8b30aa4e9eddfe1b0a3fe0c7fd822e767447)) - [@Wassim-AB](https://github.com/Wassim-AB)
- (**monorepo**) incorrect increment method used for package bumps - ([7bb3229](https://github.com/cocogitto/cocogitto/commit/7bb3229e356692dbf9eb932fdb5f42840414562e)) - [@oknozor](https://github.com/oknozor)
#### Revert
- (**partial**) revert addition of 'no_coverage' support and attributes - ([93c9903](https://github.com/cocogitto/cocogitto/commit/93c990322cccb71f27cf97d6c6bfac70cfca613c)) - Mark S
#### Documentation
- Update manpage generation docs - ([4a09837](https://github.com/cocogitto/cocogitto/commit/4a09837244e070ff6168cd247ed5621b41f4264e)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Continuous Integration
- (**formatting**) Apply cargo fmt - ([2710183](https://github.com/cocogitto/cocogitto/commit/2710183246d9f4bd43e3e6fa989f20f12cd57801)) - Mark S
- (**tests**) Add `no_coverage` support when using `llvm-cov` on nightly - ([97fd420](https://github.com/cocogitto/cocogitto/commit/97fd4208fc1dc483decfec6bc3ace85dc954c30b)) - Mark S
- remove android targets - ([1197d5f](https://github.com/cocogitto/cocogitto/commit/1197d5f98dc5e99f9bff82552b8dc813ab46ec33)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- (**cli**) adjust cog-verify args - ([8a12065](https://github.com/cocogitto/cocogitto/commit/8a120650dc3aeacf1c55e11429592074f19174ec)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Style
- remove unused `CommitConfig::{omit,include}` methods - ([3ad69eb](https://github.com/cocogitto/cocogitto/commit/3ad69ebd487968e17822442bb171476766184dff)) - Mark S

- - -

## [5.3.1](https://github.com/cocogitto/cocogitto/compare/5.3.0..5.3.1) - 2023-01-23
#### Bug Fixes
- (**monorepo**) fix package tag parsing - ([cdff4a1](https://github.com/cocogitto/cocogitto/commit/cdff4a10332fb4caf4299f5f368e5a794f862228)) - [@oknozor](https://github.com/oknozor)
- (**monorepo**) allow diffing orphan commits - ([02a1c5a](https://github.com/cocogitto/cocogitto/commit/02a1c5af617795f7da6a087d4373840f7bbba190)) - [@oknozor](https://github.com/oknozor)
- disable help subcommand for `cog(1)` - ([64df65b](https://github.com/cocogitto/cocogitto/commit/64df65b3399a890f2e598c046ccc729b37da301b)) - Orhun Parmaksız

- - -

## [5.3.0](https://github.com/cocogitto/cocogitto/compare/5.2.0..5.3.0) - 2023-01-22
#### Features
- (**cli**) add subcommand for generating manpages - ([fe6bcfe](https://github.com/cocogitto/cocogitto/commit/fe6bcfe578c00be0ded6463c96c54e7392ee1635)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- monorepo support (#240) - ([d8eed3d](https://github.com/cocogitto/cocogitto/commit/d8eed3dc7eac644bf56dd1c4bd4dea0bb1d4f746)) - [@oknozor](https://github.com/oknozor)
- NuShell completions - ([3a356cc](https://github.com/cocogitto/cocogitto/commit/3a356ccb0e5838e5fe09fa56a18842fb417eddd7)) - [@DaRacci](https://github.com/DaRacci)
- add from_latest_tag to settings - ([a154782](https://github.com/cocogitto/cocogitto/commit/a154782639c899fc585b68ea28ce00d2f4bfefb9)) - [@stephenc](https://github.com/stephenc)
#### Bug Fixes
- ignore merge commits based on parent counts - ([f8b5da6](https://github.com/cocogitto/cocogitto/commit/f8b5da64343e60ce5851e2e25b93611bfcf4db05)) - [@oknozor](https://github.com/oknozor)
- signing for chore commits - ([18b9643](https://github.com/cocogitto/cocogitto/commit/18b9643318cad85bae56a4217b04e3e684650d09)) - DaRacci
#### Documentation
- update 'pulic_api' doc - ([c9d5cbf](https://github.com/cocogitto/cocogitto/commit/c9d5cbf6207a80918c013b836a23d008fc7d8a41)) - [@oknozor](https://github.com/oknozor)
- add cargo-smart-release to the list of similar projects - ([3a04e72](https://github.com/cocogitto/cocogitto/commit/3a04e72231626af9717d4e8476e7e5af9c58a8ce)) - [@oknozor](https://github.com/oknozor)
- report one binary following `coco` deprecation - ([3ad6d28](https://github.com/cocogitto/cocogitto/commit/3ad6d28d2909f7a51ca384408550776a5e9b50be)) - [@lucatrv](https://github.com/lucatrv)
#### Refactoring
- simplify project structure for binaries - ([941fb10](https://github.com/cocogitto/cocogitto/commit/941fb103453e311fe7a51f5cf9573e73e6ff1e80)) - [@tranzystorek-io](https://github.com/tranzystorek-io)

- - -

## [5.2.0](https://github.com/cocogitto/cocogitto/compare/5.1.0..5.2.0) - 2022-09-09
#### Features
- (**commit**) implement signed commit - ([9b7c2d7](https://github.com/cocogitto/cocogitto/commit/9b7c2d73b3513d83feca7b37fd96314430d41882)) - [@oknozor](https://github.com/oknozor)
#### Bug Fixes
- fix stackover flow on 'oid..' resvpec - ([d4c3def](https://github.com/cocogitto/cocogitto/commit/d4c3defc6c6fbc4141edb3d1dda18be6b1f195c3)) - [@oknozor](https://github.com/oknozor)
- do not exit on parent not found (all_commits) - ([476d0d6](https://github.com/cocogitto/cocogitto/commit/476d0d6e98734878f9e1a8613eab8da1c5385490)) - Benoît CORTIER

- - -

## [5.1.0](https://github.com/cocogitto/cocogitto/compare/5.0.1..5.1.0) - 2022-04-13
#### Features
- add a test for dry_run option - ([c9437bb](https://github.com/cocogitto/cocogitto/commit/c9437bb2d45daac34b2ea1c9f5157cf9b4997f34)) - [@darlaam](https://github.com/darlaam)
- use stderr log to differentiate app output (for dry_run, for example) and app infos - ([ad46106](https://github.com/cocogitto/cocogitto/commit/ad4610638952db8a09faaf9ba409854aab19c474)) - [@darlaam](https://github.com/darlaam)
- Add dry run for cog bump - ([6524f43](https://github.com/cocogitto/cocogitto/commit/6524f430147ae99fe7b71b9f34aaee4475d2788f)) - [@darlaam](https://github.com/darlaam)
#### Bug Fixes
- (**verify**) remove trailing whitespace on cog verify messages - ([9a42c05](https://github.com/cocogitto/cocogitto/commit/9a42c056b6cdbe70e9645d0bbe0054623d9bf511)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- update cli generated help - ([d9773ba](https://github.com/cocogitto/cocogitto/commit/d9773baee55860817d7c02a1dafe27e408e9bcfd)) - [@oknozor](https://github.com/oknozor)
- add install instructions for Void Linux - ([2e4ffd8](https://github.com/cocogitto/cocogitto/commit/2e4ffd8ce579ecea3622d8efb595d33632563782)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Continuous Integration
- switch to stable channel for code coverage - ([60af851](https://github.com/cocogitto/cocogitto/commit/60af85103066f4004888fa67cbb24c6473b52ebf)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- (**changelog**) use tera whitespace escapes instead of antislash - ([54b3d95](https://github.com/cocogitto/cocogitto/commit/54b3d957342f4c624fab116c61d8629beb304eee)) - [@oknozor](https://github.com/oknozor)

- - -

## [5.0.1](https://github.com/cocogitto/cocogitto/compare/5.0.0..5.0.1) - 2022-03-29
#### Bug Fixes
- (**hook**) correctly escape hook commands by removing shell-word usage - ([aaa9e78](https://github.com/cocogitto/cocogitto/commit/aaa9e78723f6a5f0ae7437258405d65cfb8827e2)) - [@oknozor](https://github.com/oknozor)
- avoid using the format ident capturing Rust feature - ([4c8c8ca](https://github.com/cocogitto/cocogitto/commit/4c8c8cae1378a2de7e4386ad9d03f194e011d4ac)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Documentation
- add a contribution guide - ([905d0a3](https://github.com/cocogitto/cocogitto/commit/905d0a32c1c504f58106a5d4998e66ad180696a9)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- ignore merge commits in cog checks - ([b6bda19](https://github.com/cocogitto/cocogitto/commit/b6bda199930bdacbefcc4e66defe27da1c121438)) - [@oknozor](https://github.com/oknozor)
- update cocogitto action and add branch whitelist to cog.toml - ([45f93c2](https://github.com/cocogitto/cocogitto/commit/45f93c2a753dad8ba2007a3b61e81689e1ad4b9c)) - [@oknozor](https://github.com/oknozor)

- - -

## [5.0.0](https://github.com/cocogitto/cocogitto/compare/4.1.0..5.0.0) - 2022-03-09
#### Features
- (**bump**) use glob pattern for branch whitelist - ([654baa9](https://github.com/cocogitto/cocogitto/commit/654baa959257add03d292bb5fd12ab14d3704076)) - [@oknozor](https://github.com/oknozor)
- (**hook**) use a subshell by default on linux - ([fe47333](https://github.com/cocogitto/cocogitto/commit/fe47333de7c63af966e24ea3d0c9c199573008cf)) - [@oknozor](https://github.com/oknozor)
- (**verify**) add the ability to forbid merge commit via configuration - ([d932d91](https://github.com/cocogitto/cocogitto/commit/d932d91d786708069286bd2c7cc81c7d2a366185)) - [@oknozor](https://github.com/oknozor)
- add branch whitelist to cog bump #165 - ([d78071a](https://github.com/cocogitto/cocogitto/commit/d78071a190b8f85e290f02904d19a71a4432197d)) - [@oknozor](https://github.com/oknozor)
#### Bug Fixes
- (**changelog**) allow template context to be used in custom templates - ([974587c](https://github.com/cocogitto/cocogitto/commit/974587c9b3d41e9ff57c9ff271c6d3d35bc0f962)) - [@oknozor](https://github.com/oknozor)
- (**error**) restore parse error formatting - ([71bec5d](https://github.com/cocogitto/cocogitto/commit/71bec5d8dc3b664475f7c2cd619630a7f0b90f66)) - [@oknozor](https://github.com/oknozor)
- updated config to 0.12.0 (latest as of now) + usage - ([fc4d1a3](https://github.com/cocogitto/cocogitto/commit/fc4d1a3dfdedf55ec65c26f74fada8869700bfd2)) - Kristof Mattei
- check allowed commit type when using 'cog verify' - ([a57de18](https://github.com/cocogitto/cocogitto/commit/a57de18b25a8be350db55304d226ca8d68771527)) - [@oknozor](https://github.com/oknozor)
- make env var error more explicit - ([22120dc](https://github.com/cocogitto/cocogitto/commit/22120dcbfa35144e50999500940ddaacba68086e)) - [@oknozor](https://github.com/oknozor)
- build commit type error regardless of the command used - ([3685651](https://github.com/cocogitto/cocogitto/commit/36856519b432fcd523294c3b1dea8e19e07c493e)) - [@oknozor](https://github.com/oknozor)
- add missing ';' in `run_cmd!` calls - ([ce6b70f](https://github.com/cocogitto/cocogitto/commit/ce6b70f83e0f352b5f25721de5a5d53cbdf100c3)) - [@Zshoham](https://github.com/Zshoham)
- correctly identify empty repository when commiting - ([c442f07](https://github.com/cocogitto/cocogitto/commit/c442f0792fc4497cf65b92dad9ca598fead1c0a3)) - [@Zshoham](https://github.com/Zshoham)
#### Documentation
- replace AUR badges with repology in the installation section - ([466dffe](https://github.com/cocogitto/cocogitto/commit/466dffeb187fd117e423559974a914e991f894e8)) - [@oknozor](https://github.com/oknozor)
- add discord badge to readme - ([e8eef80](https://github.com/cocogitto/cocogitto/commit/e8eef8024c238be5e1873236ea069936ffaf47e1)) - [@oknozor](https://github.com/oknozor)
- use the `cog commit` command - ([ab50816](https://github.com/cocogitto/cocogitto/commit/ab508165533ada56aab79d77da619e901793dc6e)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- add coco deprecation notice to README - ([8e8f7b2](https://github.com/cocogitto/cocogitto/commit/8e8f7b276c2f932f321ff0fba104bbad2598faa7)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- suggest running a locked install when using cargo - ([80a4737](https://github.com/cocogitto/cocogitto/commit/80a4737359d0452e6dcc24331c775efab2ac0c12)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- nixos install onliner - ([22abfd3](https://github.com/cocogitto/cocogitto/commit/22abfd3276c91e47dd02d06f6e2b044abbbcc7a8)) - Travis Davis
#### Continuous Integration
- (**codecov**) fix codecov threshhold - ([f992c21](https://github.com/cocogitto/cocogitto/commit/f992c2196dc88fe1f126802bd880cafd3eddda64)) - [@oknozor](https://github.com/oknozor)
- (**coverage**) set coverage target threshold to 1% - ([bf2eb5d](https://github.com/cocogitto/cocogitto/commit/bf2eb5d6f67c67acc69e1e5029e967abe28bc884)) - [@oknozor](https://github.com/oknozor)
- update code coverage action - ([8c13a45](https://github.com/cocogitto/cocogitto/commit/8c13a45f48862d4e224e957168426e55998e2c26)) - [@oknozor](https://github.com/oknozor)
- use keyword with shorter lenght to comply with crates.io rules - ([cd847de](https://github.com/cocogitto/cocogitto/commit/cd847de37cda51950e65d4b4868b8faa6ffcd880)) - [@oknozor](https://github.com/oknozor)
- remove cargo manifest keyword to comply with crates.io max keyword rule - ([896bcb3](https://github.com/cocogitto/cocogitto/commit/896bcb30b93e47b65e4885e5f6e1e748ab9cdfae)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- (**commit**) improve git statuses error output on commit - ([cbdad51](https://github.com/cocogitto/cocogitto/commit/cbdad511a489a2100305f0400ca1b328ba00b94e)) - [@oknozor](https://github.com/oknozor)
- (**error**) display underlying errors when possible - ([41053e2](https://github.com/cocogitto/cocogitto/commit/41053e2732b3e015a86a3a7fdf53032292131365)) - [@oknozor](https://github.com/oknozor)
- (**error**) remove anyhow from private API - ([2346112](https://github.com/cocogitto/cocogitto/commit/23461123e45dc1933ea4472b28a045d1d81ce4dc)) - [@oknozor](https://github.com/oknozor)
#### Miscellaneous Chores
- ![BREAKING](https://img.shields.io/badge/BREAKING-red) remove deprecated coco utility - ([2c10235](https://github.com/cocogitto/cocogitto/commit/2c10235220cdf4f09c3c5a30631143683448ba98)) - [@tranzystorek-io](https://github.com/tranzystorek-io)

- - -

## [4.1.0](https://github.com/cocogitto/cocogitto/compare/4.0.1..4.1.0) - 2022-01-18
#### Features
- (**cli**) add `cog commit` as duplicate of `coco` - ([128b9d0](https://github.com/cocogitto/cocogitto/commit/128b9d0e07d094ae004c0ee75a3f98182aabe983)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Bug Fixes
- (**parser**) bump parser to 0.9.4 to support windows escape sequences in commit footers - ([415ec37](https://github.com/cocogitto/cocogitto/commit/415ec37921500bc375eb27b70722352fcd11d53b)) - [@oknozor](https://github.com/oknozor)
- support annotated tags in cog check -l - ([66faca2](https://github.com/cocogitto/cocogitto/commit/66faca2312282108e50746cf2df1b8831e304122)) - [@lukehsiao](https://github.com/lukehsiao)
- ignore comment lines in cog verify - ([2f25f5e](https://github.com/cocogitto/cocogitto/commit/2f25f5ea6e9b96882a11a271aad20db0ee420ffb)) - [@lukehsiao](https://github.com/lukehsiao)
#### Documentation
- (**readme**) update Arch Linux noting official package - ([79c1608](https://github.com/cocogitto/cocogitto/commit/79c16082407ff076852e3e8c5d06f212a5bae3fb)) - [@alerque](https://github.com/alerque)
- (**readme**) fix language issues and typos - ([9d37de3](https://github.com/cocogitto/cocogitto/commit/9d37de36cabb21f0dbb70bb3106c21f234ce9472)) - Lucas Declercq
- (**readme**) change some url to the org - ([573ef81](https://github.com/cocogitto/cocogitto/commit/573ef81dc6dd56c42ce7fe9e7916ce26324a3feb)) - [@oknozor](https://github.com/oknozor)
- adjust `cog bump` help wording - ([ef032d5](https://github.com/cocogitto/cocogitto/commit/ef032d55d007092655e5368eeecd95a99f2e03dd)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- add crates.io keywords and categories - ([7fcb0bd](https://github.com/cocogitto/cocogitto/commit/7fcb0bdd452880150e920521ab190e7c61835060)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- add cargo-bump and crates.io token - ([17fb92e](https://github.com/cocogitto/cocogitto/commit/17fb92eccde86a749e4df5d92382d92a03acd1e1)) - [@oknozor](https://github.com/oknozor)
- trigger release from github action workflow dispatch - ([ab146d0](https://github.com/cocogitto/cocogitto/commit/ab146d0842d03c90fe8abc82af76f0ecf1b6e441)) - [@oknozor](https://github.com/oknozor)
- fix code coverage breakage with latest rust nightly - ([cd76427](https://github.com/cocogitto/cocogitto/commit/cd7642729c2a56a091dd1497b37707ae8723e236)) - [@oknozor](https://github.com/oknozor)
- remove codevov artifact upload - ([f7386c1](https://github.com/cocogitto/cocogitto/commit/f7386c1f4fd3aafe3d310cf5e11a81a4361c4d70)) - [@oknozor](https://github.com/oknozor)
- add rust-cache action - ([f8dff56](https://github.com/cocogitto/cocogitto/commit/f8dff56213861fb6ea1696268ba3cb70b1602404)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- group `cog bump` flags together - ([337c7cc](https://github.com/cocogitto/cocogitto/commit/337c7ccd15d7e19a34582841dd0c2741403dc863)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- switch arg parsing to clap v3 - ([13ebca5](https://github.com/cocogitto/cocogitto/commit/13ebca567291f3d38d73eaf76d524ba00666b921)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Style
- (**logo**) add the definitive logo and visual identities - ([e9b404e](https://github.com/cocogitto/cocogitto/commit/e9b404eb3f33eb847484aa6e9d5170a7f565f947)) - [@oknozor](https://github.com/oknozor)

- - -

## [4.0.1](https://github.com/cocogitto/cocogitto/compare/4.0.0..4.0.1) - 2021-11-30
#### Bug Fixes
- (**bump**) correctly generate tag with prefix - ([9e8d592](https://github.com/cocogitto/cocogitto/commit/9e8d592d58f0200590ec3ac3d2b5c2b2c1720a06)) - [@oknozor](https://github.com/oknozor)

- - -

## [4.0.0](https://github.com/cocogitto/cocogitto/compare/3.0.0..4.0.0) - 2021-11-30
#### Features
- (**changelog**) populate template with tag oid - ([679928f](https://github.com/cocogitto/cocogitto/commit/679928f4f1913bd073693d9cafb2d53eef5ae418)) - [@oknozor](https://github.com/oknozor)
- (**changelog**) display multiple release changelog on changelog range - ([c6940c5](https://github.com/cocogitto/cocogitto/commit/c6940c524880265b55941fad20aac46aa5b79305)) - [@oknozor](https://github.com/oknozor)
- (**changelog**) add full_hash changelog template - ([10ab5c6](https://github.com/cocogitto/cocogitto/commit/10ab5c6dc0cce8daa37a17672fec0fcf77679ec9)) - [@oknozor](https://github.com/oknozor)
- (**changelog**) add custom template - ([ad2bcd2](https://github.com/cocogitto/cocogitto/commit/ad2bcd2ea3bd1bd1fe936324c1b62d21d005d0ac)) - [@oknozor](https://github.com/oknozor)
- (**changelog**) implement changelog template - #72 - ([56bbff7](https://github.com/cocogitto/cocogitto/commit/56bbff7a2e7fbfe863ddda199f91c770a5597f78)) - [@oknozor](https://github.com/oknozor)
- (**cli**) improve commit format error format - ([78dea00](https://github.com/cocogitto/cocogitto/commit/78dea00088c7e359164f09d22af1d29a8b468e57)) - [@oknozor](https://github.com/oknozor)
- (**coco**) add edit flag for opening message in editor - ([2b62de3](https://github.com/cocogitto/cocogitto/commit/2b62de3d23a5290dcf49571ceeea60509a09e65b)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- (**cog**) add from latest tag flag to cog edit - ([f391df6](https://github.com/cocogitto/cocogitto/commit/f391df65d0e165a742d3fcff68183261cc5f5836)) - [@oknozor](https://github.com/oknozor)
- (**hook**) add bump profiles configuration - ([13eeed9](https://github.com/cocogitto/cocogitto/commit/13eeed983cf2b44ebf686f4b9d7f86626792a1ff)) - [@oknozor](https://github.com/oknozor)
- (**tag**) add configurable tag prefix as described in #122 - ([38f9eab](https://github.com/cocogitto/cocogitto/commit/38f9eab1f772980dc1b81241dfce12b0cd290951)) - [@oknozor](https://github.com/oknozor)
- ![BREAKING](https://img.shields.io/badge/BREAKING-red) use revspec instead of 'from' annd 'to' flag for changelog - ([ce24789](https://github.com/cocogitto/cocogitto/commit/ce247898b99f0b5b5fbf93c235b51543224e3e50)) - [@oknozor](https://github.com/oknozor)
- add get_conventional_message fn to return the prepared message without committing - ([4668622](https://github.com/cocogitto/cocogitto/commit/46686226fc4c950f285c2d4c525a89967326ca63)) - [@its-danny](https://github.com/its-danny)
- improve cli message format and fix #97 - ([d0bb0d4](https://github.com/cocogitto/cocogitto/commit/d0bb0d4e5a36ab19f9f09ab68fdaed336e43ba89)) - [@oknozor](https://github.com/oknozor)
- add {{latest}} tag to hook dsl - ([5eff372](https://github.com/cocogitto/cocogitto/commit/5eff372bf1aaaa0458f2a382abadeeeb560ee6f5)) - [@oknozor](https://github.com/oknozor)
#### Bug Fixes
- (**bump**) fix target changelog tag on bump - ([0618192](https://github.com/cocogitto/cocogitto/commit/0618192b5e2866b7e12ac6d036df700041194506)) - [@oknozor](https://github.com/oknozor)
- (**changelog**) correctly pick tagged HEAD commit - ([9b5a591](https://github.com/cocogitto/cocogitto/commit/9b5a591a9c67a2fbdffdcbeff296932858ba49b3)) - [@oknozor](https://github.com/oknozor)
- (**hook**) use pre-commit instead of prepare-commit-message hook - ([6fe1a27](https://github.com/cocogitto/cocogitto/commit/6fe1a279a45b1f6452a23039dece95cd508b03bd)) - [@oknozor](https://github.com/oknozor)
- (**scope**) add support for multiple version placeholder and buidmetatata in hooks [#117] - ([43eba56](https://github.com/cocogitto/cocogitto/commit/43eba56766e697a0fcdb43e63835bbc608552d22)) - [@oknozor](https://github.com/oknozor)
- change git commit hook type to allow use of '--no-verify' - ([c4516b7](https://github.com/cocogitto/cocogitto/commit/c4516b77d51fce36935e6d0786fc8055557bb423)) - [@oknozor](https://github.com/oknozor)
- make footer serialization and deserialization symmetric - ([04befc1](https://github.com/cocogitto/cocogitto/commit/04befc1969e3700e33a84259a34732773577dcc0)) - [@oknozor](https://github.com/oknozor)
- fix version increment regression #129 - ([4372b57](https://github.com/cocogitto/cocogitto/commit/4372b57a8431c5b0bdacd6aa1e288719f8585dc0)) - [@oknozor](https://github.com/oknozor)
- display parse error corectly on cog verify - ([618499e](https://github.com/cocogitto/cocogitto/commit/618499ef99e1948b60c8fbe5f2a3adbc0089cb92)) - [@oknozor](https://github.com/oknozor)
- display hook-profiles value in cli and safe check commit type - ([fa59679](https://github.com/cocogitto/cocogitto/commit/fa59679e50b6f81e7125e82c2b2351f1a9c2c659)) - [@oknozor](https://github.com/oknozor)
- fix unicode trailing char panic [#101] - ([3de62ba](https://github.com/cocogitto/cocogitto/commit/3de62ba02273e6714e08374575b55788ceb4483e)) - [@oknozor](https://github.com/oknozor)
- fix typo in git hooks error messages - ([6d8bdb5](https://github.com/cocogitto/cocogitto/commit/6d8bdb5f2e6c894e326cc5439f6b5a22997f9d9a)) - [@cpoissonnier](https://github.com/cpoissonnier)
- generate completions without opening a repository - ([eaf63bb](https://github.com/cocogitto/cocogitto/commit/eaf63bb33b94a18bc50cbab9ee5b596ede24f01c)) - [@orhun](https://github.com/orhun)
- remove Cargo.lock from gitignore [#109] - ([ffe7f0d](https://github.com/cocogitto/cocogitto/commit/ffe7f0dc3a15cc52911e572c469827c598ee60b5)) - [@oknozor](https://github.com/oknozor)
#### Performance Improvements
- remove unused default features from the config crate - ([8e069e8](https://github.com/cocogitto/cocogitto/commit/8e069e8dd41499f49e28192262158e637af29cc5)) - [@oknozor](https://github.com/oknozor)
- add binary size optimizations - ([6227ca6](https://github.com/cocogitto/cocogitto/commit/6227ca626ed1f607d3a5215c5677e9fbf772baeb)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- (**readme**) fix pre_bump_hooks example - ([81ad844](https://github.com/cocogitto/cocogitto/commit/81ad8446ac018caff94ef4369c8049b30c012b6e)) - [@its-danny](https://github.com/its-danny)
- add temporary logo - ([99c39a8](https://github.com/cocogitto/cocogitto/commit/99c39a8f85d0db4b7e182b0f5e54b2d33223350b)) - [@oknozor](https://github.com/oknozor)
- update readme links - ([c3a3143](https://github.com/cocogitto/cocogitto/commit/c3a3143a8d3a450aeaf19bc91ea7bc1d2321116c)) - [@oknozor](https://github.com/oknozor)
- update readme - ([a2d9268](https://github.com/cocogitto/cocogitto/commit/a2d9268c8abeeecaf504eb77631356e86659af3a)) - [@oknozor](https://github.com/oknozor)
- document get_conventional_message - ([72b2722](https://github.com/cocogitto/cocogitto/commit/72b27229fb28b4a7a984815c3987df02ccd673fd)) - [@its-danny](https://github.com/its-danny)
- fix typo in README (#126) - ([551dc32](https://github.com/cocogitto/cocogitto/commit/551dc326ee5baf586bd26e49d5f1be35135b7f02)) - Jean-Philippe Bidegain
#### Continuous Integration
- fix prebump hooks ordering - ([ab3f841](https://github.com/cocogitto/cocogitto/commit/ab3f84118b9b2637a0ae5d0725d755575ea8bf06)) - [@oknozor](https://github.com/oknozor)
- update codecov action - ([ae66d91](https://github.com/cocogitto/cocogitto/commit/ae66d91968987b0f6a5f1008e88260388172b703)) - [@oknozor](https://github.com/oknozor)
- move cog check action to CI - ([4616065](https://github.com/cocogitto/cocogitto/commit/4616065360b51f8536f3f238553ba6317e191c3d)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- (**errors**) remove thiserror crate and rework error format - ([5b00f9e](https://github.com/cocogitto/cocogitto/commit/5b00f9ed1ac5d4cd429cb6eb80392cf41d22a6e7)) - [@oknozor](https://github.com/oknozor)
- (**git**) split repository in multiple modules - ([5ce5187](https://github.com/cocogitto/cocogitto/commit/5ce518730c225681a899326e4ceb224e132dd7d7)) - [@oknozor](https://github.com/oknozor)
- (**git-hooks**) rename prepare-commit hook asset - ([a693d6f](https://github.com/cocogitto/cocogitto/commit/a693d6f4a135b38c9b886cf5e4eb177acb153602)) - [@oknozor](https://github.com/oknozor)
- use git2 revspec instead of home made lookup - ([49e79d1](https://github.com/cocogitto/cocogitto/commit/49e79d1180f51f86fab7a60aaf6fdf94fdbd1fd1)) - [@oknozor](https://github.com/oknozor)
- organize imports and dependencies - ([807d033](https://github.com/cocogitto/cocogitto/commit/807d033d9f5f433959ffa1b2575499c894e0e118)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- remove some lazy static constants - ([dd26c84](https://github.com/cocogitto/cocogitto/commit/dd26c84373be21ce676da7dc477bf4cad0043f17)) - [@oknozor](https://github.com/oknozor)
- use matches expression instead of if lets - ([7b7e469](https://github.com/cocogitto/cocogitto/commit/7b7e4695acb5939a6a89843e80d7f569e2f24299)) - [@oknozor](https://github.com/oknozor)
- switch coco arg parsing to structopt - ([5b185d8](https://github.com/cocogitto/cocogitto/commit/5b185d8d0c9accfe056e38f40359dc90cf6f81da)) - [@tranzystorek-io](https://github.com/tranzystorek-io)

- - -

## [3.0.0](https://github.com/cocogitto/cocogitto/compare/2.1.1..3.0.0) - 2021-09-13
#### Features
- ![BREAKING](https://img.shields.io/badge/BREAKING-red) use conventional commit parser instead of custom implementation - ([53f23d9](https://github.com/cocogitto/cocogitto/commit/53f23d9ad21d5d165ed21c60fe18e37ea6e14203)) - [@oknozor](https://github.com/oknozor)
#### Bug Fixes
- validate footers on commit - ([2f95cf8](https://github.com/cocogitto/cocogitto/commit/2f95cf805df2d354785f5f0be426aade3689b44b)) - [@oknozor](https://github.com/oknozor)
- parse commit message before creating a commit - ([4bdcb3d](https://github.com/cocogitto/cocogitto/commit/4bdcb3d7697476dfca40bd9e8eabd6cf3f9adb27)) - [@oknozor](https://github.com/oknozor)
- sort tag names before searching - ([4f5bd95](https://github.com/cocogitto/cocogitto/commit/4f5bd95716e3ccbb05f9c8e313b71d0e99eaaf07)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Continuous Integration
- use cocogitto github action - ([434c222](https://github.com/cocogitto/cocogitto/commit/434c22295390fda0f276e3a3ee32fa4658489c5d)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- remove useless fonction to access metadata - ([4379e2f](https://github.com/cocogitto/cocogitto/commit/4379e2f1111cdce03f2d6ce9d31228045df550af)) - [@oknozor](https://github.com/oknozor)
- refactor test helpers - ([f7c639e](https://github.com/cocogitto/cocogitto/commit/f7c639ea72c142a1ffbc8e613693384d8cc0a7c5)) - [@oknozor](https://github.com/oknozor)
- clean up minor code details - ([8b2aafa](https://github.com/cocogitto/cocogitto/commit/8b2aafa10606ee69622ee7ed417d692b3013cef9)) - [@tranzystorek-io](https://github.com/tranzystorek-io)

- - -

## [2.1.1](https://github.com/cocogitto/cocogitto/compare/2.1.0..2.1.1) - 2021-07-17
#### Bug Fixes
- (**hooks**) use range to replace version expression in hooks - ([acf354b](https://github.com/cocogitto/cocogitto/commit/acf354b554d9c7cdc8284abecfc65e9c61006db0)) - [@oknozor](https://github.com/oknozor)

- - -

## [2.1.0](https://github.com/cocogitto/cocogitto/compare/2.0.0..2.1.0) - 2021-07-16
#### Features
- add check from latest tag option - ([53f5a91](https://github.com/cocogitto/cocogitto/commit/53f5a91b0059917e9e9b1593b7449fb07f4aa0ad)) - [@oknozor](https://github.com/oknozor)
- add check from latest tag option - ([caa6ec3](https://github.com/cocogitto/cocogitto/commit/caa6ec31abdcf0f9115204c968eb01ec571abdcc)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- fix typo in completion commands - ([80e4887](https://github.com/cocogitto/cocogitto/commit/80e4887e342b5808631cbeb287ca05f6a3b30c89)) - [@oknozor](https://github.com/oknozor)

- - -

## [2.0.0](https://github.com/cocogitto/cocogitto/compare/1.2.0..2.0.0) - 2021-03-18
#### Features
- ![BREAKING](https://img.shields.io/badge/BREAKING-red) (**hook**) add version DSL in cog.toml - ([edf610e](https://github.com/cocogitto/cocogitto/commit/edf610e8cab474b93017faf6404b11e8e4fa1093)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- use tarpaulin 0.16 to fix build before next cargo release - ([7fb50ed](https://github.com/cocogitto/cocogitto/commit/7fb50ed991917581178c121acfb78327a28e384e)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- do some general code clean-up - ([4c74429](https://github.com/cocogitto/cocogitto/commit/4c74429e9cbab88fce0f73c73332102feae48963)) - [@tranzystorek-io](https://github.com/tranzystorek-io)

- - -

## [1.2.0](https://github.com/cocogitto/cocogitto/compare/1.1.0..1.2.0) - 2021-01-19
#### Features
- (**bump**) add --config flag to cog bump - ([635a043](https://github.com/cocogitto/cocogitto/commit/635a043c04a37a4b355b473759ab4071e91530cf)) - [@renaultfernandes](https://github.com/renaultfernandes)
#### Bug Fixes
- get latest tag based on semver format - ([3c4f601](https://github.com/cocogitto/cocogitto/commit/3c4f60138e5eaf38ff317d4b7a8e0878ae6b7a34)) - [@pjvds](https://github.com/pjvds)
#### Continuous Integration
- fix deprecated set env and add path github action commands - ([84b334a](https://github.com/cocogitto/cocogitto/commit/84b334a4c4dae4fa25acd0475e7a311c74c0135e)) - [@oknozor](https://github.com/oknozor)
- add aur package submodule - ([ac87bab](https://github.com/cocogitto/cocogitto/commit/ac87babab51aef2f24e71d4bc684694b2eb65126)) - [@oknozor](https://github.com/oknozor)

- - -

## [1.1.0](https://github.com/cocogitto/cocogitto/compare/1.0.3..1.1.0) - 2020-10-26
#### Features
- (**bump**) stash hook generated changes on prehook failure - ([72a6925](https://github.com/cocogitto/cocogitto/commit/72a6925a582b9038ba7b75d49a76df57ce21adfb)) - [@oknozor](https://github.com/oknozor)
- (**cli**) add shell completions - ([fa24d64](https://github.com/cocogitto/cocogitto/commit/fa24d643c2b8415abc0e931ea3fae7907c40bbc4)) - [@oknozor](https://github.com/oknozor)
- (**cli**) add git-hooks installer - ([940df13](https://github.com/cocogitto/cocogitto/commit/940df1369520a2c68dee9e90e9f8cd0eff346fc3)) - [@oknozor](https://github.com/oknozor)
- (**edit**) add editor hint on cog edit - ([271b920](https://github.com/cocogitto/cocogitto/commit/271b920ceb6eaf61ad2c21f7568ea1e9cedcd0db)) - [@oknozor](https://github.com/oknozor)
#### Bug Fixes
- (**cli**) remove default value for install hook command - ([dac8698](https://github.com/cocogitto/cocogitto/commit/dac8698e72d1d8f1d712fa983c4b0196608dc35d)) - [@oknozor](https://github.com/oknozor)
- (**log**) use shorthand instead of full oid in cog log - ([6ff44d4](https://github.com/cocogitto/cocogitto/commit/6ff44d47c981399c6bfabf8e5a40e6bc30ac2092)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- change git hooks readme title - ([97503fb](https://github.com/cocogitto/cocogitto/commit/97503fb0d665e0a5014c75c6aa308de1f061dfbf)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- (**changelog**) remove unused writter mode: Append & Replace - ([098d6c0](https://github.com/cocogitto/cocogitto/commit/098d6c079379e88c13d77685a5eee4a3be34df67)) - [@oknozor](https://github.com/oknozor)
- (**lib**) extract git statuses to a dedicated module - ([7191f4e](https://github.com/cocogitto/cocogitto/commit/7191f4e25a70fe437b4c567550d2308b3702cbb7)) - [@oknozor](https://github.com/oknozor)
- use dir modules instead of file - ([a69bb2a](https://github.com/cocogitto/cocogitto/commit/a69bb2a6f43f901dafcb1eb17d2ed3685927a862)) - [@oknozor](https://github.com/oknozor)
- use Astr<str> for commit type instead of custom impl - ([bac60fd](https://github.com/cocogitto/cocogitto/commit/bac60fd07ba76fd0648a64aecad67589aeae5eba)) - [@oknozor](https://github.com/oknozor)

- - -

## [1.0.3](https://github.com/cocogitto/cocogitto/compare/1.0.2..1.0.3) - 2020-10-16
#### Bug Fixes
- (**hook**) %version is now interpretted even without space separator - ([2103a7f](https://github.com/cocogitto/cocogitto/commit/2103a7f768cf67eeb85f30ad72a75134ee89e772)) - [@oknozor](https://github.com/oknozor)

- - -

## [1.0.2](https://github.com/cocogitto/cocogitto/compare/1.0.1..1.0.2) - 2020-10-12
#### Bug Fixes
- fix typo in ci LICENSE path - ([2505a44](https://github.com/cocogitto/cocogitto/commit/2505a4442fa477561ce4e17fd4b9c6edc90d99dc)) - [@oknozor](https://github.com/oknozor)

- - -

## [1.0.1](https://github.com/cocogitto/cocogitto/compare/1.0.0..1.0.1) - 2020-10-12
#### Bug Fixes
- treat 0.y.z autobumps specially - ([f113744](https://github.com/cocogitto/cocogitto/commit/f11374426cbdd395a89437e11ae6fbe1eae88144)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Documentation
- add AUR package to README - ([2bd8bb3](https://github.com/cocogitto/cocogitto/commit/2bd8bb3f8534efc7de808417a237c72e84173fa2)) - [@oknozor](https://github.com/oknozor)
- document special behavior of auto bumps on 0.y.z - ([6fb5ec5](https://github.com/cocogitto/cocogitto/commit/6fb5ec579a1532a90619ddccd80788a9c99acc72)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Continuous Integration
- use rust stable in github ci - ([f66ad94](https://github.com/cocogitto/cocogitto/commit/f66ad94585567a496487a989dd3051f61defd387)) - [@oknozor](https://github.com/oknozor)
- add license to release tar - ([fe94f46](https://github.com/cocogitto/cocogitto/commit/fe94f46b0db1b1b73c11a061a134ca1a1285ac54)) - [@oknozor](https://github.com/oknozor)
- build on rust stable instead of nightly - ([1f59a85](https://github.com/cocogitto/cocogitto/commit/1f59a859cb0f382f6a64ef59921c655b90ef2e58)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- replace drain_filter() with a stable alternative - ([7c4a1cb](https://github.com/cocogitto/cocogitto/commit/7c4a1cb692b445f36a44857ce17db32b91acd24c)) - [@tranzystorek-io](https://github.com/tranzystorek-io)

- - -

## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.34.0..1.0.0) - 2020-10-11
#### Features
- (**cli**) include current branch name in "cog log" - ([2bcf972](https://github.com/cocogitto/cocogitto/commit/2bcf972be90c903e334f97cf67e010fe1147bd92)) - [@renaultfernandes](https://github.com/renaultfernandes)
- (**cli**) show repo and current tag name in "cog log" - ([7c6c725](https://github.com/cocogitto/cocogitto/commit/7c6c7259d67bbb7b5ae42df6c3b53dacd503073d)) - [@renaultfernandes](https://github.com/renaultfernandes)
- ![BREAKING](https://img.shields.io/badge/BREAKING-red) rename hooks to pre_bump_hooks - ([7015c51](https://github.com/cocogitto/cocogitto/commit/7015c51480c15171cc211efb0c3eda0854fc9b09)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- add post-bump-hooks - ([a56b1e2](https://github.com/cocogitto/cocogitto/commit/a56b1e268703e9548fdd90b0a40d0ef602b29156)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Bug Fixes
- (**ci**) move cargo package to post bump - ([602030e](https://github.com/cocogitto/cocogitto/commit/602030e6bdb519f716ee4439300118cf5fe5e4c3)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- add signature to contributors list - ([78a9978](https://github.com/cocogitto/cocogitto/commit/78a99781add5b25a5126d1d939adf67ef5748687)) - [@renaultfernandes](https://github.com/renaultfernandes)
#### Refactoring
- fix clippy lints - ([061004e](https://github.com/cocogitto/cocogitto/commit/061004e682269b615a85aa51adc52cfbe5c696ec)) - [@renaultfernandes](https://github.com/renaultfernandes)

- - -

## [0.34.0](https://github.com/cocogitto/cocogitto/compare/0.33.1..0.34.0) - 2020-10-10
#### Features
- (**cli**) use external pager instead of moins - ([e4d5fef](https://github.com/cocogitto/cocogitto/commit/e4d5fef7cdb5b3421345b21491c01407e509cfe2)) - Mike
- pre-commit bump hooks - ([c11147d](https://github.com/cocogitto/cocogitto/commit/c11147d7ed082d05f0731512579c6dfa6dbc8831)) - Mike
#### Bug Fixes
- cog bump now perform a single version bump (#44) - ([b0609e7](https://github.com/cocogitto/cocogitto/commit/b0609e7920cc4aae093fca49e9643973610c41b0)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- (**hook**) add documentation for version hooks - ([cf5419c](https://github.com/cocogitto/cocogitto/commit/cf5419c29ebc1ffb355c00668b7adcd2d646ae7d)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- update codecov action to work with forks - ([42827f0](https://github.com/cocogitto/cocogitto/commit/42827f02f762fbd36af88489e929abe12e603030)) - [@oknozor](https://github.com/oknozor)

- - -

## [0.33.1](https://github.com/cocogitto/cocogitto/compare/0.32.3..0.33.1) - 2020-10-06
#### Features
- (**cli**) bump --pre flag to set the pre-release version - ([05a487a](https://github.com/cocogitto/cocogitto/commit/05a487aa73b55e7f84d324fc30b145de67b75d91)) - [@mersinvald](https://github.com/mersinvald)
#### Bug Fixes
- typo in get_committer - ([f97a6f3](https://github.com/cocogitto/cocogitto/commit/f97a6f33b3f1992018208747746332dab60b05b3)) - [@jackdorland](https://github.com/jackdorland)
#### Documentation
- add log filters to the doc - ([dff77b3](https://github.com/cocogitto/cocogitto/commit/dff77b3a13d9ca1c9fd5720a7e3e688db7338996)) - [@oknozor](https://github.com/oknozor)
- add  bump flag to the doc - ([a1906c3](https://github.com/cocogitto/cocogitto/commit/a1906c3ea740efd15882c7a957d9d62e2ab2182e)) - [@oknozor](https://github.com/oknozor)
- add contributors github usernames to cog.toml - ([1c66d72](https://github.com/cocogitto/cocogitto/commit/1c66d72dd250a722d3b96c15b114a077592e342e)) - [@oknozor](https://github.com/oknozor)

- - -

## [0.32.3](https://github.com/cocogitto/cocogitto/compare/0.32.2..0.32.3) - 2020-09-30
#### Bug Fixes
- fix openssl missing in CD - ([1c0d2e9](https://github.com/cocogitto/cocogitto/commit/1c0d2e9398323e6d4fc778309bed242040aa55b5)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- tag, conventional commit and license badges to readme - ([da6f63d](https://github.com/cocogitto/cocogitto/commit/da6f63db9577a9e4ec9d3b10c3022e80be2d0f69)) - [@oknozor](https://github.com/oknozor)

- - -

## [0.32.2](https://github.com/cocogitto/cocogitto/compare/0.32.1..0.32.2) - 2020-09-30
#### Bug Fixes
- (**cd**) bump setup-rust-action to v1.3.3 - ([5350b11](https://github.com/cocogitto/cocogitto/commit/5350b110b4e39bf6942a58b7a89425e21927b966)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- add corrections to README - ([9a33516](https://github.com/cocogitto/cocogitto/commit/9a33516649ba8dd15fafbb6b22970efab1c04dee)) - [@oknozor](https://github.com/oknozor)

- - -

## [0.32.1](https://github.com/cocogitto/cocogitto/compare/0.30.0..0.32.1) - 2020-09-30
#### Features
- move check edit to dedicated subcommand and fix rebase - ([fc74207](https://github.com/cocogitto/cocogitto/commit/fc74207b943bfd1b3e36eab80f943e349b0eef36)) - [@oknozor](https://github.com/oknozor)
- remove config commit on init existing repo - ([1028d0b](https://github.com/cocogitto/cocogitto/commit/1028d0bee3c12756fd429787a88232bdeae9dc81)) - [@oknozor](https://github.com/oknozor)
#### Bug Fixes
- (**cd**) fix ci cross build command bin args - ([7f04a98](https://github.com/cocogitto/cocogitto/commit/7f04a985b05be36dff170795767a7ad4e696eb4d)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- rewritte readme completely - ([b223f7b](https://github.com/cocogitto/cocogitto/commit/b223f7bec7f2f9df2189e56ffc7177ffa49d6440)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- change config name to cog.toml - ([d4aa61b](https://github.com/cocogitto/cocogitto/commit/d4aa61b20ee0d5dd2299f8cb97a75186c75a64f5)) - [@oknozor](https://github.com/oknozor)

- - -

## [0.30.0](https://github.com/cocogitto/cocogitto/compare/0.29.0..0.30.0) - 2020-09-28
#### Features
- (**changelog**) improve changelog title formatting - ([d713886](https://github.com/cocogitto/cocogitto/commit/d7138865ee4d57a7b8bc18d8fcb73d43feedf504)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- (**cd**) fix publish action script - ([d0d0ae9](https://github.com/cocogitto/cocogitto/commit/d0d0ae928069a1cdb9cb81f4e483f93c4abc29b0)) - [@oknozor](https://github.com/oknozor)

- - -

## [0.29.0](https://github.com/cocogitto/cocogitto/compare/8806a55727b6c1767cca5d494599623fbb5dd1dd..0.29.0) - 2020-09-27
#### Features
- (**changelog**) add author map for github markdown rendering - ([ba16b89](https://github.com/cocogitto/cocogitto/commit/ba16b89d5dc8e8c03661fd091fa320d09f1ecf05)) - [@oknozor](https://github.com/oknozor)
- (**changelog**) add changelog generation for bump command - ([1bdb65a](https://github.com/cocogitto/cocogitto/commit/1bdb65aa01a4cc977ebf91fde557d6d2e1e83331)) - [@oknozor](https://github.com/oknozor)
- (**changelog**) add changelog arg modes - ([21abecf](https://github.com/cocogitto/cocogitto/commit/21abecfa06c9b4aec798197679b27cfd6f0dc7eb)) - [@oknozor](https://github.com/oknozor)
- (**changelog**) add changelog date - ([7b7e474](https://github.com/cocogitto/cocogitto/commit/7b7e474a5995ccaa555edbc498ce4d38054cc829)) - [@oknozor](https://github.com/oknozor)
- (**changelog**) add colors to changelog - ([d0e87bf](https://github.com/cocogitto/cocogitto/commit/d0e87bfb78df99cecc44350c320f7c997e1a5362)) - [@oknozor](https://github.com/oknozor)
- (**changelog**) convert changelog to markdown - ([e858d55](https://github.com/cocogitto/cocogitto/commit/e858d5591be359d4e04c17c9f85a9a336c4ec59e)) - [@oknozor](https://github.com/oknozor)
- (**check**) add edit flag for interactive rebase commit renaming - ([ab054a3](https://github.com/cocogitto/cocogitto/commit/ab054a3c3f93c070b8e9a6717b556961ec7602c7)) - [@oknozor](https://github.com/oknozor)
- (**check**) add check command - ([b932a5e](https://github.com/cocogitto/cocogitto/commit/b932a5e0873888edb1a369596a38f55fd9404ae7)) - [@oknozor](https://github.com/oknozor)
- (**cli**) improve git statuses display - ([cf380e6](https://github.com/cocogitto/cocogitto/commit/cf380e6e5b6dae6db7a47a9ae125a334f5db064e)) - [@oknozor](https://github.com/oknozor)
- (**cli**) add DeriveDiplayOrder to cli - ([92cca40](https://github.com/cocogitto/cocogitto/commit/92cca40d897aa2a758f167119e275cd5aea23dbc)) - [@oknozor](https://github.com/oknozor)
- (**cli**) split commit and utility command into separate bins - ([d2ebbe7](https://github.com/cocogitto/cocogitto/commit/d2ebbe77bbae2fbc7e31d73a5ecb5fac47b80785)) - [@oknozor](https://github.com/oknozor)
- (**cli**) add custom commit type help generation and fix cli help display order - ([fe0e143](https://github.com/cocogitto/cocogitto/commit/fe0e143926c6ef36308487bfe61db9441ba76660)) - [@oknozor](https://github.com/oknozor)
- (**commit**) display git statuses and error message on commit to empty index - ([2f37106](https://github.com/cocogitto/cocogitto/commit/2f3710644fc4f2095eed7f73d941d8b49a1d94cd)) - [@oknozor](https://github.com/oknozor)
- (**commit**) reimplement custom commits - ([9f29665](https://github.com/cocogitto/cocogitto/commit/9f296650d65fee74a88de0f69997cecbe37dcad8)) - [@oknozor](https://github.com/oknozor)
- (**commit**) add commit optional args {body} {footer} and {breaking-change} - ([819a04d](https://github.com/cocogitto/cocogitto/commit/819a04d6a621dccb59892b999739dd81fcbfc806)) - [@oknozor](https://github.com/oknozor)
- (**commit**) add commit pretty print - ([2248c90](https://github.com/cocogitto/cocogitto/commit/2248c9072345ae279497bf1beab7ea150affcc66)) - [@oknozor](https://github.com/oknozor)
- (**commit**) add commit date - ([bbbc908](https://github.com/cocogitto/cocogitto/commit/bbbc9084177dab0b5fee79ceae993bdc64b0726a)) - [@oknozor](https://github.com/oknozor)
- (**commit**) add markdown formating - ([925adb4](https://github.com/cocogitto/cocogitto/commit/925adb484ca98efcb4a54525eec064bfe3e12ec1)) - [@oknozor](https://github.com/oknozor)
- (**hook**) add example pre-commit hook - ([fac83f2](https://github.com/cocogitto/cocogitto/commit/fac83f2fb371010a349c77298759d90a12f636e8)) - [@oknozor](https://github.com/oknozor)
- (**log**) add  log filter - ([3ebaac0](https://github.com/cocogitto/cocogitto/commit/3ebaac00622e3db773b2c48b633a972286af87b8)) - [@oknozor](https://github.com/oknozor)
- (**log**) add multiple args for log filters - ([88f6f2b](https://github.com/cocogitto/cocogitto/commit/88f6f2bbba1954a2990458a12a1a652051044b6c)) - [@oknozor](https://github.com/oknozor)
- (**log**) add log filters - ([44bc3f3](https://github.com/cocogitto/cocogitto/commit/44bc3f38d35e4575e24d18439f23dd9dd0725ae8)) - [@oknozor](https://github.com/oknozor)
- (**scope**) this is a commit message - ([fc0962d](https://github.com/cocogitto/cocogitto/commit/fc0962d439e0080309ad67249aeb61535b665394)) - [@oknozor](https://github.com/oknozor)
- add init subcommand in cli and the ability to use cog outside git - ([45ac57a](https://github.com/cocogitto/cocogitto/commit/45ac57abb4a8e12145cb62ce8223663f67dba534)) - [@oknozor](https://github.com/oknozor)
- add custom git and semver error types - ([5ff48a0](https://github.com/cocogitto/cocogitto/commit/5ff48a0625ece8d2c4d9af403aa5077ae1b49dd8)) - [@oknozor](https://github.com/oknozor)
- add log command and improve logging - ([ce4d62c](https://github.com/cocogitto/cocogitto/commit/ce4d62cef8d775f800aa91d63626995bbadb56cb)) - [@oknozor](https://github.com/oknozor)
- add verify command - ([d7508af](https://github.com/cocogitto/cocogitto/commit/d7508afde5e073bcf3b1aaaa149de9baa721b2ee)) - [@oknozor](https://github.com/oknozor)
- add commit command - ([0309325](https://github.com/cocogitto/cocogitto/commit/0309325413c618a06bd945d703be31aee6ace6a8)) - [@oknozor](https://github.com/oknozor)
- implement changelog - ([46dad5b](https://github.com/cocogitto/cocogitto/commit/46dad5b7aa41f5fac7d71a1b2add6ded912f3305)) - [@oknozor](https://github.com/oknozor)
#### Bug Fixes
- (**changelog**) fix changelog markdown format - ([342f81a](https://github.com/cocogitto/cocogitto/commit/342f81a0313d9a2fd0812d3afc01050b6ad4637d)) - [@oknozor](https://github.com/oknozor)
- (**git**) hide internal method visibility and fix some clippy lints - ([55f62ac](https://github.com/cocogitto/cocogitto/commit/55f62ac87c42376e386d6d64d701f045f25e64e0)) - [@oknozor](https://github.com/oknozor)
- (**git**) decrease  method visibility - ([d5684c4](https://github.com/cocogitto/cocogitto/commit/d5684c437daa81848bdd5758eec4a222073a6381)) - [@oknozor](https://github.com/oknozor)
- (**semver**) add bump test and fix version auto bump - ([eeb917e](https://github.com/cocogitto/cocogitto/commit/eeb917e8e2e685f74813d00783ae36556cacf371)) - [@oknozor](https://github.com/oknozor)
- fix error: 'parent index out of bounds' (#18) - ([d2270fc](https://github.com/cocogitto/cocogitto/commit/d2270fcad3a6b818a5f6c73e78e8500fbc092748)) - [@oknozor](https://github.com/oknozor)
- add line break between changelog commit line - ([f3dc3b9](https://github.com/cocogitto/cocogitto/commit/f3dc3b96a3b24fd2083a2c07a47776d57829aec0)) - [@oknozor](https://github.com/oknozor)
- bump version arg - ([17668ba](https://github.com/cocogitto/cocogitto/commit/17668baeda1369910789c4ceeee0968a6a2f7243)) - [@oknozor](https://github.com/oknozor)
#### Revert
- (**settings**) remove changelog header and footer from config - ([6d810a6](https://github.com/cocogitto/cocogitto/commit/6d810a67d20489860a5c3c24dcee8d4c03056449)) - [@oknozor](https://github.com/oknozor)
- remove test changelog - ([ba4a2cf](https://github.com/cocogitto/cocogitto/commit/ba4a2cfc23eea7186761e5738441793755b503e5)) - [@oknozor](https://github.com/oknozor)
- remove commit sort struct - ([329587f](https://github.com/cocogitto/cocogitto/commit/329587fdf19167fbe652fe675b083dd84e4ca976)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- add codecov badge - ([ee63242](https://github.com/cocogitto/cocogitto/commit/ee6324275263c31ea9609b29fc3e9d53f3441d9b)) - [@oknozor](https://github.com/oknozor)
- line break after logo in readme - ([b67e0e8](https://github.com/cocogitto/cocogitto/commit/b67e0e8f8102a05fb58a2f76827cea9196368960)) - [@oknozor](https://github.com/oknozor)
- add ci badge to readme - ([00aadb5](https://github.com/cocogitto/cocogitto/commit/00aadb5cd082a2f51a47a2f6de8b810b0667ef24)) - [@oknozor](https://github.com/oknozor)
- add toc to README.md - ([346716a](https://github.com/cocogitto/cocogitto/commit/346716abf5fc155988c70c8b304237d090c764cf)) - [@oknozor](https://github.com/oknozor)
- add README.md - ([aa4a853](https://github.com/cocogitto/cocogitto/commit/aa4a853a787695fa9a247b7570a7a4a07a50419b)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- run tarpaulin on one thread - ([d7f3146](https://github.com/cocogitto/cocogitto/commit/d7f3146a762b9f07b9efe69ff9bc673371f1fbbf)) - [@oknozor](https://github.com/oknozor)
- split tarpaulin and unit tests - ([88d67a0](https://github.com/cocogitto/cocogitto/commit/88d67a09a919a386cb1f367c4e0a75db1f5664cd)) - [@oknozor](https://github.com/oknozor)
- add git user for tarpaulin - ([35085f2](https://github.com/cocogitto/cocogitto/commit/35085f20c5293fc8830e4e44a9bb487f98734f73)) - [@oknozor](https://github.com/oknozor)
- add github action ci/cd - ([a1147ba](https://github.com/cocogitto/cocogitto/commit/a1147ba3cd9cf92cc4013c2ddec40b08d7dc5d71)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- (**commit**) move conventional commit command logic to pub function - ([156d9b5](https://github.com/cocogitto/cocogitto/commit/156d9b586140cf193201ff6608230532796da8a0)) - [@oknozor](https://github.com/oknozor)
- (**commit**) move commit to a dedicated module - ([3cd77b6](https://github.com/cocogitto/cocogitto/commit/3cd77b64cb46ec6e231cee3578ed6ec2cdaca79e)) - [@oknozor](https://github.com/oknozor)
- (**semver**) extract version logic to a dedicated module - ([a2b7098](https://github.com/cocogitto/cocogitto/commit/a2b7098f8351015cedeb6c971e641b413976b36a)) - [@oknozor](https://github.com/oknozor)
- refactor verify to get current user signature - ([dad15d1](https://github.com/cocogitto/cocogitto/commit/dad15d17e175021453c61d8f7e919cba944faa63)) - [@oknozor](https://github.com/oknozor)
- replace custom semver struct with semver crate - ([f780561](https://github.com/cocogitto/cocogitto/commit/f780561a95a75bc132637825d43db029335ae8b9)) - [@oknozor](https://github.com/oknozor)
- clippy lints - ([0639872](https://github.com/cocogitto/cocogitto/commit/0639872f1076e6cac320de0a118e7e545f1a2300)) - [@oknozor](https://github.com/oknozor)
- rework check command output and commit parsing - ([d9cf446](https://github.com/cocogitto/cocogitto/commit/d9cf446afd1675fa59858e1569ab517833e796e1)) - [@oknozor](https://github.com/oknozor)
- add closure for markdown commit section generation - ([2537a32](https://github.com/cocogitto/cocogitto/commit/2537a32ded42cea58675de1cf29a0aa61595df44)) - [@oknozor](https://github.com/oknozor)
- - -

This changelog was generated by [cocogitto](https://github.com/oknozor/cocogitto).
