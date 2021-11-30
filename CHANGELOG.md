# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

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