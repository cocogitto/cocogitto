# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

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