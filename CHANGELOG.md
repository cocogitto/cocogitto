# Changelog

All notable changes to this project will be documented in this file. \
See {}(https://www.conventionalcommits.org/) for commit guidelines.

- - -
## 8806a5..96e35d - 2020-09-26


### Refactoring

dad15d1 - refactor verify to get current user signature - Paul Delafosse

a2b7098 - extract version logic to a dedicated module - Paul Delafosse

f780561 - replace custom semver struct with semver crate - Paul Delafosse

0639872 - clippy lints - Paul Delafosse

d9cf446 - rework check command output and commit parsing - Paul Delafosse

156d9b5 - move conventional commit command logic to pub function - Paul Delafosse

2537a32 - add closure for markdown commit section generation - Paul Delafosse

3cd77b6 - move commit to a dedicated module - Paul Delafosse


### Continuous Integration

96e35d0 - run tarpaulin on one thread - Paul Delafosse

88d67a0 - split tarpaulin and unit tests - Paul Delafosse

35085f2 - add git user for tarpaulin - Paul Delafosse

a1147ba - add github action ci/cd - Paul Delafosse


### Features

d2ebbe7 - split commit and utility command into separate bins - Paul Delafosse

2f37106 - display git statuses and error message on commit to empty index - Paul Delafosse

45ac57a - add init subcommand in cli and the ability to use cog outside git - Paul Delafosse

fac83f2 - add example pre-commit hook - Paul Delafosse

1bdb65a - add changelog generation for bump command - Paul Delafosse

9f29665 - reimplement custom commits - Paul Delafosse

fe0e143 - add custom commit type help generation and fix cli help display order - Paul Delafosse

3ebaac0 - add  log filter - Paul Delafosse

88f6f2b - add multiple args for log filters - Paul Delafosse

44bc3f3 - add log filters - Paul Delafosse

819a04d - add commit optional args {body} {footer} and {breaking-change} - Paul Delafosse

2248c90 - add commit pretty print - Paul Delafosse

5ff48a0 - add custom git and semver error types - Paul Delafosse

ce4d62c - add log command and improve logging - Paul Delafosse

bbbc908 - add commit date - Paul Delafosse

d7508af - add verify command - Paul Delafosse

ab054a3 - add edit flag for interactive rebase commit renaming - Paul Delafosse

0309325 - add commit command - Paul Delafosse

b932a5e - add check command - Paul Delafosse

21abecf - add changelog arg modes - Paul Delafosse

46dad5b - implement changelog - Paul Delafosse

7b7e474 - add changelog date - Paul Delafosse

d0e87bf - add colors to changelog - Paul Delafosse

925adb4 - add markdown formating - Paul Delafosse

e858d55 - convert changelog to markdown - Paul Delafosse


### Bug Fixes

7a91014 - fix error: 'parent index out of bounds' (#18) - Paul Delafosse

f3dc3b9 - add line break between changelog commit line - Paul Delafosse

55f62ac - hide internal method visibility and fix some clippy lints - Paul Delafosse

d5684c4 - decrease  method visibility - Paul Delafosse

17668ba - bump version arg - Paul Delafosse


### Documentation

ee63242 - add codecov badge - Paul Delafosse

b67e0e8 - line break after logo in readme - Paul Delafosse

00aadb5 - add ci badge to readme - Paul Delafosse

346716a - add toc to README.md - Paul Delafosse

aa4a853 - add README.md - Paul Delafosse


### Miscellaneous Chores

e0b9440 - add cocogitto config - Paul Delafosse

f19bd20 - 0.25.0 - Paul Delafosse

481d577 - fix clippy lints - Paul Delafosse

3fd06fe - add coco bin to Cargo.toml - Paul Delafosse

5375e15 - changelog format - Paul Delafosse

956152e - fix lints - Paul Delafosse

0473233 - add issue templates - Paul Delafosse

63169a6 - add issue templates - Paul Delafosse

bea5a2d - remove dummy_repo from gitignore - Paul Delafosse

5c84905 - bump moins version - Paul Delafosse

fecbcf4 - add temporary ugly logo - Paul Delafosse

02a289d - add MIT license - Paul Delafosse

b9ad271 - fmt all - Paul Delafosse

6290168 - clean lints - Paul Delafosse


### Tests

dc9a63e - add repository unit tests - Paul Delafosse

bd49725 - add init command cli test - Paul Delafosse

de60c0b - add changelog generation tests - Paul Delafosse

121235f - add test for cocogitto check - Paul Delafosse

e4271db - add test util git commands - Paul Delafosse


### Revert

6d810a6 - remove changelog header and footer from config - Paul Delafosse

329587f - remove commit sort struct - Paul Delafosse


- - -
## 8806a5..481d57 - 2020-09-24


### Features

d2ebbe7 - split commit and utility command into separate bins - Paul Delafosse

2f37106 - display git statuses and error message on commit to empty index - Paul Delafosse

45ac57a - add init subcommand in cli and the ability to use cog outside git - Paul Delafosse

fac83f2 - add example pre-commit hook - Paul Delafosse

1bdb65a - add changelog generation for bump command - Paul Delafosse

9f29665 - reimplement custom commits - Paul Delafosse

fe0e143 - add custom commit type help generation and fix cli help display order - Paul Delafosse

3ebaac0 - add  log filter - Paul Delafosse

88f6f2b - add multiple args for log filters - Paul Delafosse

44bc3f3 - add log filters - Paul Delafosse

819a04d - add commit optional args {body} {footer} and {breaking-change} - Paul Delafosse

2248c90 - add commit pretty print - Paul Delafosse

5ff48a0 - add custom git and semver error types - Paul Delafosse

ce4d62c - add log command and improve logging - Paul Delafosse

bbbc908 - add commit date - Paul Delafosse

d7508af - add verify command - Paul Delafosse

ab054a3 - add edit flag for interactive rebase commit renaming - Paul Delafosse

0309325 - add commit command - Paul Delafosse

b932a5e - add check command - Paul Delafosse

21abecf - add changelog arg modes - Paul Delafosse

46dad5b - implement changelog - Paul Delafosse

7b7e474 - add changelog date - Paul Delafosse

d0e87bf - add colors to changelog - Paul Delafosse

925adb4 - add markdown formating - Paul Delafosse

e858d55 - convert changelog to markdown - Paul Delafosse


### Miscellaneous Chores

481d577 - fix clippy lints - Paul Delafosse

3fd06fe - add coco bin to Cargo.toml - Paul Delafosse

5375e15 - changelog format - Paul Delafosse

956152e - fix lints - Paul Delafosse

0473233 - add issue templates - Paul Delafosse

63169a6 - add issue templates - Paul Delafosse

bea5a2d - remove dummy_repo from gitignore - Paul Delafosse

5c84905 - bump moins version - Paul Delafosse

fecbcf4 - add temporary ugly logo - Paul Delafosse

02a289d - add MIT license - Paul Delafosse

b9ad271 - fmt all - Paul Delafosse

6290168 - clean lints - Paul Delafosse


### Revert

6d810a6 - remove changelog header and footer from config - Paul Delafosse

329587f - remove commit sort struct - Paul Delafosse


### Bug Fixes

f3dc3b9 - add line break between changelog commit line - Paul Delafosse

55f62ac - hide internal method visibility and fix some clippy lints - Paul Delafosse

d5684c4 - decrease  method visibility - Paul Delafosse

17668ba - bump version arg - Paul Delafosse


### Refactoring

dad15d1 - refactor verify to get current user signature - Paul Delafosse

a2b7098 - extract version logic to a dedicated module - Paul Delafosse

f780561 - replace custom semver struct with semver crate - Paul Delafosse

0639872 - clippy lints - Paul Delafosse

d9cf446 - rework check command output and commit parsing - Paul Delafosse

156d9b5 - move conventional commit command logic to pub function - Paul Delafosse

2537a32 - add closure for markdown commit section generation - Paul Delafosse

3cd77b6 - move commit to a dedicated module - Paul Delafosse


### Tests

bd49725 - add init command cli test - Paul Delafosse

de60c0b - add changelog generation tests - Paul Delafosse

121235f - add test for cocogitto check - Paul Delafosse

e4271db - add test util git commands - Paul Delafosse


### Documentation

ee63242 - add codecov badge - Paul Delafosse

b67e0e8 - line break after logo in readme - Paul Delafosse

00aadb5 - add ci badge to readme - Paul Delafosse

346716a - add toc to README.md - Paul Delafosse

aa4a853 - add README.md - Paul Delafosse


### Continuous Integration

88d67a0 - split tarpaulin and unit tests - Paul Delafosse

35085f2 - add git user for tarpaulin - Paul Delafosse

a1147ba - add github action ci/cd - Paul Delafosse


- - -

- - -
This changelog was generated by [cocogitto](https://github.com/oknozor/cocogitto).
