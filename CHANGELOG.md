# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [6.2.0](https://github.com/cocogitto/cocogitto/compare/6.1.0..6.2.0) - 2024-11-28
#### Bug Fixes
- fix ignore merge commit no longer honored - ([112bfcc](https://github.com/cocogitto/cocogitto/commit/112bfcc4a014b05dc43712cd4cb32846e5923251)) - [@oknozor](https://github.com/oknozor)
- populate monorepo changelog tera context - ([3875f65](https://github.com/cocogitto/cocogitto/commit/3875f65918f2710b335a872a433b5cef0a7c82b6)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- fix codedov upload - ([6f3a292](https://github.com/cocogitto/cocogitto/commit/6f3a292c7c52144c225c4901c01d5c1020a1f112)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- **(config)** add link to cog.toml config reference to README and cog.toml - ([ab1fccb](https://github.com/cocogitto/cocogitto/commit/ab1fccba029aa0c05345cb38c98a99c01a09e8ea)) - Emily Zall
- **(website)** clarify which commit types auto bump by default and how to change this config - ([13ab5a3](https://github.com/cocogitto/cocogitto/commit/13ab5a3e4112efb4705d7b82da9b95bba894a179)) - Emily Zall
- **(website)** fix some typos - ([7096430](https://github.com/cocogitto/cocogitto/commit/7096430d1852cecf8e85d842971563cb71dc8b2f)) - mroetsc
- **(website)** include MacOS install instructions on homepage - ([7f6b5f1](https://github.com/cocogitto/cocogitto/commit/7f6b5f1d3cb50db10b89b77c19a45ee2611df903)) - Ali Dowair
- migrate to vitepress - ([3ff98b2](https://github.com/cocogitto/cocogitto/commit/3ff98b20eeeb4e8b44000e23ddb77de53902f477)) - [@oknozor](https://github.com/oknozor)
- update tera link in README.md - ([24dd3da](https://github.com/cocogitto/cocogitto/commit/24dd3da91ee0427d7cc5ecfe36fff3cc0521566b)) - David LJ
- correct minor typos (#396) - ([72e1f86](https://github.com/cocogitto/cocogitto/commit/72e1f8624939a089414aa28418bd2249972fac23)) - tjmurray
#### Features
- **(settings)** allow to disable default commit types - ([b84247f](https://github.com/cocogitto/cocogitto/commit/b84247fd39c2bb1aff685293fe32cc4d43dfa090)) - [@oknozor](https://github.com/oknozor)
- allow to disable package tagging for monorepos - ([426223e](https://github.com/cocogitto/cocogitto/commit/426223e807e0e653afc85cde498a5db976babfa9)) - [@oknozor](https://github.com/oknozor)
- open repo at path - ([f496807](https://github.com/cocogitto/cocogitto/commit/f4968079684972c9d616f29e0cfa4aeb2d56b344)) - Finley Thomalla
- Add optional default version definition in the VersionDSL, the separator is a `|`, the default version is defined in a way that allows us to apply modifiers to it when a version is not available. The same applies for tags (we use the tag prefix defined in the settings). - ([07ae2ac](https://github.com/cocogitto/cocogitto/commit/07ae2acca7cca729693c3c56515f77f79865366c)) - Matjaz Domen Pecan
#### Miscellaneous Chores
- **(deps-dev)** bump follow-redirects from 1.15.4 to 1.15.6 in /docs - ([bf0c67b](https://github.com/cocogitto/cocogitto/commit/bf0c67b05a3a2986b64dfc5cca2ea956058f6996)) - dependabot[bot]
- **(documentation)** More clarity in documentation, add clarification that the default cannot be used with the package keyword. - ([23e4310](https://github.com/cocogitto/cocogitto/commit/23e43101fd9611d13f053eced5737fd9ced5e1d5)) - Matjaz Domen Pecan
- **(documentation)** Break the lines on added documentation to make it more readable. - ([6da99a2](https://github.com/cocogitto/cocogitto/commit/6da99a251259c7686ed03f33d7778581712f6f26)) - Matjaz Domen Pecan
- **(documentation)** Fleshing out the documentation about defaults. - ([68c3528](https://github.com/cocogitto/cocogitto/commit/68c3528152513b6937e828387e7e88a727a2f7db)) - Matjaz Domen Pecan
- **(documentation)** add documentation for the VersionDSL default version functionality. - ([145df03](https://github.com/cocogitto/cocogitto/commit/145df0300a93a301614ccbdf6f6c577446aaaebf)) - Matjaz Domen Pecan
- **(rustfmt)** Forgot to run it before. - ([2283359](https://github.com/cocogitto/cocogitto/commit/22833597e148afac2bb1ea7002592f1e6c889160)) - Matjaz Domen Pecan
- update docs - ([ded0d4d](https://github.com/cocogitto/cocogitto/commit/ded0d4d7d29db0f194f545a9cfbbdd5f6fa1ade2)) - Jacob Torrång
- cargo update - ([7d022f9](https://github.com/cocogitto/cocogitto/commit/7d022f970be66c0b3a26d89bb5471609b4c84352)) - [@oknozor](https://github.com/oknozor)
#### Tests
- **(changelog)** add reproducer for #388 - ([f5ac58b](https://github.com/cocogitto/cocogitto/commit/f5ac58b5e74f21036083a294e7353fb1be382c5c)) - [@oknozor](https://github.com/oknozor)

- - -

## [6.1.0](https://github.com/cocogitto/cocogitto/compare/6.0.1..6.1.0) - 2024-03-14
#### Bug Fixes
- **(bump)** use commit meta to determine no bump commits - ([f9d3dd3](https://github.com/cocogitto/cocogitto/commit/f9d3dd35a1adeffc465b5aa7e3645bdbcdff7bbf)) - Maksym Kondratenko
#### Continuous Integration
- fixed copy wrong path - ([facdefb](https://github.com/cocogitto/cocogitto/commit/facdefbae670f4b2ca245a1106dc9d07345ac993)) - ABWassim
#### Documentation
- **(README)** fix typo - ([64fc19c](https://github.com/cocogitto/cocogitto/commit/64fc19cef4e3625d7dc15f4104871a3813002ae5)) - Oluf Lorenzen
- **(bump)** disable bump commit - ([12df7a2](https://github.com/cocogitto/cocogitto/commit/12df7a23d6bea841d00314f7c4fa07ad1e2c6f57)) - ABWassim
- **(commits-types)** bump minor and patch options - ([dd5517b](https://github.com/cocogitto/cocogitto/commit/dd5517b4a1d079c38565ac7b3c223ea25d9a8310)) - ABWassim
- update docs with semver build meta - ([aec74df](https://github.com/cocogitto/cocogitto/commit/aec74df67038732c872c45334b38141a0ce6303a)) - David Arnold
#### Features
- **(bump)** disable bump commit - ([e6b5468](https://github.com/cocogitto/cocogitto/commit/e6b5468c698ab15db6129a3edb7c8ec48895cdcc)) - ABWassim
- **(commit)** add gitsign support - ([56a8f32](https://github.com/cocogitto/cocogitto/commit/56a8f32f2591d0b6985fb8173c8305d9883e5579)) - [@oknozor](https://github.com/oknozor)
- **(commit)** add and update files - ([0666ffe](https://github.com/cocogitto/cocogitto/commit/0666ffed9c93942751258622c4fbf08ba2e779c2)) - ABWassim
- add build version to command - ([1680042](https://github.com/cocogitto/cocogitto/commit/1680042edd9f811fee2f6232a7573e456d3a65b9)) - David Arnold
- ssh signing for commits - ([3cd580e](https://github.com/cocogitto/cocogitto/commit/3cd580e6b6cfb500e1d14fa84486f26d0b0523db)) - [@DaRacci](https://github.com/DaRacci)
- complete rework of revspec and revwalk - ([6a3b2db](https://github.com/cocogitto/cocogitto/commit/6a3b2dbabeaf243050902628810e684ff278590d)) - [@oknozor](https://github.com/oknozor)
- add additional package path filtering options - ([dde8ffe](https://github.com/cocogitto/cocogitto/commit/dde8ffe38ac4b3395e55f542ff7d1d57f1f89a43)) - Greg Fairbanks
#### Miscellaneous Chores
- **(commit)** reinforce skip-ci tests and process - ([9f7fcd6](https://github.com/cocogitto/cocogitto/commit/9f7fcd6029edc88f737e7d0dc3c0f0e7de5996da)) - ABWassim
- **(deps)** bump vite and vuepress in /docs - ([01cfa4a](https://github.com/cocogitto/cocogitto/commit/01cfa4a3a9a81c942307759b2e8d6c8c72fea8c3)) - dependabot[bot]
- **(deps-dev)** bump vite from 4.4.11 to 4.4.12 in /docs - ([21ceb2a](https://github.com/cocogitto/cocogitto/commit/21ceb2a915d641c8154d94bbdd95e87dff2b58ba)) - dependabot[bot]
- **(deps-dev)** bump follow-redirects from 1.15.3 to 1.15.4 in /docs - ([398019b](https://github.com/cocogitto/cocogitto/commit/398019bc68989779d25448ab7cb7270603434604)) - dependabot[bot]
- **(doc)** update repo name in doc - ([19a9303](https://github.com/cocogitto/cocogitto/commit/19a9303a6f2f7c90b800ce12ff33c53d509b9410)) - Guillaume Gayot
- **(website)** add doc on how to access tag with prefix in conf - ([a3da049](https://github.com/cocogitto/cocogitto/commit/a3da049b0adc0e9a70e9e95eb002127e39cbb555)) - SergeJomon
- thanks clippy - ([093306a](https://github.com/cocogitto/cocogitto/commit/093306a06667e350fbfeae4159edd7c34c705fde)) - [@oknozor](https://github.com/oknozor)
- update dependencies - ([a4403ce](https://github.com/cocogitto/cocogitto/commit/a4403cea8b7a4a2dd60f3781a9546515fc3a7f3d)) - [@oknozor](https://github.com/oknozor)
- fix deprecated usage in chrono - ([d7edf13](https://github.com/cocogitto/cocogitto/commit/d7edf13296f0305eece034f747b7f252476aa72c)) - [@oknozor](https://github.com/oknozor)
- thanks clippy - ([104d23a](https://github.com/cocogitto/cocogitto/commit/104d23ac072b06ec6e0f01d64504881295b6e598)) - [@oknozor](https://github.com/oknozor)
#### Performance Improvements
- **(changelog)** build cache once - ([1ca130f](https://github.com/cocogitto/cocogitto/commit/1ca130f8e0c034f1cadc18bedffe48c23779886c)) - [@oknozor](https://github.com/oknozor)
- **(revwalk)** cache every tags and ref - ([99eef16](https://github.com/cocogitto/cocogitto/commit/99eef1603c58b5d15a7be24e484838d182d13431)) - [@oknozor](https://github.com/oknozor)
- add flamegraph and massif scripts to justfile - ([3f1d2cf](https://github.com/cocogitto/cocogitto/commit/3f1d2cf6ae5c91de363c2bacc5a5e4a0dcf4aea1)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- **(bump)** wrap bump and commit args into dedicated opts structs - ([452097d](https://github.com/cocogitto/cocogitto/commit/452097dd152542e3783a08ecd5ee842a8dcb7810)) - [@oknozor](https://github.com/oknozor)
- try from impl for release - ([06ec52b](https://github.com/cocogitto/cocogitto/commit/06ec52bbc29f08d3937149c522d575d736eec010)) - [@oknozor](https://github.com/oknozor)
#### Tests
- **(bump)** more skip-ci tests - ([1c478ed](https://github.com/cocogitto/cocogitto/commit/1c478ed1cf9b6bb3f2e2d976f22361f7647c0815)) - ABWassim
- **(bump)** disable bump commit - ([b03d66e](https://github.com/cocogitto/cocogitto/commit/b03d66e468db4f40d6f3609187ae5ce010ef6ba5)) - ABWassim
- **(commit)** add and update files - ([9bf9045](https://github.com/cocogitto/cocogitto/commit/9bf9045e7bd923506b916ca9d587039905ebc8ef)) - ABWassim
- update test keys expiration - ([a27b725](https://github.com/cocogitto/cocogitto/commit/a27b725385939673f59058d4d26a7882eb03b829)) - [@oknozor](https://github.com/oknozor)
- fix monorepo test - ([a9be507](https://github.com/cocogitto/cocogitto/commit/a9be5079cab0c21a2ebde065ebd64b7947fdcc16)) - [@oknozor](https://github.com/oknozor)

- - -

## [6.0.1](https://github.com/cocogitto/cocogitto/compare/6.0.0..6.0.1) - 2023-11-30
#### Bug Fixes
- gh pages build path - ([52b8307](https://github.com/cocogitto/cocogitto/commit/52b8307f23d76cf7d853ff60631e0872a1fb268e)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- fix the format of release assets - ([8a1df55](https://github.com/cocogitto/cocogitto/commit/8a1df55ed99d05ca9443a348f446dfdab42fd2fa)) - Shunsuke Suzuki
#### Documentation
- fix gh-pages deploy - ([4c79de9](https://github.com/cocogitto/cocogitto/commit/4c79de930fd120e6cee12fa49c3f96e4a52d2faa)) - [@oknozor](https://github.com/oknozor)

- - -

## [6.0.0](https://github.com/cocogitto/cocogitto/compare/5.6.0..6.0.0) - 2023-11-30
#### Bug Fixes
- shortened commit when parsing into oidof - ([7ed4fa7](https://github.com/cocogitto/cocogitto/commit/7ed4fa738bec4821b8873344b123ca1a70087e9d)) - ABWassim
- correctly handle bump with pre-release tags - ([891d55b](https://github.com/cocogitto/cocogitto/commit/891d55b2293d10d556455ad4cadb75e19b2ac819)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- check tags for release job instead of steps - ([fb23340](https://github.com/cocogitto/cocogitto/commit/fb233402864134304e1ef4c730638062c7125895)) - [@oknozor](https://github.com/oknozor)
- update docker image and gh release - ([377c2c2](https://github.com/cocogitto/cocogitto/commit/377c2c2481c4403657f825d3c4934d5a8d614098)) - [@oknozor](https://github.com/oknozor)
- docker build - ([b1ca37b](https://github.com/cocogitto/cocogitto/commit/b1ca37bbefdba3eaefa7a9139ef3fe4b4ac07f85)) - [@oknozor](https://github.com/oknozor)
- add docker multiarch build - ([9fea07f](https://github.com/cocogitto/cocogitto/commit/9fea07fadd47874b4d8f0ae938b6473daeb5670b)) - [@oknozor](https://github.com/oknozor)
- run tests on all platforms and add a windows binary to release - ([5987707](https://github.com/cocogitto/cocogitto/commit/59877075e0d90e9e36c5e1b301f00fd7507641d8)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- remove coco deprecation notice - ([6ce82f4](https://github.com/cocogitto/cocogitto/commit/6ce82f42a82273333aa55018f56b8ec896896a96)) - [@oknozor](https://github.com/oknozor)
- add docker to readme and link to github app page - ([6931c79](https://github.com/cocogitto/cocogitto/commit/6931c790641b98b7065fcd71615a3ea2656a5dba)) - [@oknozor](https://github.com/oknozor)
- fix indentation for gh action doc - ([809b386](https://github.com/cocogitto/cocogitto/commit/809b3867f5d7722096620e628473c6f26500bc30)) - [@oknozor](https://github.com/oknozor)
- add docs for the docker image - ([bedfaa8](https://github.com/cocogitto/cocogitto/commit/bedfaa81775dd5b8aae1967adbba6720bed05377)) - [@oknozor](https://github.com/oknozor)
- skip-ci breaking change - ([8e05d9c](https://github.com/cocogitto/cocogitto/commit/8e05d9c01f23e3dbdfa390cea0f7268179c10718)) - ABWassim
- migrate docs to main repo - ([fb73056](https://github.com/cocogitto/cocogitto/commit/fb73056b0968c13773865099f2f744d9336af109)) - [@oknozor](https://github.com/oknozor)
- add installation instructions for MacOS - ([b64ce0f](https://github.com/cocogitto/cocogitto/commit/b64ce0f6b68bbcf3ee4f5b6acfdf71d037660f65)) - Gianluca Recchia
#### Features
- **(bump)** skip-ci rework - ([3a42e68](https://github.com/cocogitto/cocogitto/commit/3a42e685d9182bbd3148cca8ab532116da6a12d8)) - ABWassim
- add docker image - ([c28d934](https://github.com/cocogitto/cocogitto/commit/c28d934fd60e75637902e3eb0543d36a88674b42)) - [@oknozor](https://github.com/oknozor)
- allow bumping with other commit types - ([49d586a](https://github.com/cocogitto/cocogitto/commit/49d586ac1d70057ea0a83a124daba4f6d8b6e1ff)) - [@oknozor](https://github.com/oknozor)
- version access substitution - ([3346a84](https://github.com/cocogitto/cocogitto/commit/3346a84b5f2a8dd7b2482a868599a6985ae2635a)) - Ezekiel Warren
- Allow users to disable changelog generation - ([17f98dc](https://github.com/cocogitto/cocogitto/commit/17f98dc3ea2ebf233fb19f8e25700fab3664058f)) - Billie Thompson
- add option to overwrite existing git-hooks - ([ec2c4c4](https://github.com/cocogitto/cocogitto/commit/ec2c4c43870bc81839b02bd581a62010b2dfb55a)) - marcbull
#### Miscellaneous Chores
- cargo update && switch to main for gh release - ([0e8f375](https://github.com/cocogitto/cocogitto/commit/0e8f375406cf3ef01491c8ec7921904dc04f5b3a)) - [@oknozor](https://github.com/oknozor)
- git ignore docs build - ([187dd4f](https://github.com/cocogitto/cocogitto/commit/187dd4fdf7308f07d4c721235a9ed5f2b3dcfb51)) - [@oknozor](https://github.com/oknozor)
- update dependencies - ([94ba7bc](https://github.com/cocogitto/cocogitto/commit/94ba7bce7d9b86d5322e41118ec751b9acbb61b1)) - [@oknozor](https://github.com/oknozor)
- fix clippy lints - ([9cd0644](https://github.com/cocogitto/cocogitto/commit/9cd064422207f27f837521049f731433d5566cc7)) - [@oknozor](https://github.com/oknozor)
#### Tests
- **(bump)** fixed missing parameters - ([4bdbe9b](https://github.com/cocogitto/cocogitto/commit/4bdbe9b4464bbc40a0e60f090c6c2dfa81117d67)) - ABWassim
- **(bump)** added skip-ci tests - ([39ebfa7](https://github.com/cocogitto/cocogitto/commit/39ebfa707a678ea546da449b631ef55d79deb78b)) - ABWassim
- **(commit)** added skip-ci tests - ([9b31eae](https://github.com/cocogitto/cocogitto/commit/9b31eaed26d25d0bf129e46b2d864fc70743ac1f)) - ABWassim
- add overwrite test for install_git _hook - ([b471cac](https://github.com/cocogitto/cocogitto/commit/b471cac27d4cc2235ec80974cfce083dbceba8d3)) - marcbull

- - -

## [5.6.0](https://github.com/cocogitto/cocogitto/compare/5.5.0..5.6.0) - 2023-09-27
#### Bug Fixes
- **(bump)** option to disable untracked changes error - ([da459de](https://github.com/cocogitto/cocogitto/commit/da459decff9a84bb5bde471f5d64f577df28df11)) - [@Wassim-AB](https://github.com/Wassim-AB)
#### Documentation
- fix discord invite link - ([5d11d5a](https://github.com/cocogitto/cocogitto/commit/5d11d5aec58b4d88b2606c3f8b2ca4e7688590c2)) - [@oknozor](https://github.com/oknozor)
#### Features
- **(bump)** added skip untracked as cli argument - ([fb6a7e6](https://github.com/cocogitto/cocogitto/commit/fb6a7e667cdcddc185e02894b03b3b623610ca77)) - [@Wassim-AB](https://github.com/Wassim-AB)
#### Miscellaneous Chores
- edited contributors list - ([eca20b8](https://github.com/cocogitto/cocogitto/commit/eca20b8a4cd602225e5a386a06e1f22c39e4fddd)) - [@Wassim-AB](https://github.com/Wassim-AB)
- bump yanked crates - ([89149ad](https://github.com/cocogitto/cocogitto/commit/89149adfb9e45fc3b236fb2bf44d92843be30047)) - [@tranzystorek-io](https://github.com/tranzystorek-io)

- - -

## [5.5.0](https://github.com/cocogitto/cocogitto/compare/5.4.0..5.5.0) - 2023-08-17
#### Documentation
- **(README)** fix links to cocogitto docs - ([d62a83c](https://github.com/cocogitto/cocogitto/commit/d62a83c5613dc51a5b8201a0d2747493f148a758)) - [@lukehsiao](https://github.com/lukehsiao)
- update cog.toml example and contributing guidelines - ([ee09f87](https://github.com/cocogitto/cocogitto/commit/ee09f87c8b76b616279894d75c88b3400cf87762)) - [@oknozor](https://github.com/oknozor)
#### Features
- **(bump)** option to specify skip-ci pattern for bump commit - ([92736aa](https://github.com/cocogitto/cocogitto/commit/92736aaea536916b0650c6a26176bb1e23b7fa18)) - Wassim Ahmed-Belkacem
- implement `TryFrom` support for `Settings` for `String` and `&Repository` - ([b2d36aa](https://github.com/cocogitto/cocogitto/commit/b2d36aa39d37bf6491fe3074ada401480d7832a6)) - Mark S
#### Miscellaneous Chores
- **(compat)** make `SettingError` public - ([3e86732](https://github.com/cocogitto/cocogitto/commit/3e8673257dd7510cfe42dbe731e9cab05c958cc8)) - Mark S
- **(compat)** derive `Debug` for `ConventionalCommitError` - ([a666bca](https://github.com/cocogitto/cocogitto/commit/a666bca040381f6a5994649d38848e822c625c26)) - Mark S
- **(compat)** re-export `ConventionalCommitError` - ([2fe4d2c](https://github.com/cocogitto/cocogitto/commit/2fe4d2c55735d1281c6d16627766de3869118685)) - Mark S
- add SanchithHegde to contributors list - ([36d1ea5](https://github.com/cocogitto/cocogitto/commit/36d1ea5d94d2785d5019702c3d51c734436ce3b9)) - [@SanchithHegde](https://github.com/SanchithHegde)
- fix typo in comment - ([c981eb7](https://github.com/cocogitto/cocogitto/commit/c981eb7fb22e3c6d0a3726803c079a18cfe3750e)) - [@lukehsiao](https://github.com/lukehsiao)
- update cocogitto-action - ([dc6cfa2](https://github.com/cocogitto/cocogitto/commit/dc6cfa21d1b0b63e91b9c7cf66e4725136e4bf05)) - [@oknozor](https://github.com/oknozor)
- add commit-msg git-hook - ([64ddcf6](https://github.com/cocogitto/cocogitto/commit/64ddcf686aa553b35be08b99a1acae828befe6d6)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- **(revspec)** use git describe for finding latest tag - ([9a49999](https://github.com/cocogitto/cocogitto/commit/9a499990c66b4472d14a4a1e2f1ba9b6131d36ef)) - [@lukehsiao](https://github.com/lukehsiao)
- **(revspec)** raise error instead of panicking when parsing `RevspecPattern` - ([71ee6c6](https://github.com/cocogitto/cocogitto/commit/71ee6c67fb170ec535ee18aefd51cf142fc19911)) - [@SanchithHegde](https://github.com/SanchithHegde)
#### Style
- rename 'Cog' to 'cog' internally - ([9b0830e](https://github.com/cocogitto/cocogitto/commit/9b0830ef456a2713e31934a304339d36231b1df0)) - [@tranzystorek-io](https://github.com/tranzystorek-io)

- - -

## [5.4.0](https://github.com/cocogitto/cocogitto/compare/5.3.1..5.4.0) - 2023-06-23
#### Bug Fixes
- **(bump)** bump don't throw error on no bump types commits - ([4a6a8b3](https://github.com/cocogitto/cocogitto/commit/4a6a8b30aa4e9eddfe1b0a3fe0c7fd822e767447)) - Wassim Ahmed-Belkacem
- **(monorepo)** incorrect increment method used for package bumps - ([7bb3229](https://github.com/cocogitto/cocogitto/commit/7bb3229e356692dbf9eb932fdb5f42840414562e)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- **(formatting)** Apply cargo fmt - ([2710183](https://github.com/cocogitto/cocogitto/commit/2710183246d9f4bd43e3e6fa989f20f12cd57801)) - Mark S
- **(tests)** Add `no_coverage` support when using `llvm-cov` on nightly - ([97fd420](https://github.com/cocogitto/cocogitto/commit/97fd4208fc1dc483decfec6bc3ace85dc954c30b)) - Mark S
- remove android targets - ([1197d5f](https://github.com/cocogitto/cocogitto/commit/1197d5f98dc5e99f9bff82552b8dc813ab46ec33)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- Update manpage generation docs - ([4a09837](https://github.com/cocogitto/cocogitto/commit/4a09837244e070ff6168cd247ed5621b41f4264e)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Features
- **(bump)** support annotated tags - ([363387d](https://github.com/cocogitto/cocogitto/commit/363387df62050d96bd2988a11e827cb720487395)) - [@bitfehler](https://github.com/bitfehler)
- **(check)** allow running check on commit range - ([5168f75](https://github.com/cocogitto/cocogitto/commit/5168f75323ed14d34f497a7d14c7f2fa71db1693)) - Sanchith Hegde
- **(cli)** add file parameter to verify command - ([5e02aef](https://github.com/cocogitto/cocogitto/commit/5e02aefc6a77302f6bbe505425f731ebbc5214c6)) - sebasnabas
- **(cli)** Added get-version feature (#248) - ([5670cd8](https://github.com/cocogitto/cocogitto/commit/5670cd81a4127f2a125fde4b4eb9d38da3e121ed)) - [@andre161292](https://github.com/andre161292)
- **(commit)** execute commit hooks when running `cog commit` - ([bf38fa6](https://github.com/cocogitto/cocogitto/commit/bf38fa6454d038c4d175943f7c47aeea2a433ec8)) - [@oknozor](https://github.com/oknozor)
- **(monorepo)** add config opt to disable global global-tag-opt (#264) - ([96aa3b6](https://github.com/cocogitto/cocogitto/commit/96aa3b678845548cb50ef56c40948a86450d0ad0)) - [@oknozor](https://github.com/oknozor)
- Add configurable changelog omission for custom commit types - ([88f8742](https://github.com/cocogitto/cocogitto/commit/88f874220e058709ad4a1f2f2c35eb4a0d67dc4c)) - Mark S
- add custom git-hook installation - ([39cba74](https://github.com/cocogitto/cocogitto/commit/39cba74ba21c9679a03667e0fa8cbc6057bdf967)) - [@oknozor](https://github.com/oknozor)
- reorganize manpages generation - ([1509583](https://github.com/cocogitto/cocogitto/commit/1509583ca058d8e43e4d02e603d10d13248723b0)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- add {{package}} to hook version dsl - ([af08a7e](https://github.com/cocogitto/cocogitto/commit/af08a7e86c714fcdd0977d283d4887f3e98b6aa2)) - [@oknozor](https://github.com/oknozor)
- add version_tag and latest_tag to hook version dsl - ([9eaee5a](https://github.com/cocogitto/cocogitto/commit/9eaee5abcc16bd8fa06c50b83e1892dde126f38f)) - [@oknozor](https://github.com/oknozor)
#### Miscellaneous Chores
- **(template)** remove deprecated usage of json_pointer - ([7266575](https://github.com/cocogitto/cocogitto/commit/72665753acb129d709df38f385aadef1b06e17b7)) - [@oknozor](https://github.com/oknozor)
- bump clap to v4.2.4 for v1.69 clippy lints - ([3c259b6](https://github.com/cocogitto/cocogitto/commit/3c259b6345691d4aa37f38e61d85f6a0441563af)) - sebasnabas
- fix clippy default impl for TemplateKind - ([808632a](https://github.com/cocogitto/cocogitto/commit/808632ae4405fec2e8365dfa89803a0e220cacd7)) - sebasnabas
- rustfmt - ([c2e7ab0](https://github.com/cocogitto/cocogitto/commit/c2e7ab01d89cdf454028e53453fcce5c1f98bc0c)) - [@oknozor](https://github.com/oknozor)
- add bitfehler to contributors list - ([b9d351d](https://github.com/cocogitto/cocogitto/commit/b9d351df3b69c77857b4a7ce1bf78c8f22b50b2e)) - [@oknozor](https://github.com/oknozor)
- rename codesee token - ([0b82758](https://github.com/cocogitto/cocogitto/commit/0b82758078370786f032ff2ca6cfeaefc5e5a20a)) - [@oknozor](https://github.com/oknozor)
- add codesee workflow - ([548c76e](https://github.com/cocogitto/cocogitto/commit/548c76ec764f633968b62ab6c115763a093bcaf6)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- **(cli)** adjust cog-verify args - ([8a12065](https://github.com/cocogitto/cocogitto/commit/8a120650dc3aeacf1c55e11429592074f19174ec)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Revert
- **(partial)** revert addition of 'no_coverage' support and attributes - ([93c9903](https://github.com/cocogitto/cocogitto/commit/93c990322cccb71f27cf97d6c6bfac70cfca613c)) - Mark S
#### Style
- remove unused `CommitConfig::{omit,include}` methods - ([3ad69eb](https://github.com/cocogitto/cocogitto/commit/3ad69ebd487968e17822442bb171476766184dff)) - Mark S
#### Tests
- **(check)** add CLI tests for running check on commit range - ([e276bfa](https://github.com/cocogitto/cocogitto/commit/e276bfaf60590f6fb8c3fa04227b76eba8c31c64)) - Sanchith Hegde
- **(check)** add tests for running check on commit range - ([754e54d](https://github.com/cocogitto/cocogitto/commit/754e54d5904a487edf746271e997e983c33b4d08)) - Sanchith Hegde
- **(ci)** add test for configurable changelog omission - ([c1b070c](https://github.com/cocogitto/cocogitto/commit/c1b070cffd0b7fe17c22f826c48c382dbcc69b80)) - Mark S
- **(coverage)** add test for CommitConfig::{omit, include} methods - ([dd4461d](https://github.com/cocogitto/cocogitto/commit/dd4461db52f4e607d070c4c7e1dab53641f768af)) - Mark S
- - -

## [5.3.1](https://github.com/cocogitto/cocogitto/compare/5.3.0..5.3.1) - 2023-01-23
#### Bug Fixes
- **(monorepo)** fix package tag parsing - ([cdff4a1](https://github.com/cocogitto/cocogitto/commit/cdff4a10332fb4caf4299f5f368e5a794f862228)) - [@oknozor](https://github.com/oknozor)
- **(monorepo)** allow diffing orphan commits - ([02a1c5a](https://github.com/cocogitto/cocogitto/commit/02a1c5af617795f7da6a087d4373840f7bbba190)) - [@oknozor](https://github.com/oknozor)
- disable help subcommand for `cog(1)` - ([64df65b](https://github.com/cocogitto/cocogitto/commit/64df65b3399a890f2e598c046ccc729b37da301b)) - Orhun Parmaksız
#### Miscellaneous Chores
- clippy + fmt - ([6c7a2e7](https://github.com/cocogitto/cocogitto/commit/6c7a2e7b7ebe1fc051e5f16e73f87144a22dba8c)) - [@oknozor](https://github.com/oknozor)
- bump dev dependencies - ([e89c097](https://github.com/cocogitto/cocogitto/commit/e89c097b7bf4d2218a3cf817e522642976a74b47)) - [@oknozor](https://github.com/oknozor)
#### Tests
- remove deprecated chrono api usage - ([a43cc51](https://github.com/cocogitto/cocogitto/commit/a43cc51459fca240e6861d0fa8258c8a82ef7820)) - [@oknozor](https://github.com/oknozor)
- - -

## [5.3.0](https://github.com/cocogitto/cocogitto/compare/5.2.0..5.3.0) - 2023-01-22
#### Bug Fixes
- ignore merge commits based on parent counts - ([f8b5da6](https://github.com/cocogitto/cocogitto/commit/f8b5da64343e60ce5851e2e25b93611bfcf4db05)) - [@oknozor](https://github.com/oknozor)
- signing for chore commits - ([18b9643](https://github.com/cocogitto/cocogitto/commit/18b9643318cad85bae56a4217b04e3e684650d09)) - DaRacci
#### Documentation
- update 'pulic_api' doc - ([c9d5cbf](https://github.com/cocogitto/cocogitto/commit/c9d5cbf6207a80918c013b836a23d008fc7d8a41)) - [@oknozor](https://github.com/oknozor)
- add cargo-smart-release to the list of similar projects - ([3a04e72](https://github.com/cocogitto/cocogitto/commit/3a04e72231626af9717d4e8476e7e5af9c58a8ce)) - [@oknozor](https://github.com/oknozor)
- report one binary following `coco` deprecation - ([3ad6d28](https://github.com/cocogitto/cocogitto/commit/3ad6d28d2909f7a51ca384408550776a5e9b50be)) - [@lucatrv](https://github.com/lucatrv)
#### Features
- **(cli)** add subcommand for generating manpages - ([fe6bcfe](https://github.com/cocogitto/cocogitto/commit/fe6bcfe578c00be0ded6463c96c54e7392ee1635)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- monorepo support (#240) - ([d8eed3d](https://github.com/cocogitto/cocogitto/commit/d8eed3dc7eac644bf56dd1c4bd4dea0bb1d4f746)) - [@oknozor](https://github.com/oknozor)
- NuShell completions - ([3a356cc](https://github.com/cocogitto/cocogitto/commit/3a356ccb0e5838e5fe09fa56a18842fb417eddd7)) - [@DaRacci](https://github.com/DaRacci)
- add from_latest_tag to settings - ([a154782](https://github.com/cocogitto/cocogitto/commit/a154782639c899fc585b68ea28ce00d2f4bfefb9)) - [@stephenc](https://github.com/stephenc)
#### Miscellaneous Chores
- bump Cargo.lock - ([a43ab28](https://github.com/cocogitto/cocogitto/commit/a43ab2885f7a8db213aca0d2542d587b9a9c49e4)) - [@oknozor](https://github.com/oknozor)
- add new contributors to cog.toml - ([fb05641](https://github.com/cocogitto/cocogitto/commit/fb05641aeba90e948ba5078895fdd18c5fd556a8)) - [@oknozor](https://github.com/oknozor)
- bump dependencies and fix rustc-serialize cve - ([fc0e129](https://github.com/cocogitto/cocogitto/commit/fc0e129606911fde53b7256c2fda3b408b3484af)) - [@oknozor](https://github.com/oknozor)
- bump clap version to 4.0 - ([dbef47b](https://github.com/cocogitto/cocogitto/commit/dbef47b3fe69c9518dbc5757965c844c37f512fd)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Refactoring
- simplify project structure for binaries - ([941fb10](https://github.com/cocogitto/cocogitto/commit/941fb103453e311fe7a51f5cf9573e73e6ff1e80)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Tests
- hard code init branch to 'master' to avoid conflict with user config - ([d22482e](https://github.com/cocogitto/cocogitto/commit/d22482e4e2406a4db00d76c25cfcc7ea5e1579c2)) - [@oknozor](https://github.com/oknozor)
- - -

## [5.2.0](https://github.com/cocogitto/cocogitto/compare/5.1.0..5.2.0) - 2022-09-09
#### Bug Fixes
- fix stackover flow on 'oid..' resvpec - ([d4c3def](https://github.com/cocogitto/cocogitto/commit/d4c3defc6c6fbc4141edb3d1dda18be6b1f195c3)) - [@oknozor](https://github.com/oknozor)
- do not exit on parent not found (all_commits) - ([476d0d6](https://github.com/cocogitto/cocogitto/commit/476d0d6e98734878f9e1a8613eab8da1c5385490)) - Benoît CORTIER
#### Features
- **(commit)** implement signed commit - ([9b7c2d7](https://github.com/cocogitto/cocogitto/commit/9b7c2d73b3513d83feca7b37fd96314430d41882)) - [@oknozor](https://github.com/oknozor)
#### Miscellaneous Chores
- bump Cargo.lock - ([b7a55ef](https://github.com/cocogitto/cocogitto/commit/b7a55ef29ca7703c344f3beeeed033ec1e61f7ea)) - [@oknozor](https://github.com/oknozor)
- fix clippy lints on rust stable - ([cff1a15](https://github.com/cocogitto/cocogitto/commit/cff1a1526edb62dcb2d06786ab5dcb6226cb1323)) - [@oknozor](https://github.com/oknozor)
- fix clippy lints - ([991007c](https://github.com/cocogitto/cocogitto/commit/991007ceb18e3cb3e642f362f77ce2f3e8f4ffad)) - [@oknozor](https://github.com/oknozor)
- fix clippy lints - ([fe53d89](https://github.com/cocogitto/cocogitto/commit/fe53d891d75d6da197ef4c2f74eaf24a690107ef)) - [@oknozor](https://github.com/oknozor)
#### Tests
- **(commit)** add integration test for signed commit - ([ed3250c](https://github.com/cocogitto/cocogitto/commit/ed3250c356b2c7919155c9afcf7717821afae47a)) - [@oknozor](https://github.com/oknozor)
- - -

## [5.1.0](https://github.com/cocogitto/cocogitto/compare/5.0.1..5.1.0) - 2022-04-13
#### Bug Fixes
- **(verify)** remove trailing whitespace on cog verify messages - ([9a42c05](https://github.com/cocogitto/cocogitto/commit/9a42c056b6cdbe70e9645d0bbe0054623d9bf511)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- switch to stable channel for code coverage - ([60af851](https://github.com/cocogitto/cocogitto/commit/60af85103066f4004888fa67cbb24c6473b52ebf)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- update cli generated help - ([d9773ba](https://github.com/cocogitto/cocogitto/commit/d9773baee55860817d7c02a1dafe27e408e9bcfd)) - [@oknozor](https://github.com/oknozor)
- add install instructions for Void Linux - ([2e4ffd8](https://github.com/cocogitto/cocogitto/commit/2e4ffd8ce579ecea3622d8efb595d33632563782)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Features
- add a test for dry_run option - ([c9437bb](https://github.com/cocogitto/cocogitto/commit/c9437bb2d45daac34b2ea1c9f5157cf9b4997f34)) - [@darlaam](https://github.com/darlaam)
- use stderr log to differentiate app output (for dry_run, for example) and app infos - ([ad46106](https://github.com/cocogitto/cocogitto/commit/ad4610638952db8a09faaf9ba409854aab19c474)) - [@darlaam](https://github.com/darlaam)
- Add dry run for cog bump - ([6524f43](https://github.com/cocogitto/cocogitto/commit/6524f430147ae99fe7b71b9f34aaee4475d2788f)) - [@darlaam](https://github.com/darlaam)
#### Miscellaneous Chores
- add darlaam to the author list - ([04bba91](https://github.com/cocogitto/cocogitto/commit/04bba915e032c414a6d50df05cccba42926401ec)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- **(changelog)** use tera whitespace escapes instead of antislash - ([54b3d95](https://github.com/cocogitto/cocogitto/commit/54b3d957342f4c624fab116c61d8629beb304eee)) - [@oknozor](https://github.com/oknozor)
#### Tests
- fix failing tests - ([2507a68](https://github.com/cocogitto/cocogitto/commit/2507a68204dd8c593539e07cbe0acced9736be7e)) - [@darlaam](https://github.com/darlaam)
- - -

## [5.0.1](https://github.com/cocogitto/cocogitto/compare/5.0.0..5.0.1) - 2022-03-29
#### Bug Fixes
- **(hook)** correctly escape hook commands by removing shell-word usage - ([aaa9e78](https://github.com/cocogitto/cocogitto/commit/aaa9e78723f6a5f0ae7437258405d65cfb8827e2)) - [@oknozor](https://github.com/oknozor)
- avoid using the format ident capturing Rust feature - ([4c8c8ca](https://github.com/cocogitto/cocogitto/commit/4c8c8cae1378a2de7e4386ad9d03f194e011d4ac)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Continuous Integration
- ignore merge commits in cog checks - ([b6bda19](https://github.com/cocogitto/cocogitto/commit/b6bda199930bdacbefcc4e66defe27da1c121438)) - [@oknozor](https://github.com/oknozor)
- update cocogitto action and add branch whitelist to cog.toml - ([45f93c2](https://github.com/cocogitto/cocogitto/commit/45f93c2a753dad8ba2007a3b61e81689e1ad4b9c)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- add a contribution guide - ([905d0a3](https://github.com/cocogitto/cocogitto/commit/905d0a32c1c504f58106a5d4998e66ad180696a9)) - [@oknozor](https://github.com/oknozor)
- - -

## [5.0.0](https://github.com/cocogitto/cocogitto/compare/4.1.0..5.0.0) - 2022-03-09
#### Bug Fixes
- **(changelog)** allow template context to be used in custom templates - ([974587c](https://github.com/cocogitto/cocogitto/commit/974587c9b3d41e9ff57c9ff271c6d3d35bc0f962)) - [@oknozor](https://github.com/oknozor)
- **(error)** restore parse error formatting - ([71bec5d](https://github.com/cocogitto/cocogitto/commit/71bec5d8dc3b664475f7c2cd619630a7f0b90f66)) - [@oknozor](https://github.com/oknozor)
- updated config to 0.12.0 (latest as of now) + usage - ([fc4d1a3](https://github.com/cocogitto/cocogitto/commit/fc4d1a3dfdedf55ec65c26f74fada8869700bfd2)) - Kristof Mattei
- check allowed commit type when using 'cog verify' - ([a57de18](https://github.com/cocogitto/cocogitto/commit/a57de18b25a8be350db55304d226ca8d68771527)) - [@oknozor](https://github.com/oknozor)
- make env var error more explicit - ([22120dc](https://github.com/cocogitto/cocogitto/commit/22120dcbfa35144e50999500940ddaacba68086e)) - [@oknozor](https://github.com/oknozor)
- build commit type error regardless of the command used - ([3685651](https://github.com/cocogitto/cocogitto/commit/36856519b432fcd523294c3b1dea8e19e07c493e)) - [@oknozor](https://github.com/oknozor)
- add missing ';' in `run_cmd!` calls - ([ce6b70f](https://github.com/cocogitto/cocogitto/commit/ce6b70f83e0f352b5f25721de5a5d53cbdf100c3)) - [@Zshoham](https://github.com/Zshoham)
- correctly identify empty repository when commiting - ([c442f07](https://github.com/cocogitto/cocogitto/commit/c442f0792fc4497cf65b92dad9ca598fead1c0a3)) - [@Zshoham](https://github.com/Zshoham)
#### Continuous Integration
- **(codecov)** fix codecov threshhold - ([f992c21](https://github.com/cocogitto/cocogitto/commit/f992c2196dc88fe1f126802bd880cafd3eddda64)) - [@oknozor](https://github.com/oknozor)
- **(coverage)** set coverage target threshold to 1% - ([bf2eb5d](https://github.com/cocogitto/cocogitto/commit/bf2eb5d6f67c67acc69e1e5029e967abe28bc884)) - [@oknozor](https://github.com/oknozor)
- update code coverage action - ([8c13a45](https://github.com/cocogitto/cocogitto/commit/8c13a45f48862d4e224e957168426e55998e2c26)) - [@oknozor](https://github.com/oknozor)
- use keyword with shorter lenght to comply with crates.io rules - ([cd847de](https://github.com/cocogitto/cocogitto/commit/cd847de37cda51950e65d4b4868b8faa6ffcd880)) - [@oknozor](https://github.com/oknozor)
- remove cargo manifest keyword to comply with crates.io max keyword rule - ([896bcb3](https://github.com/cocogitto/cocogitto/commit/896bcb30b93e47b65e4885e5f6e1e748ab9cdfae)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- replace AUR badges with repology in the installation section - ([466dffe](https://github.com/cocogitto/cocogitto/commit/466dffeb187fd117e423559974a914e991f894e8)) - [@oknozor](https://github.com/oknozor)
- add discord badge to readme - ([e8eef80](https://github.com/cocogitto/cocogitto/commit/e8eef8024c238be5e1873236ea069936ffaf47e1)) - [@oknozor](https://github.com/oknozor)
- use the `cog commit` command - ([ab50816](https://github.com/cocogitto/cocogitto/commit/ab508165533ada56aab79d77da619e901793dc6e)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- add coco deprecation notice to README - ([8e8f7b2](https://github.com/cocogitto/cocogitto/commit/8e8f7b276c2f932f321ff0fba104bbad2598faa7)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- suggest running a locked install when using cargo - ([80a4737](https://github.com/cocogitto/cocogitto/commit/80a4737359d0452e6dcc24331c775efab2ac0c12)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- nixos install onliner - ([22abfd3](https://github.com/cocogitto/cocogitto/commit/22abfd3276c91e47dd02d06f6e2b044abbbcc7a8)) - Travis Davis
#### Features
- **(bump)** use glob pattern for branch whitelist - ([654baa9](https://github.com/cocogitto/cocogitto/commit/654baa959257add03d292bb5fd12ab14d3704076)) - [@oknozor](https://github.com/oknozor)
- **(hook)** use a subshell by default on linux - ([fe47333](https://github.com/cocogitto/cocogitto/commit/fe47333de7c63af966e24ea3d0c9c199573008cf)) - [@oknozor](https://github.com/oknozor)
- **(verify)** add the ability to forbid merge commit via configuration - ([d932d91](https://github.com/cocogitto/cocogitto/commit/d932d91d786708069286bd2c7cc81c7d2a366185)) - [@oknozor](https://github.com/oknozor)
- add branch whitelist to cog bump #165 - ([d78071a](https://github.com/cocogitto/cocogitto/commit/d78071a190b8f85e290f02904d19a71a4432197d)) - [@oknozor](https://github.com/oknozor)
#### Miscellaneous Chores
- remove tarpaulin conditional config - ([a8484ac](https://github.com/cocogitto/cocogitto/commit/a8484acb909e92dba4ca368d78f9b64eb532f6c5)) - [@oknozor](https://github.com/oknozor)
- remove deprecated coco utility - ([2c10235](https://github.com/cocogitto/cocogitto/commit/2c10235220cdf4f09c3c5a30631143683448ba98)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- use cargo-edit for bumping crate version - ([6cb774e](https://github.com/cocogitto/cocogitto/commit/6cb774ee8daeb56a6171725cc2c4f57dc578201b)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- update clap and config - ([421ef1f](https://github.com/cocogitto/cocogitto/commit/421ef1f70f22bf2196c247120d3a73db19c2aecf)) - [@oknozor](https://github.com/oknozor)
- clippy lints - ([cc961d3](https://github.com/cocogitto/cocogitto/commit/cc961d3360595a89cf0131519ae63078435dadfa)) - [@oknozor](https://github.com/oknozor)
- use rust 2021 edition - ([db8cfb5](https://github.com/cocogitto/cocogitto/commit/db8cfb581d954148d7109d80f61a356892b13c39)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- **(commit)** improve git statuses error output on commit - ([cbdad51](https://github.com/cocogitto/cocogitto/commit/cbdad511a489a2100305f0400ca1b328ba00b94e)) - [@oknozor](https://github.com/oknozor)
- **(error)** display underlying errors when possible - ([41053e2](https://github.com/cocogitto/cocogitto/commit/41053e2732b3e015a86a3a7fdf53032292131365)) - [@oknozor](https://github.com/oknozor)
- **(error)** remove anyhow from private API - ([2346112](https://github.com/cocogitto/cocogitto/commit/23461123e45dc1933ea4472b28a045d1d81ce4dc)) - [@oknozor](https://github.com/oknozor)
#### Tests
- fix sealed_test question mark unwrapping - ([a138911](https://github.com/cocogitto/cocogitto/commit/a1389119d033de6d65b59251a08c48424558f9ba)) - [@oknozor](https://github.com/oknozor)
- - -

## [4.1.0](https://github.com/cocogitto/cocogitto/compare/4.0.1..4.1.0) - 2022-01-18
#### Bug Fixes
- **(parser)** bump parser to 0.9.4 to support windows escape sequences in commit footers - ([415ec37](https://github.com/cocogitto/cocogitto/commit/415ec37921500bc375eb27b70722352fcd11d53b)) - [@oknozor](https://github.com/oknozor)
- support annotated tags in cog check -l - ([66faca2](https://github.com/cocogitto/cocogitto/commit/66faca2312282108e50746cf2df1b8831e304122)) - [@lukehsiao](https://github.com/lukehsiao)
- ignore comment lines in cog verify - ([2f25f5e](https://github.com/cocogitto/cocogitto/commit/2f25f5ea6e9b96882a11a271aad20db0ee420ffb)) - [@lukehsiao](https://github.com/lukehsiao)
#### Continuous Integration
- add cargo-bump and crates.io token - ([17fb92e](https://github.com/cocogitto/cocogitto/commit/17fb92eccde86a749e4df5d92382d92a03acd1e1)) - [@oknozor](https://github.com/oknozor)
- trigger release from github action workflow dispatch - ([ab146d0](https://github.com/cocogitto/cocogitto/commit/ab146d0842d03c90fe8abc82af76f0ecf1b6e441)) - [@oknozor](https://github.com/oknozor)
- fix code coverage breakage with latest rust nightly - ([cd76427](https://github.com/cocogitto/cocogitto/commit/cd7642729c2a56a091dd1497b37707ae8723e236)) - [@oknozor](https://github.com/oknozor)
- remove codevov artifact upload - ([f7386c1](https://github.com/cocogitto/cocogitto/commit/f7386c1f4fd3aafe3d310cf5e11a81a4361c4d70)) - [@oknozor](https://github.com/oknozor)
- add rust-cache action - ([f8dff56](https://github.com/cocogitto/cocogitto/commit/f8dff56213861fb6ea1696268ba3cb70b1602404)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- **(readme)** update Arch Linux noting official package - ([79c1608](https://github.com/cocogitto/cocogitto/commit/79c16082407ff076852e3e8c5d06f212a5bae3fb)) - [@alerque](https://github.com/alerque)
- **(readme)** fix language issues and typos - ([9d37de3](https://github.com/cocogitto/cocogitto/commit/9d37de36cabb21f0dbb70bb3106c21f234ce9472)) - Lucas Declercq
- **(readme)** change some url to the org - ([573ef81](https://github.com/cocogitto/cocogitto/commit/573ef81dc6dd56c42ce7fe9e7916ce26324a3feb)) - [@oknozor](https://github.com/oknozor)
- adjust `cog bump` help wording - ([ef032d5](https://github.com/cocogitto/cocogitto/commit/ef032d55d007092655e5368eeecd95a99f2e03dd)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- add crates.io keywords and categories - ([7fcb0bd](https://github.com/cocogitto/cocogitto/commit/7fcb0bdd452880150e920521ab190e7c61835060)) - [@oknozor](https://github.com/oknozor)
#### Features
- **(cli)** add `cog commit` as duplicate of `coco` - ([128b9d0](https://github.com/cocogitto/cocogitto/commit/128b9d0e07d094ae004c0ee75a3f98182aabe983)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Miscellaneous Chores
- add @calerque to the list of contributors - ([50063e1](https://github.com/cocogitto/cocogitto/commit/50063e1d1540809d8e8ef22b30768f441dca5bb2)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- group `cog bump` flags together - ([337c7cc](https://github.com/cocogitto/cocogitto/commit/337c7ccd15d7e19a34582841dd0c2741403dc863)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- switch arg parsing to clap v3 - ([13ebca5](https://github.com/cocogitto/cocogitto/commit/13ebca567291f3d38d73eaf76d524ba00666b921)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Style
- **(logo)** add the definitive logo and visual identities - ([e9b404e](https://github.com/cocogitto/cocogitto/commit/e9b404eb3f33eb847484aa6e9d5170a7f565f947)) - [@oknozor](https://github.com/oknozor)
#### Tests
- duplicate coco tests for cog commit - ([3a3249c](https://github.com/cocogitto/cocogitto/commit/3a3249cd0167adc5183f2a384155d5e1120a500d)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- use sealed_test to run test in parallel - ([6f3ef7f](https://github.com/cocogitto/cocogitto/commit/6f3ef7f972ad4fa98e1922676cbde530b4adc3b1)) - [@oknozor](https://github.com/oknozor)
- fix IT test failing when HEAD==latest tag - ([fb4a294](https://github.com/cocogitto/cocogitto/commit/fb4a2945eec1a2b5d76cea9803f673886e99d5bd)) - [@oknozor](https://github.com/oknozor)
- - -

## [4.0.1](https://github.com/cocogitto/cocogitto/compare/4.0.0..4.0.1) - 2021-11-30
#### Bug Fixes
- **(bump)** correctly generate tag with prefix - ([9e8d592](https://github.com/cocogitto/cocogitto/commit/9e8d592d58f0200590ec3ac3d2b5c2b2c1720a06)) - [@oknozor](https://github.com/oknozor)
#### Tests
- fix test failing on HEAD->latest - ([d597208](https://github.com/cocogitto/cocogitto/commit/d59720866671d8076088979a9425a74af949a193)) - [@oknozor](https://github.com/oknozor)
- - -

## [4.0.0](https://github.com/cocogitto/cocogitto/compare/3.0.0..4.0.0) - 2021-11-30
#### Bug Fixes
- **(bump)** fix target changelog tag on bump - ([0618192](https://github.com/cocogitto/cocogitto/commit/0618192b5e2866b7e12ac6d036df700041194506)) - [@oknozor](https://github.com/oknozor)
- **(changelog)** correctly pick tagged HEAD commit - ([9b5a591](https://github.com/cocogitto/cocogitto/commit/9b5a591a9c67a2fbdffdcbeff296932858ba49b3)) - [@oknozor](https://github.com/oknozor)
- **(hook)** use pre-commit instead of prepare-commit-message hook - ([6fe1a27](https://github.com/cocogitto/cocogitto/commit/6fe1a279a45b1f6452a23039dece95cd508b03bd)) - [@oknozor](https://github.com/oknozor)
- **(scope)** add support for multiple version placeholder and buidmetatata in hooks [#117] - ([43eba56](https://github.com/cocogitto/cocogitto/commit/43eba56766e697a0fcdb43e63835bbc608552d22)) - [@oknozor](https://github.com/oknozor)
- change git commit hook type to allow use of '--no-verify' - ([c4516b7](https://github.com/cocogitto/cocogitto/commit/c4516b77d51fce36935e6d0786fc8055557bb423)) - [@oknozor](https://github.com/oknozor)
- make footer serialization and deserialization symmetric - ([04befc1](https://github.com/cocogitto/cocogitto/commit/04befc1969e3700e33a84259a34732773577dcc0)) - [@oknozor](https://github.com/oknozor)
- fix version increment regression #129 - ([4372b57](https://github.com/cocogitto/cocogitto/commit/4372b57a8431c5b0bdacd6aa1e288719f8585dc0)) - [@oknozor](https://github.com/oknozor)
- display parse error corectly on cog verify - ([618499e](https://github.com/cocogitto/cocogitto/commit/618499ef99e1948b60c8fbe5f2a3adbc0089cb92)) - [@oknozor](https://github.com/oknozor)
- display hook-profiles value in cli and safe check commit type - ([fa59679](https://github.com/cocogitto/cocogitto/commit/fa59679e50b6f81e7125e82c2b2351f1a9c2c659)) - [@oknozor](https://github.com/oknozor)
- fix unicode trailing char panic [#101] - ([3de62ba](https://github.com/cocogitto/cocogitto/commit/3de62ba02273e6714e08374575b55788ceb4483e)) - [@oknozor](https://github.com/oknozor)
- fix typo in git hooks error messages - ([6d8bdb5](https://github.com/cocogitto/cocogitto/commit/6d8bdb5f2e6c894e326cc5439f6b5a22997f9d9a)) - [@cpoissonnier](https://github.com/cpoissonnier)
- generate completions without opening a repository - ([eaf63bb](https://github.com/cocogitto/cocogitto/commit/eaf63bb33b94a18bc50cbab9ee5b596ede24f01c)) - [@orhun](https://github.com/orhun)
- remove Cargo.lock from gitignore [#109] - ([ffe7f0d](https://github.com/cocogitto/cocogitto/commit/ffe7f0dc3a15cc52911e572c469827c598ee60b5)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- fix prebump hooks ordering - ([ab3f841](https://github.com/cocogitto/cocogitto/commit/ab3f84118b9b2637a0ae5d0725d755575ea8bf06)) - [@oknozor](https://github.com/oknozor)
- update codecov action - ([ae66d91](https://github.com/cocogitto/cocogitto/commit/ae66d91968987b0f6a5f1008e88260388172b703)) - [@oknozor](https://github.com/oknozor)
- move cog check action to CI - ([4616065](https://github.com/cocogitto/cocogitto/commit/4616065360b51f8536f3f238553ba6317e191c3d)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- **(readme)** fix pre_bump_hooks example - ([81ad844](https://github.com/cocogitto/cocogitto/commit/81ad8446ac018caff94ef4369c8049b30c012b6e)) - [@its-danny](https://github.com/its-danny)
- add temporary logo - ([99c39a8](https://github.com/cocogitto/cocogitto/commit/99c39a8f85d0db4b7e182b0f5e54b2d33223350b)) - [@oknozor](https://github.com/oknozor)
- update readme links - ([c3a3143](https://github.com/cocogitto/cocogitto/commit/c3a3143a8d3a450aeaf19bc91ea7bc1d2321116c)) - [@oknozor](https://github.com/oknozor)
- update readme - ([a2d9268](https://github.com/cocogitto/cocogitto/commit/a2d9268c8abeeecaf504eb77631356e86659af3a)) - [@oknozor](https://github.com/oknozor)
- document get_conventional_message - ([72b2722](https://github.com/cocogitto/cocogitto/commit/72b27229fb28b4a7a984815c3987df02ccd673fd)) - [@its-danny](https://github.com/its-danny)
- fix typo in README (#126) - ([551dc32](https://github.com/cocogitto/cocogitto/commit/551dc326ee5baf586bd26e49d5f1be35135b7f02)) - Jean-Philippe Bidegain
#### Features
- **(changelog)** populate template with tag oid - ([679928f](https://github.com/cocogitto/cocogitto/commit/679928f4f1913bd073693d9cafb2d53eef5ae418)) - [@oknozor](https://github.com/oknozor)
- **(changelog)** display multiple release changelog on changelog range - ([c6940c5](https://github.com/cocogitto/cocogitto/commit/c6940c524880265b55941fad20aac46aa5b79305)) - [@oknozor](https://github.com/oknozor)
- **(changelog)** add full_hash changelog template - ([10ab5c6](https://github.com/cocogitto/cocogitto/commit/10ab5c6dc0cce8daa37a17672fec0fcf77679ec9)) - [@oknozor](https://github.com/oknozor)
- **(changelog)** add custom template - ([ad2bcd2](https://github.com/cocogitto/cocogitto/commit/ad2bcd2ea3bd1bd1fe936324c1b62d21d005d0ac)) - [@oknozor](https://github.com/oknozor)
- **(changelog)** implement changelog template - #72 - ([56bbff7](https://github.com/cocogitto/cocogitto/commit/56bbff7a2e7fbfe863ddda199f91c770a5597f78)) - [@oknozor](https://github.com/oknozor)
- **(cli)** improve commit format error format - ([78dea00](https://github.com/cocogitto/cocogitto/commit/78dea00088c7e359164f09d22af1d29a8b468e57)) - [@oknozor](https://github.com/oknozor)
- **(coco)** add edit flag for opening message in editor - ([2b62de3](https://github.com/cocogitto/cocogitto/commit/2b62de3d23a5290dcf49571ceeea60509a09e65b)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- **(cog)** add from latest tag flag to cog edit - ([f391df6](https://github.com/cocogitto/cocogitto/commit/f391df65d0e165a742d3fcff68183261cc5f5836)) - [@oknozor](https://github.com/oknozor)
- **(hook)** add bump profiles configuration - ([13eeed9](https://github.com/cocogitto/cocogitto/commit/13eeed983cf2b44ebf686f4b9d7f86626792a1ff)) - [@oknozor](https://github.com/oknozor)
- **(tag)** add configurable tag prefix as described in #122 - ([38f9eab](https://github.com/cocogitto/cocogitto/commit/38f9eab1f772980dc1b81241dfce12b0cd290951)) - [@oknozor](https://github.com/oknozor)
- use revspec instead of 'from' annd 'to' flag for changelog - ([ce24789](https://github.com/cocogitto/cocogitto/commit/ce247898b99f0b5b5fbf93c235b51543224e3e50)) - [@oknozor](https://github.com/oknozor)
- add get_conventional_message fn to return the prepared message without committing - ([4668622](https://github.com/cocogitto/cocogitto/commit/46686226fc4c950f285c2d4c525a89967326ca63)) - [@its-danny](https://github.com/its-danny)
- improve cli message format and fix #97 - ([d0bb0d4](https://github.com/cocogitto/cocogitto/commit/d0bb0d4e5a36ab19f9f09ab68fdaed336e43ba89)) - [@oknozor](https://github.com/oknozor)
- add {{latest}} tag to hook dsl - ([5eff372](https://github.com/cocogitto/cocogitto/commit/5eff372bf1aaaa0458f2a382abadeeeb560ee6f5)) - [@oknozor](https://github.com/oknozor)
#### Miscellaneous Chores
- **(settings)** deny unknown settings fields - ([8cf426a](https://github.com/cocogitto/cocogitto/commit/8cf426a665c025288817546520abcd81c73b3918)) - [@oknozor](https://github.com/oknozor)
- use MIT license in cargo.toml - ([ffbab13](https://github.com/cocogitto/cocogitto/commit/ffbab13819681a4be494f73db2519173c7834e84)) - [@oknozor](https://github.com/oknozor)
- remove aur package submodule #141 - ([6a030ca](https://github.com/cocogitto/cocogitto/commit/6a030ca2dec7a53c35cbf14cf1b92c52f8853007)) - [@oknozor](https://github.com/oknozor)
- update default branch name and remote - ([477b6ac](https://github.com/cocogitto/cocogitto/commit/477b6ac50e2b489c6c85b23299d2fa08b2920af7)) - [@oknozor](https://github.com/oknozor)
- update conventional commit parser - ([9c52c23](https://github.com/cocogitto/cocogitto/commit/9c52c23a93a51d5468cb4ffcfabef05185b38b40)) - [@oknozor](https://github.com/oknozor)
- add myself to the contributor list - ([aa002ba](https://github.com/cocogitto/cocogitto/commit/aa002bac405f43d8ff9f84300ba41e5d8f17f45e)) - [@its-danny](https://github.com/its-danny)
- remove spaces before column everywhere - ([3f21a55](https://github.com/cocogitto/cocogitto/commit/3f21a55a0ba662f3bea5f2be73e1af3a9d6d5c4c)) - [@oknozor](https://github.com/oknozor)
- bump semver to v1 - ([3d8db7d](https://github.com/cocogitto/cocogitto/commit/3d8db7df0e1003e220ce0a94dc34ead6a64e12fe)) - [@oknozor](https://github.com/oknozor)
- add cpoissonnier to the contributors list - ([b55c3ee](https://github.com/cocogitto/cocogitto/commit/b55c3ee83501258873900666b9c1f59799d5410d)) - [@cpoissonnier](https://github.com/cpoissonnier)
- add github sponsor - ([f2175fc](https://github.com/cocogitto/cocogitto/commit/f2175fc963fca5bc445f396ff26d043641bd1012)) - [@oknozor](https://github.com/oknozor)
- add orhun to the contributors list (#113) - ([b68f1fd](https://github.com/cocogitto/cocogitto/commit/b68f1fd0af11e6ee4b02ebbfabee8b116295ce77)) - Orhun Parmaksız
#### Performance Improvements
- remove unused default features from the config crate - ([8e069e8](https://github.com/cocogitto/cocogitto/commit/8e069e8dd41499f49e28192262158e637af29cc5)) - [@oknozor](https://github.com/oknozor)
- add binary size optimizations - ([6227ca6](https://github.com/cocogitto/cocogitto/commit/6227ca626ed1f607d3a5215c5677e9fbf772baeb)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- **(errors)** remove thiserror crate and rework error format - ([5b00f9e](https://github.com/cocogitto/cocogitto/commit/5b00f9ed1ac5d4cd429cb6eb80392cf41d22a6e7)) - [@oknozor](https://github.com/oknozor)
- **(git)** split repository in multiple modules - ([5ce5187](https://github.com/cocogitto/cocogitto/commit/5ce518730c225681a899326e4ceb224e132dd7d7)) - [@oknozor](https://github.com/oknozor)
- **(git-hooks)** rename prepare-commit hook asset - ([a693d6f](https://github.com/cocogitto/cocogitto/commit/a693d6f4a135b38c9b886cf5e4eb177acb153602)) - [@oknozor](https://github.com/oknozor)
- use git2 revspec instead of home made lookup - ([49e79d1](https://github.com/cocogitto/cocogitto/commit/49e79d1180f51f86fab7a60aaf6fdf94fdbd1fd1)) - [@oknozor](https://github.com/oknozor)
- organize imports and dependencies - ([807d033](https://github.com/cocogitto/cocogitto/commit/807d033d9f5f433959ffa1b2575499c894e0e118)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
- remove some lazy static constants - ([dd26c84](https://github.com/cocogitto/cocogitto/commit/dd26c84373be21ce676da7dc477bf4cad0043f17)) - [@oknozor](https://github.com/oknozor)
- use matches expression instead of if lets - ([7b7e469](https://github.com/cocogitto/cocogitto/commit/7b7e4695acb5939a6a89843e80d7f569e2f24299)) - [@oknozor](https://github.com/oknozor)
- switch coco arg parsing to structopt - ([5b185d8](https://github.com/cocogitto/cocogitto/commit/5b185d8d0c9accfe056e38f40359dc90cf6f81da)) - [@tranzystorek-io](https://github.com/tranzystorek-io)
#### Tests
- **(changelog)** add github template test - ([e1f219b](https://github.com/cocogitto/cocogitto/commit/e1f219bb891fcb6967fb005156bc379fd546ce36)) - [@oknozor](https://github.com/oknozor)
- **(coco)** fix failing test [#103] - ([14cbc8d](https://github.com/cocogitto/cocogitto/commit/14cbc8d744613d13d92c06fe67bd6f026b50d4a3)) - [@oknozor](https://github.com/oknozor)
- fix changelog date in test - ([fbe02bb](https://github.com/cocogitto/cocogitto/commit/fbe02bb037f95d78299644cac98068ae1670f4fc)) - [@oknozor](https://github.com/oknozor)
- add write_file test helper - ([ad41a5f](https://github.com/cocogitto/cocogitto/commit/ad41a5f3f6ead623d5f3f2524629e0c426a130be)) - [@oknozor](https://github.com/oknozor)
- fix test failing on a colored VTE - ([3922623](https://github.com/cocogitto/cocogitto/commit/3922623b1effe6e589ceb603c959fabde01417df)) - [@oknozor](https://github.com/oknozor)
- ignore binaries from coverage - ([8ee7a2e](https://github.com/cocogitto/cocogitto/commit/8ee7a2e19f78bcffea0a6fffa868011b31b3c9d4)) - [@oknozor](https://github.com/oknozor)
- use fluent assertions - ([ce9882c](https://github.com/cocogitto/cocogitto/commit/ce9882c6042f729573e628406019567d4e46180f)) - [@oknozor](https://github.com/oknozor)
- add init tests - ([a9fc5f3](https://github.com/cocogitto/cocogitto/commit/a9fc5f36ec82aa41f2bfb51c151dc1fe0c5d016a)) - [@oknozor](https://github.com/oknozor)
- refactor test modules structure - ([abc8f01](https://github.com/cocogitto/cocogitto/commit/abc8f018816cd2857c9a84eb950945810eb7b46d)) - [@oknozor](https://github.com/oknozor)
- remove panic unwind from test helper - ([738ed8a](https://github.com/cocogitto/cocogitto/commit/738ed8a944b0d27f700d6d0cc3429b08d3fdd719)) - [@oknozor](https://github.com/oknozor)
- make test helper module  public - ([205397b](https://github.com/cocogitto/cocogitto/commit/205397b39013845c1da002926f2120ab28057dbf)) - [@oknozor](https://github.com/oknozor)
- refactor IT test using a run in context helper - ([a32a517](https://github.com/cocogitto/cocogitto/commit/a32a517265a9755696bdbed2250a14accdfe703e)) - [@oknozor](https://github.com/oknozor)
- - -

## 3.0.0 - 2021-09-13


### Bug Fixes

[2f95cf](https://github.com/oknozor/cocogitto/commit/2f95cf805df2d354785f5f0be426aade3689b44b) - validate footers on commit - [oknozor](https://github.com/oknozor)

[4bdcb3](https://github.com/oknozor/cocogitto/commit/4bdcb3d7697476dfca40bd9e8eabd6cf3f9adb27) - parse commit message before creating a commit - [oknozor](https://github.com/oknozor)

[4f5bd9](https://github.com/oknozor/cocogitto/commit/4f5bd95716e3ccbb05f9c8e313b71d0e99eaaf07) - sort tag names before searching - [tranzystorek-io](https://github.com/tranzystorek-io)


### Tests

[1ea9d0](https://github.com/oknozor/cocogitto/commit/1ea9d0f2d88087fff7d28d6661a9f1ad66d9da25) - ignore test helpers in coverage results - [oknozor](https://github.com/oknozor)


### Features

[53f23d](https://github.com/oknozor/cocogitto/commit/53f23d9ad21d5d165ed21c60fe18e37ea6e14203) - use conventional commit parser instead of custom implementation - [oknozor](https://github.com/oknozor)


### Continuous Integration

[434c22](https://github.com/oknozor/cocogitto/commit/434c22295390fda0f276e3a3ee32fa4658489c5d) - use cocogitto github action - [oknozor](https://github.com/oknozor)


### Refactoring

[4379e2](https://github.com/oknozor/cocogitto/commit/4379e2f1111cdce03f2d6ce9d31228045df550af) - remove useless fonction to access metadata - [oknozor](https://github.com/oknozor)

[f7c639](https://github.com/oknozor/cocogitto/commit/f7c639ea72c142a1ffbc8e613693384d8cc0a7c5) - refactor test helpers - [oknozor](https://github.com/oknozor)

[8b2aaf](https://github.com/oknozor/cocogitto/commit/8b2aafa10606ee69622ee7ed417d692b3013cef9) - clean up minor code details - [tranzystorek-io](https://github.com/tranzystorek-io)


- - -
## 2.1.1 - 2021-07-17


### Bug Fixes

[acf354](https://github.com/oknozor/cocogitto/commit/acf354b554d9c7cdc8284abecfc65e9c61006db0) - use range to replace version expression in hooks - [oknozor](https://github.com/oknozor)


- - -
## 2.1.0 - 2021-07-16


### Features

[53f5a9](https://github.com/oknozor/cocogitto/commit/53f5a91b0059917e9e9b1593b7449fb07f4aa0ad) - add check from latest tag option - [oknozor](https://github.com/oknozor)

[caa6ec](https://github.com/oknozor/cocogitto/commit/caa6ec31abdcf0f9115204c968eb01ec571abdcc) - add check from latest tag option - [oknozor](https://github.com/oknozor)


### Tests

[a27c74](https://github.com/oknozor/cocogitto/commit/a27c749d75cbe0bc37d91cf5a4e2391ceee77bfc) - add missing test for changelog and check commands - [oknozor](https://github.com/oknozor)


### Documentation

[80e488](https://github.com/oknozor/cocogitto/commit/80e4887e342b5808631cbeb287ca05f6a3b30c89) - fix typo in completion commands - [oknozor](https://github.com/oknozor)


- - -
## 2.0.0 - 2021-03-18


### Continuous Integration

[7fb50e](https://github.com/oknozor/cocogitto/commit/7fb50ed991917581178c121acfb78327a28e384e) - use tarpaulin 0.16 to fix build before next cargo release - [oknozor](https://github.com/oknozor)


### Features

[edf610](https://github.com/oknozor/cocogitto/commit/edf610e8cab474b93017faf6404b11e8e4fa1093) - add version DSL in cog.toml - [oknozor](https://github.com/oknozor)


### Refactoring

[4c7442](https://github.com/oknozor/cocogitto/commit/4c74429e9cbab88fce0f73c73332102feae48963) - do some general code clean-up - [tranzystorek-io](https://github.com/tranzystorek-io)


### Miscellaneous Chores

[4fc160](https://github.com/oknozor/cocogitto/commit/4fc160e02b74d68d5c807a2d116fc7912a61bae3) - update cog.toml - [oknozor](https://github.com/oknozor)

[ba9bfd](https://github.com/oknozor/cocogitto/commit/ba9bfd699f2ec4d9ad6c6ecbd39deaf9c484c317) - remove temp_test_dir dep - [oknozor](https://github.com/oknozor)

[edf667](https://github.com/oknozor/cocogitto/commit/edf667f9f6a9173ac5581b93474f32d7e5fba29b) - bump assert_cmd crate - [oknozor](https://github.com/oknozor)


- - -
## 1.2.0 - 2021-01-19


### Features

[635a04](https://github.com/oknozor/cocogitto/commit/635a043c04a37a4b355b473759ab4071e91530cf) - add --config flag to cog bump - [renaultfernandes](https://github.com/renaultfernandes)


### Bug Fixes

[3c4f60](https://github.com/oknozor/cocogitto/commit/3c4f60138e5eaf38ff317d4b7a8e0878ae6b7a34) - get latest tag based on semver format - [pjvds](https://github.com/pjvds)


### Tests

[2a7da2](https://github.com/oknozor/cocogitto/commit/2a7da281e45fcd014cdc38c8274d458baeee2e10) - fix failing test - [oknozor](https://github.com/oknozor)

[968592](https://github.com/oknozor/cocogitto/commit/9685924a2f76cd7829892b034d7e630eb4bf7bda) - add test to test semver sorting for auto bump - [pjvds](https://github.com/pjvds)


### Continuous Integration

[84b334](https://github.com/oknozor/cocogitto/commit/84b334a4c4dae4fa25acd0475e7a311c74c0135e) - fix deprecated set env and add path github action commands - [oknozor](https://github.com/oknozor)

[ac87ba](https://github.com/oknozor/cocogitto/commit/ac87babab51aef2f24e71d4bc684694b2eb65126) - add aur package submodule - [oknozor](https://github.com/oknozor)


### Miscellaneous Chores

[764739](https://github.com/oknozor/cocogitto/commit/7647397ce681909272b21bbc5c5f27ca43dc9ec5) - add pjvds to the contributors list - [oknozor](https://github.com/oknozor)

[09ed3e](https://github.com/oknozor/cocogitto/commit/09ed3e6ca3b51307e7b7cfe35fb14666119b90d2) - clippy lints and fmt * - [oknozor](https://github.com/oknozor)

[b0ef1e](https://github.com/oknozor/cocogitto/commit/b0ef1ef870389f96b9db1fec144c7f1fb1476132) - replace serde fmt rexport with std::fmt - [oknozor](https://github.com/oknozor)


- - -
## 1.1.0 - 2020-10-26


### Features

[271b92](https://github.com/oknozor/cocogitto/commit/271b920ceb6eaf61ad2c21f7568ea1e9cedcd0db) - add editor hint on cog edit - [oknozor](https://github.com/oknozor)

[72a692](https://github.com/oknozor/cocogitto/commit/72a6925a582b9038ba7b75d49a76df57ce21adfb) - stash hook generated changes on prehook failure - [oknozor](https://github.com/oknozor)

[fa24d6](https://github.com/oknozor/cocogitto/commit/fa24d643c2b8415abc0e931ea3fae7907c40bbc4) - add shell completions - [oknozor](https://github.com/oknozor)

[940df1](https://github.com/oknozor/cocogitto/commit/940df1369520a2c68dee9e90e9f8cd0eff346fc3) - add git-hooks installer - [oknozor](https://github.com/oknozor)


### Refactoring

[098d6c](https://github.com/oknozor/cocogitto/commit/098d6c079379e88c13d77685a5eee4a3be34df67) - remove unused writter mode: Append & Replace - [oknozor](https://github.com/oknozor)

[7191f4](https://github.com/oknozor/cocogitto/commit/7191f4e25a70fe437b4c567550d2308b3702cbb7) - extract git statuses to a dedicated module - [oknozor](https://github.com/oknozor)

[a69bb2](https://github.com/oknozor/cocogitto/commit/a69bb2a6f43f901dafcb1eb17d2ed3685927a862) - use dir modules instead of file - [oknozor](https://github.com/oknozor)

[bac60f](https://github.com/oknozor/cocogitto/commit/bac60fd07ba76fd0648a64aecad67589aeae5eba) - use Astr<str> for commit type instead of custom impl - [oknozor](https://github.com/oknozor)


### Miscellaneous Chores

[1f0671](https://github.com/oknozor/cocogitto/commit/1f0671d1e3faff35f7f30aaa9fba9c226318797b) - use carret requirement for all dependencies - [oknozor](https://github.com/oknozor)


### Documentation

[97503f](https://github.com/oknozor/cocogitto/commit/97503fb0d665e0a5014c75c6aa308de1f061dfbf) - change git hooks readme title - [oknozor](https://github.com/oknozor)


### Tests

[bd69f4](https://github.com/oknozor/cocogitto/commit/bd69f41c1f47b5e7172df710c97c74c7ec9e6b56) - add statuses test - [oknozor](https://github.com/oknozor)

[6d107c](https://github.com/oknozor/cocogitto/commit/6d107cd0bfe0e401134f9ec8f3d6b23b59c0a759) - move verify to commit module add add tests - [oknozor](https://github.com/oknozor)


### Bug Fixes

[6ff44d](https://github.com/oknozor/cocogitto/commit/6ff44d47c981399c6bfabf8e5a40e6bc30ac2092) - use shorthand instead of full oid in cog log - [oknozor](https://github.com/oknozor)

[dac869](https://github.com/oknozor/cocogitto/commit/dac8698e72d1d8f1d712fa983c4b0196608dc35d) - remove default value for install hook command - [oknozor](https://github.com/oknozor)


- - -
## 1.0.3 - 2020-10-16


### Bug Fixes

[2103a7](https://github.com/oknozor/cocogitto/commit/2103a7f768cf67eeb85f30ad72a75134ee89e772) - %version is now interpretted even without space separator - [oknozor](https://github.com/oknozor)


- - -
## 1.0.2 - 2020-10-12


### Bug Fixes

[2505a4](https://github.com/oknozor/cocogitto/commit/2505a4442fa477561ce4e17fd4b9c6edc90d99dc) - fix typo in ci LICENSE path - [oknozor](https://github.com/oknozor)


- - -
## 1.0.1 - 2020-10-12


### Refactoring

[7c4a1c](https://github.com/oknozor/cocogitto/commit/7c4a1cb692b445f36a44857ce17db32b91acd24c) - replace drain_filter() with a stable alternative - [tranzystorek-io](https://github.com/tranzystorek-io)


### Documentation

[2bd8bb](https://github.com/oknozor/cocogitto/commit/2bd8bb3f8534efc7de808417a237c72e84173fa2) - add AUR package to README - [oknozor](https://github.com/oknozor)

[6fb5ec](https://github.com/oknozor/cocogitto/commit/6fb5ec579a1532a90619ddccd80788a9c99acc72) - document special behavior of auto bumps on 0.y.z - [tranzystorek-io](https://github.com/tranzystorek-io)


### Bug Fixes

[f11374](https://github.com/oknozor/cocogitto/commit/f11374426cbdd395a89437e11ae6fbe1eae88144) - treat 0.y.z autobumps specially - [tranzystorek-io](https://github.com/tranzystorek-io)


### Tests

[67a736](https://github.com/oknozor/cocogitto/commit/67a736c22eaea6d923c76497e82da6f4f3c14666) - add test for autobumping a breaking change on 0.y.z - [tranzystorek-io](https://github.com/tranzystorek-io)


### Continuous Integration

[f66ad9](https://github.com/oknozor/cocogitto/commit/f66ad94585567a496487a989dd3051f61defd387) - use rust stable in github ci - [oknozor](https://github.com/oknozor)

[fe94f4](https://github.com/oknozor/cocogitto/commit/fe94f46b0db1b1b73c11a061a134ca1a1285ac54) - add license to release tar - [oknozor](https://github.com/oknozor)

[1f59a8](https://github.com/oknozor/cocogitto/commit/1f59a859cb0f382f6a64ef59921c655b90ef2e58) - build on rust stable instead of nightly - [oknozor](https://github.com/oknozor)


- - -
## 1.0.0 - 2020-10-11


### Miscellaneous Chores

[8ace14](https://github.com/oknozor/cocogitto/commit/8ace14d36b08b0cba31dc1064af35c960ccb2660) - add several bump hooks and update doc - [oknozor](https://github.com/oknozor)

[1cd3fc](https://github.com/oknozor/cocogitto/commit/1cd3fc18323c2c1ad81ea4dcf4fb20f5b88c89c4) - add tranzystorek-io to contributors - [tranzystorek-io](https://github.com/tranzystorek-io)


### Refactoring

[061004](https://github.com/oknozor/cocogitto/commit/061004e682269b615a85aa51adc52cfbe5c696ec) - fix clippy lints - [renaultfernandes](https://github.com/renaultfernandes)


### Documentation

[78a997](https://github.com/oknozor/cocogitto/commit/78a99781add5b25a5126d1d939adf67ef5748687) - add signature to contributors list - [renaultfernandes](https://github.com/renaultfernandes)


### Bug Fixes

[602030](https://github.com/oknozor/cocogitto/commit/602030e6bdb519f716ee4439300118cf5fe5e4c3) - move cargo package to post bump - [oknozor](https://github.com/oknozor)


### Features

[7015c5](https://github.com/oknozor/cocogitto/commit/7015c51480c15171cc211efb0c3eda0854fc9b09) - rename hooks to pre_bump_hooks - [tranzystorek-io](https://github.com/tranzystorek-io)

[a56b1e](https://github.com/oknozor/cocogitto/commit/a56b1e268703e9548fdd90b0a40d0ef602b29156) - add post-bump-hooks - [tranzystorek-io](https://github.com/tranzystorek-io)

[2bcf97](https://github.com/oknozor/cocogitto/commit/2bcf972be90c903e334f97cf67e010fe1147bd92) - include current branch name in "cog log" - [renaultfernandes](https://github.com/renaultfernandes)

[7c6c72](https://github.com/oknozor/cocogitto/commit/7c6c7259d67bbb7b5ae42df6c3b53dacd503073d) - show repo and current tag name in "cog log" - [renaultfernandes](https://github.com/renaultfernandes)


- - -
## 0.34.0 - 2020-10-10


### Miscellaneous Chores

[5de190](https://github.com/oknozor/cocogitto/commit/5de19002f103ef62a5202fbcf6476cd1cf1d661d) - bump cargo.toml version - [oknozor](https://github.com/oknozor)


### Features

[e4d5fe](https://github.com/oknozor/cocogitto/commit/e4d5fef7cdb5b3421345b21491c01407e509cfe2) - use external pager instead of moins - Mike

[c11147](https://github.com/oknozor/cocogitto/commit/c11147d7ed082d05f0731512579c6dfa6dbc8831) - pre-commit bump hooks - Mike


### Documentation

[cf5419](https://github.com/oknozor/cocogitto/commit/cf5419c29ebc1ffb355c00668b7adcd2d646ae7d) - add documentation for version hooks - [oknozor](https://github.com/oknozor)


### Bug Fixes

[b0609e](https://github.com/oknozor/cocogitto/commit/b0609e7920cc4aae093fca49e9643973610c41b0) - cog bump now perform a single version bump (#44) - [oknozor](https://github.com/oknozor)


### Continuous Integration

[42827f](https://github.com/oknozor/cocogitto/commit/42827f02f762fbd36af88489e929abe12e603030) - update codecov action to work with forks - [oknozor](https://github.com/oknozor)


- - -
## 0.33.1 - 2020-10-06


### Features

[05a487](https://github.com/oknozor/cocogitto/commit/05a487aa73b55e7f84d324fc30b145de67b75d91) - bump --pre flag to set the pre-release version - [mersinvald](https://github.com/mersinvald)


### Documentation

[dff77b](https://github.com/oknozor/cocogitto/commit/dff77b3a13d9ca1c9fd5720a7e3e688db7338996) - add log filters to the doc - [oknozor](https://github.com/oknozor)

[a1906c](https://github.com/oknozor/cocogitto/commit/a1906c3ea740efd15882c7a957d9d62e2ab2182e) - add  bump flag to the doc - [oknozor](https://github.com/oknozor)

[1c66d7](https://github.com/oknozor/cocogitto/commit/1c66d72dd250a722d3b96c15b114a077592e342e) - add contributors github usernames to cog.toml - [oknozor](https://github.com/oknozor)


### Bug Fixes

[f97a6f](https://github.com/oknozor/cocogitto/commit/f97a6f33b3f1992018208747746332dab60b05b3) - typo in get_committer - [jackdorland](https://github.com/jackdorland)


- - -
## 0.32.3 - 2020-09-30


### Bug Fixes

[1c0d2e](https://github.com/oknozor/cocogitto/commit/1c0d2e9398323e6d4fc778309bed242040aa55b5) - fix openssl missing in CD - [oknozor](https://github.com/oknozor)


### Documentation

[da6f63](https://github.com/oknozor/cocogitto/commit/da6f63db9577a9e4ec9d3b10c3022e80be2d0f69) - tag, conventional commit and license badges to readme - [oknozor](https://github.com/oknozor)


- - -
## 0.32.2 - 2020-09-30


### Bug Fixes

[5350b1](https://github.com/oknozor/cocogitto/commit/5350b110b4e39bf6942a58b7a89425e21927b966) - bump setup-rust-action to v1.3.3 - [oknozor](https://github.com/oknozor)


### Documentation

[9a3351](https://github.com/oknozor/cocogitto/commit/9a33516649ba8dd15fafbb6b22970efab1c04dee) - add corrections to README - [oknozor](https://github.com/oknozor)


- - -
## 0.32.1 - 2020-09-30


### Documentation

[b223f7](https://github.com/oknozor/cocogitto/commit/b223f7bec7f2f9df2189e56ffc7177ffa49d6440) - rewritte readme completely - [oknozor](https://github.com/oknozor)


### Bug Fixes

[7f04a9](https://github.com/oknozor/cocogitto/commit/7f04a985b05be36dff170795767a7ad4e696eb4d) - fix ci cross build command bin args - [oknozor](https://github.com/oknozor)


### Refactoring

[d4aa61](https://github.com/oknozor/cocogitto/commit/d4aa61b20ee0d5dd2299f8cb97a75186c75a64f5) - change config name to cog.toml - [oknozor](https://github.com/oknozor)


### Features

[fc7420](https://github.com/oknozor/cocogitto/commit/fc74207b943bfd1b3e36eab80f943e349b0eef36) - move check edit to dedicated subcommand and fix rebase - [oknozor](https://github.com/oknozor)

[1028d0](https://github.com/oknozor/cocogitto/commit/1028d0bee3c12756fd429787a88232bdeae9dc81) - remove config commit on init existing repo - [oknozor](https://github.com/oknozor)


### Miscellaneous Chores

[72bd1e](https://github.com/oknozor/cocogitto/commit/72bd1e4190120b189db64ea2b63318839b219250) - update Cargo.toml - [oknozor](https://github.com/oknozor)


- - -
## 0.30.0 - 2020-09-28


### Features

[d71388](https://github.com/oknozor/cocogitto/commit/d7138865ee4d57a7b8bc18d8fcb73d43feedf504) - improve changelog title formatting - [oknozor](https://github.com/oknozor)


### Miscellaneous Chores

[a6fba9](https://github.com/oknozor/cocogitto/commit/a6fba9c4088032e5979876c4e6a829e7017a4496) - remove test generated dir - [oknozor](https://github.com/oknozor)


### Tests

[9da732](https://github.com/oknozor/cocogitto/commit/9da7321822225d823bf77ef7a06c579017b55cd3) - add verify it tests - [oknozor](https://github.com/oknozor)


### Continuous Integration

[d0d0ae](https://github.com/oknozor/cocogitto/commit/d0d0ae928069a1cdb9cb81f4e483f93c4abc29b0) - fix publish action script - [oknozor](https://github.com/oknozor)


- - -
## 0.29.0 - 2020-09-27


### Features

[ba16b8](https://github.com/oknozor/cocogitto/commit/ba16b89d5dc8e8c03661fd091fa320d09f1ecf05) - add author map for github markdown rendering - [oknozor](https://github.com/oknozor)

[cf380e](https://github.com/oknozor/cocogitto/commit/cf380e6e5b6dae6db7a47a9ae125a334f5db064e) - improve git statuses display - [oknozor](https://github.com/oknozor)

[92cca4](https://github.com/oknozor/cocogitto/commit/92cca40d897aa2a758f167119e275cd5aea23dbc) - add DeriveDiplayOrder to cli - [oknozor](https://github.com/oknozor)

[fc0962](https://github.com/oknozor/cocogitto/commit/fc0962d439e0080309ad67249aeb61535b665394) - this is a commit message - [oknozor](https://github.com/oknozor)

[d2ebbe](https://github.com/oknozor/cocogitto/commit/d2ebbe77bbae2fbc7e31d73a5ecb5fac47b80785) - split commit and utility command into separate bins - [oknozor](https://github.com/oknozor)

[2f3710](https://github.com/oknozor/cocogitto/commit/2f3710644fc4f2095eed7f73d941d8b49a1d94cd) - display git statuses and error message on commit to empty index - [oknozor](https://github.com/oknozor)

[45ac57](https://github.com/oknozor/cocogitto/commit/45ac57abb4a8e12145cb62ce8223663f67dba534) - add init subcommand in cli and the ability to use cog outside git - [oknozor](https://github.com/oknozor)

[fac83f](https://github.com/oknozor/cocogitto/commit/fac83f2fb371010a349c77298759d90a12f636e8) - add example pre-commit hook - [oknozor](https://github.com/oknozor)

[1bdb65](https://github.com/oknozor/cocogitto/commit/1bdb65aa01a4cc977ebf91fde557d6d2e1e83331) - add changelog generation for bump command - [oknozor](https://github.com/oknozor)

[9f2966](https://github.com/oknozor/cocogitto/commit/9f296650d65fee74a88de0f69997cecbe37dcad8) - reimplement custom commits - [oknozor](https://github.com/oknozor)

[fe0e14](https://github.com/oknozor/cocogitto/commit/fe0e143926c6ef36308487bfe61db9441ba76660) - add custom commit type help generation and fix cli help display order - [oknozor](https://github.com/oknozor)

[3ebaac](https://github.com/oknozor/cocogitto/commit/3ebaac00622e3db773b2c48b633a972286af87b8) - add  log filter - [oknozor](https://github.com/oknozor)

[88f6f2](https://github.com/oknozor/cocogitto/commit/88f6f2bbba1954a2990458a12a1a652051044b6c) - add multiple args for log filters - [oknozor](https://github.com/oknozor)

[44bc3f](https://github.com/oknozor/cocogitto/commit/44bc3f38d35e4575e24d18439f23dd9dd0725ae8) - add log filters - [oknozor](https://github.com/oknozor)

[819a04](https://github.com/oknozor/cocogitto/commit/819a04d6a621dccb59892b999739dd81fcbfc806) - add commit optional args {body} {footer} and {breaking-change} - [oknozor](https://github.com/oknozor)

[2248c9](https://github.com/oknozor/cocogitto/commit/2248c9072345ae279497bf1beab7ea150affcc66) - add commit pretty print - [oknozor](https://github.com/oknozor)

[5ff48a](https://github.com/oknozor/cocogitto/commit/5ff48a0625ece8d2c4d9af403aa5077ae1b49dd8) - add custom git and semver error types - [oknozor](https://github.com/oknozor)

[ce4d62](https://github.com/oknozor/cocogitto/commit/ce4d62cef8d775f800aa91d63626995bbadb56cb) - add log command and improve logging - [oknozor](https://github.com/oknozor)

[bbbc90](https://github.com/oknozor/cocogitto/commit/bbbc9084177dab0b5fee79ceae993bdc64b0726a) - add commit date - [oknozor](https://github.com/oknozor)

[d7508a](https://github.com/oknozor/cocogitto/commit/d7508afde5e073bcf3b1aaaa149de9baa721b2ee) - add verify command - [oknozor](https://github.com/oknozor)

[ab054a](https://github.com/oknozor/cocogitto/commit/ab054a3c3f93c070b8e9a6717b556961ec7602c7) - add edit flag for interactive rebase commit renaming - [oknozor](https://github.com/oknozor)

[030932](https://github.com/oknozor/cocogitto/commit/0309325413c618a06bd945d703be31aee6ace6a8) - add commit command - [oknozor](https://github.com/oknozor)

[b932a5](https://github.com/oknozor/cocogitto/commit/b932a5e0873888edb1a369596a38f55fd9404ae7) - add check command - [oknozor](https://github.com/oknozor)

[21abec](https://github.com/oknozor/cocogitto/commit/21abecfa06c9b4aec798197679b27cfd6f0dc7eb) - add changelog arg modes - [oknozor](https://github.com/oknozor)

[46dad5](https://github.com/oknozor/cocogitto/commit/46dad5b7aa41f5fac7d71a1b2add6ded912f3305) - implement changelog - [oknozor](https://github.com/oknozor)

[7b7e47](https://github.com/oknozor/cocogitto/commit/7b7e474a5995ccaa555edbc498ce4d38054cc829) - add changelog date - [oknozor](https://github.com/oknozor)

[d0e87b](https://github.com/oknozor/cocogitto/commit/d0e87bfb78df99cecc44350c320f7c997e1a5362) - add colors to changelog - [oknozor](https://github.com/oknozor)

[925adb](https://github.com/oknozor/cocogitto/commit/925adb484ca98efcb4a54525eec064bfe3e12ec1) - add markdown formating - [oknozor](https://github.com/oknozor)

[e858d5](https://github.com/oknozor/cocogitto/commit/e858d5591be359d4e04c17c9f85a9a336c4ec59e) - convert changelog to markdown - [oknozor](https://github.com/oknozor)


### Documentation

[ee6324](https://github.com/oknozor/cocogitto/commit/ee6324275263c31ea9609b29fc3e9d53f3441d9b) - add codecov badge - [oknozor](https://github.com/oknozor)

[b67e0e](https://github.com/oknozor/cocogitto/commit/b67e0e8f8102a05fb58a2f76827cea9196368960) - line break after logo in readme - [oknozor](https://github.com/oknozor)

[00aadb](https://github.com/oknozor/cocogitto/commit/00aadb5cd082a2f51a47a2f6de8b810b0667ef24) - add ci badge to readme - [oknozor](https://github.com/oknozor)

[346716](https://github.com/oknozor/cocogitto/commit/346716abf5fc155988c70c8b304237d090c764cf) - add toc to README.md - [oknozor](https://github.com/oknozor)

[aa4a85](https://github.com/oknozor/cocogitto/commit/aa4a853a787695fa9a247b7570a7a4a07a50419b) - add README.md - [oknozor](https://github.com/oknozor)


### Bug Fixes

[342f81](https://github.com/oknozor/cocogitto/commit/342f81a0313d9a2fd0812d3afc01050b6ad4637d) - fix changelog markdown format - [oknozor](https://github.com/oknozor)

[eeb917](https://github.com/oknozor/cocogitto/commit/eeb917e8e2e685f74813d00783ae36556cacf371) - add bump test and fix version auto bump - [oknozor](https://github.com/oknozor)

[d2270f](https://github.com/oknozor/cocogitto/commit/d2270fcad3a6b818a5f6c73e78e8500fbc092748) - fix error: 'parent index out of bounds' (#18) - [oknozor](https://github.com/oknozor)

[f3dc3b](https://github.com/oknozor/cocogitto/commit/f3dc3b96a3b24fd2083a2c07a47776d57829aec0) - add line break between changelog commit line - [oknozor](https://github.com/oknozor)

[55f62a](https://github.com/oknozor/cocogitto/commit/55f62ac87c42376e386d6d64d701f045f25e64e0) - hide internal method visibility and fix some clippy lints - [oknozor](https://github.com/oknozor)

[d5684c](https://github.com/oknozor/cocogitto/commit/d5684c437daa81848bdd5758eec4a222073a6381) - decrease  method visibility - [oknozor](https://github.com/oknozor)

[17668b](https://github.com/oknozor/cocogitto/commit/17668baeda1369910789c4ceeee0968a6a2f7243) - bump version arg - [oknozor](https://github.com/oknozor)


### Revert

[ba4a2c](https://github.com/oknozor/cocogitto/commit/ba4a2cfc23eea7186761e5738441793755b503e5) - remove test changelog - [oknozor](https://github.com/oknozor)

[6d810a](https://github.com/oknozor/cocogitto/commit/6d810a67d20489860a5c3c24dcee8d4c03056449) - remove changelog header and footer from config - [oknozor](https://github.com/oknozor)

[329587](https://github.com/oknozor/cocogitto/commit/329587fdf19167fbe652fe675b083dd84e4ca976) - remove commit sort struct - [oknozor](https://github.com/oknozor)


### Miscellaneous Chores

[7b5f61](https://github.com/oknozor/cocogitto/commit/7b5f614ecd0699eba6e9f50a05df04e213e6012c) - test commit - [oknozor](https://github.com/oknozor)

[a5abe8](https://github.com/oknozor/cocogitto/commit/a5abe80c3c171ee2f1172b1ce4b0b6738034df05) - add cocogitto config - [oknozor](https://github.com/oknozor)

[481d57](https://github.com/oknozor/cocogitto/commit/481d577e4e1821b684c68d9e39756c4d015de98d) - fix clippy lints - [oknozor](https://github.com/oknozor)

[3fd06f](https://github.com/oknozor/cocogitto/commit/3fd06fec7325eed469fae9dc2968eed23b683545) - add coco bin to Cargo.toml - [oknozor](https://github.com/oknozor)

[5375e1](https://github.com/oknozor/cocogitto/commit/5375e15770ddf8821d0c1ad393d315e243014c15) - changelog format - [oknozor](https://github.com/oknozor)

[956152](https://github.com/oknozor/cocogitto/commit/956152e44e3a2b004f0d7f45c3351ed3db0fdc8a) - fix lints - [oknozor](https://github.com/oknozor)

[047323](https://github.com/oknozor/cocogitto/commit/04732333bb9c14db9b5224bfb876bebe36746f32) - add issue templates - [oknozor](https://github.com/oknozor)

[63169a](https://github.com/oknozor/cocogitto/commit/63169a67351522743481cbe968899d27e5a8555e) - add issue templates - [oknozor](https://github.com/oknozor)

[bea5a2](https://github.com/oknozor/cocogitto/commit/bea5a2d2d9cfdeb170127a7c830f430d514b9178) - remove dummy_repo from gitignore - [oknozor](https://github.com/oknozor)

[5c8490](https://github.com/oknozor/cocogitto/commit/5c84905751bd58b7d5578c5b7751293722c8b6ae) - bump moins version - [oknozor](https://github.com/oknozor)

[fecbcf](https://github.com/oknozor/cocogitto/commit/fecbcf46f628c45f908b643d1cf85764ed0d86b8) - add temporary ugly logo - [oknozor](https://github.com/oknozor)

[02a289](https://github.com/oknozor/cocogitto/commit/02a289d498ebcf9d751d3567ddb51baf5a11c322) - add MIT license - [oknozor](https://github.com/oknozor)

[b9ad27](https://github.com/oknozor/cocogitto/commit/b9ad2717436d17200bd1095d211a7faae1d6193b) - fmt all - [oknozor](https://github.com/oknozor)

[629016](https://github.com/oknozor/cocogitto/commit/6290168be8dc18adb139319f60e0036321c86954) - clean lints - [oknozor](https://github.com/oknozor)


### Tests

[3b26be](https://github.com/oknozor/cocogitto/commit/3b26be99e539b535dc5e00e4ba87f0f2bdd743ab) - add version bump test - [oknozor](https://github.com/oknozor)

[8a9921](https://github.com/oknozor/cocogitto/commit/8a9921f3bacb377e620fa547f14a69e5f325c8e0) - add repository unit tests - [oknozor](https://github.com/oknozor)

[0db6a4](https://github.com/oknozor/cocogitto/commit/0db6a47789b11a567ac887808ec2452b5f09dc21) - bootstrap bump command - [oknozor](https://github.com/oknozor)

[bd4972](https://github.com/oknozor/cocogitto/commit/bd497255ea629feef2189d0c9ddb00a1dd2d7ec3) - add init command cli test - [oknozor](https://github.com/oknozor)

[de60c0](https://github.com/oknozor/cocogitto/commit/de60c0b2a121a497f2dd786646492c57249ec056) - add changelog generation tests - [oknozor](https://github.com/oknozor)

[121235](https://github.com/oknozor/cocogitto/commit/121235f8272e978de1d83af8a58248b18636c658) - add test for cocogitto check - [oknozor](https://github.com/oknozor)

[e4271d](https://github.com/oknozor/cocogitto/commit/e4271db6cf9552865a0f5a263a0529a9389357c2) - add test util git commands - [oknozor](https://github.com/oknozor)


### Continuous Integration

[d7f314](https://github.com/oknozor/cocogitto/commit/d7f3146a762b9f07b9efe69ff9bc673371f1fbbf) - run tarpaulin on one thread - [oknozor](https://github.com/oknozor)

[88d67a](https://github.com/oknozor/cocogitto/commit/88d67a09a919a386cb1f367c4e0a75db1f5664cd) - split tarpaulin and unit tests - [oknozor](https://github.com/oknozor)

[35085f](https://github.com/oknozor/cocogitto/commit/35085f20c5293fc8830e4e44a9bb487f98734f73) - add git user for tarpaulin - [oknozor](https://github.com/oknozor)

[a1147b](https://github.com/oknozor/cocogitto/commit/a1147ba3cd9cf92cc4013c2ddec40b08d7dc5d71) - add github action ci/cd - [oknozor](https://github.com/oknozor)


### Refactoring

[dad15d](https://github.com/oknozor/cocogitto/commit/dad15d17e175021453c61d8f7e919cba944faa63) - refactor verify to get current user signature - [oknozor](https://github.com/oknozor)

[a2b709](https://github.com/oknozor/cocogitto/commit/a2b7098f8351015cedeb6c971e641b413976b36a) - extract version logic to a dedicated module - [oknozor](https://github.com/oknozor)

[f78056](https://github.com/oknozor/cocogitto/commit/f780561a95a75bc132637825d43db029335ae8b9) - replace custom semver struct with semver crate - [oknozor](https://github.com/oknozor)

[063987](https://github.com/oknozor/cocogitto/commit/0639872f1076e6cac320de0a118e7e545f1a2300) - clippy lints - [oknozor](https://github.com/oknozor)

[d9cf44](https://github.com/oknozor/cocogitto/commit/d9cf446afd1675fa59858e1569ab517833e796e1) - rework check command output and commit parsing - [oknozor](https://github.com/oknozor)

[156d9b](https://github.com/oknozor/cocogitto/commit/156d9b586140cf193201ff6608230532796da8a0) - move conventional commit command logic to pub function - [oknozor](https://github.com/oknozor)

[2537a3](https://github.com/oknozor/cocogitto/commit/2537a32ded42cea58675de1cf29a0aa61595df44) - add closure for markdown commit section generation - [oknozor](https://github.com/oknozor)

[3cd77b](https://github.com/oknozor/cocogitto/commit/3cd77b64cb46ec6e231cee3578ed6ec2cdaca79e) - move commit to a dedicated module - [oknozor](https://github.com/oknozor)


- - -

This changelog was generated by [cocogitto](https://github.com/oknozor/cocogitto).