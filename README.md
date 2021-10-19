# Cocogitto
![logo](docs/assets/logo.png)
![CI](https://github.com/oknozor/cocogitto/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/oknozor/cocogitto/branch/master/graph/badge.svg)](https://codecov.io/gh/oknozor/cocogitto)
![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/oknozor/cocogitto)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
![License](https://img.shields.io/github/license/oknozor/cocogitto)


Cocogitto is a set of cli tools for the [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/) 
and [semver](https://semver.org/) specifications.  

## Foreword

There are plenty of tools listed on the [conventional commit web site](https://www.conventionalcommits.org/en/v1.0.0/#tooling-for-conventional-commits) 
to help you generate changelog, git hooks, commit template etc,
Some of them are specifically designed for the conventional commit specification, and some of them are general purpose.
Cocogitto was designed to help you respect the conventional and semver standard and is not intended to be use in any other context.  

It strives to be a set of simples, moderns and fast command line interfaces and leverage [git2-rs](https://github.com/rust-lang/git2-rs)
to provide what you would expect from such tools plus some original features.

### Goals

- Make using the conventional commit spec easy and fun.
- Enable people to focus on their work instead of correcting small mistakes and typo.
- Correctness regarding semver and conventional commit.
- Automate things when possible (ex: bumping versions).

### Non goals

- Coccogitto is not a `git` remplacement. It uses some of libgit2 but  only to provide
features related to the conventional commit specification. Anything else shall be done with `git`. 
- Supporting other commit convention or git workflow.   



## Table of contents
- [Configuration](#Configuration)
- [Installation](#Installation)
    - [Cargo](#cargo)
    - [Archlinux](#archlinux)
- [Binaries](#Binaries)
    - [Shell completions](#Shell-completions)
- [Coco Commits](#Coco-Commits)
    - [Breaking changes](#Breaking-changes)
- [Cog commands](#Cog-commands)
    - [Initialize a project](#Initialize-a-project)
        - [New repository](#New-repository)
        - [Existing repository](#Existing-repository)
    - [Check commit history](#Check-commit-history)
    - [Edit commit history](#Edit-commit-history)
    - [Conventional commit logs](#Conventional-commit-logs)
    - [Generate changelogs](#Generate-changelogs)
    - [Auto bump](#Auto-bump)
    - [Bump hooks](#Bump-hooks)
    - [Pre bump hooks](#pre-bump-hooks)
    - [Post bump hooks](#post-bump-hooks)
    - [Builtin git hooks](#Builtin-git-hooks)
    - [Github action](#Github-action)
- [Contributing](#Contributing)
- [Licence](#Licence)

## Installation

### Cargo

Cocogitto is available on [crates.io](https://crates.io/crates/cocogitto): 

```
cargo install cocogitto
```

### Archlinux

```
yay -S cocogitto-bin
```

## Configuration

All configuration values are optional, take a look at [cog.toml](cog.toml) to know more.

## Binaries

At the moment Cocogitto comes with two binaries `coco` and `cog`. 

- `coco` is used to create commits respecting the conventional commit spec.
- `cog` does everything else: check your 
repo history against the spec, edit malformed commit messages, generate changelog and bump versions etc.

### Shell completions

Before getting started you might want to install shell completions for `cog` and `coco` commands.
Supported shells are `bash`, `elvish`, `fish` and `zsh`.

Example installing completions: 

```
# Bash
$ cog generate-completions bash > ~/.local/share/bash-completion/completions/cog
$ coco generate-completions bash > ~/.local/share/bash-completion/completions/coco

# Bash (macOS/Homebrew)
$ cog generate-completions bash > $(brew --prefix)/etc/bash_completion.d/cog.bash-completion
$ coco generate-completions bash > $(brew --prefix)/etc/bash_completion.d/coco.bash-completion

# Fish
$ mkdir -p ~/.config/fish/completions
$ cog generate-completions fish > ~/.config/fish/completions/cog.fish
$ coco generate-completions fish > ~/.config/fish/completions/coco.fish

# Zsh
$ cog generate-completions zsh > ~/.zfunc/_cog
$ coco generate-completions zsh > ~/.zfunc/_coco
```

## Coco Commits

`coco` allows you to easily create commits respecting the conventional specification. It comes with a set of predefined arguments
named after conventional commit types: `style`, `build`, `refactor`, `ci`, `fix`, `test`, `perf`, `chore`, `feat`, `revert`, `docs`.    

Conventional commits are structured as follows: 

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

All `coco` commit commands follows the same structure: 

```
coco {type} {message} [optional scope] [optional body] [optional footer]
```

The only difference you need to remember is that `coco` commit scope comes after the commit description. This allows 
using positional arguments instead of typing flags (ex: `coco -t {type} -s {scope} -m {message}... and so on`)

For instance if you want to create the following commit: `feat: add awesome feature` you would run this :

```shell script
coco feat "add awesome feature"
```

### Breaking changes

All `coco` argument are positional except the optional `-B` flag used to create breaking changes commits: 

```shell script
coco fix -B "add fix a nasty bug" cli
```

This would create the following [breaking change](https://www.conventionalcommits.org/en/v1.0.0/#commit-message-with--to-draw-attention-to-breaking-change) 
commit:`fix!: fix a nasty bug`.

`coco` use the `!` notation to denote breaking changes commit because it can be easily seen in your git log, however if
you manually create breaking changes commits with [the footer notation](https://www.conventionalcommits.org/en/v1.0.0/#commit-message-with-description-and-breaking-change-footer)
cocogitto tools will still pick them.


## Cog commands

The `cog` binary is used for everything that is not commit the available commands are the following:

| Command     | Description | Required flags |
| :---        | :----       |:---           |  
| `init`      | Create an empty repository with `cog.toml` template config an initial commit ||
| `check`     | Check current repository commit message ||
| `edit`      | Interactive rebase of all non-conventional commit message ||
| `log`       | Like `git log` but for conventional commits ||
| `verify`    | Verify an input string against the conventional commit specification ||
| `changelog` | Generate a changelog to stdout|`--from {revspec}`, `--to {revspec}` |
| `bump`      | Bump current version, append to changelog file and create a version commit| `--auto`, `--major`, `--minor`, `--patch`, `--version <version>`|

To know more about a specific `cog` subcommand one run `cog {subcommand} --help`. 


### Initialize a project

### New repository

```shell script
mkdir my_repo && cd my_repo
cog init
```

`cog init` works like `git init` except it create a template `cog.toml` config file, 
and a default init commit with the following message:`chore: initial commit`.

Optionally you can specify the path of the repository you want to create :

```shell script
cog init my_repo
```

### Existing repository

Running `cog init` on an existing repository will just create a template configuration without creating any commit :

```shell script
git init my_repo && cd my_repo
cog init
```

```
❯ git status
On branch master
Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
	new file:   cog.toml
```

### Check commit history

Running `cog check` will check your commit history against the conventional commit specification:

```
❯ cog check
No errored commits
```

Let us create an invalid commit:
```shell script
git commit -m "Your Mother Was A Hamster, And Your Father Smelt Of Elderberries"
```

And check our commit history again :
```
❯ cog check
ERROR - Your Mother Was A Hamster, And Your Father Smelt Of Elderberries - (c82c30)
	cause: invalid commit format:missing `: ` separator
```

Additionally, you can check your history, starting from the latest tag to HEAD using `from-latest-tag` flag.  
This is useful when your git repo started to use conventional commits from a certain point in history, and you
don't care about editing old commits. 

### Edit commit history

Once you have spotted invalid commits you can quickly fix your commit history by running `cog edit`.
This will perform an automatic rebase and let you edit each malformed commit message with your `$EDITOR`
of choice. 

### Conventional commit logs

`cog log` is like `git log` but it displays additional conventional commit information, such as commit scope, 
commit type etc. 


[![asciicast](https://asciinema.org/a/ssH4yRSlc28Rb9dHEDN7TowGe.svg)](https://asciinema.org/a/ssH4yRSlc28Rb9dHEDN7TowGe)

You can also filter the log content with the following flags (`cog log --help`) :

- `-B`:display breaking changes only
- `-t`:filter on commit type
- `-a`:filter on commit author
- `-s`:filter on commit scope
- `-e`:ignore errors 

Those flag can be combined to achieve complex search in your commit history:

```shell script
cog log --author "Paul Delafosse" "Mike Lubinets" --type feat --scope cli --no-error
```

### Generate changelogs

There are two way to generate changelog with `cog`:
- To stdout with `cog changelog`.

    ```
    ❯ cog changelog
    
    ## 0.30.0..HEAD - 2020-09-30
    
    
    ### Bug Fixes
    
    7f04a9 - fix ci cross build command bin args - Paul Delafosse
    
    ### Features
    
    fc7420 - move check edit to dedicated subcommand and fix rebase - Paul Delafosse
    1028d0 - remove config commit on init existing repo - Paul Delafosse
    
    ### Refactoring
    
    d4aa61 - change config name to cog.toml - Paul Delafosse
    ```
  
- To your repo `CHANGELOG.md` file with `cog bump`. 


### Auto bump

Assuming we are working on the following git repository:
```
* (HEAD -> master) feat: another cool feature
* docs: add some documentation
* fix: fix ugly bug
* feat: add awesome feature
* chore: initial commit
```

Let us now create a version:
```
❯ cog bump --auto
Found feature commit caef0f, bumping to 0.1.0
Skipping irrelevant commit 025cc0 with type: docs
Found bug fix commit e2af66, bumping to 0.1.1
Found feature commit 1b87aa, bumping to 0.2.0
Bumped version:0.0.0 -> 0.2.0
```

If we look again at our git log:
```
(HEAD -> master, tag: 0.2.0) chore(version): 0.2.0
... 
```

Also, a `CHANGELOG.md` file have been created. 

```markdown
# Changelog
All notable changes to this project will be documented in this file.
See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## 0.2.0 - 2020-09-30


### Bug Fixes

e2af66 - fix ugly bug - Paul Delafosse


### Documentation

025cc0 - add some documentation - Paul Delafosse


### Features

caef0f - another cool feature - Paul Delafosse

1b87aa - add awesome feature - Paul Delafosse


- - -

This changelog was generated by [cocogitto](https://github.com/oknozor/cocogitto).
```

You need to run `cog bump` with one of the following flags:
- `--auto`:choose the next version for you (based on feature commit, bug fixes commit and BREAKING_CHANGE commit).
- `--major`:increment the MAJOR version.
- `--minor`:increment the MINOR version.
- `--patch`:increment the PATCH version.
- `--version <version>`:set version manually (ex:`cog bump --version 3.2.1`).

You can also create pre-release version by adding the `--pre` flag to the bump command :

```shell script
cog bump --major --pre "beta.1"
```

Coming from `1.2.3` this would create the following tag:`2.0.0-beta.0`.


If you create a new version, the version changelog will be prepended to your changelog file separated by `- - -`.
Note that if your project already contains a changelog you can tell `cog` about it by adding this to the file:

```markdown
- - -
- - -
```

You might also need to adjust `changelog_path` in `cog.toml`.

**Note:** `cog bump --auto` treats `0.y.z` versions specially,
i.e. it will never do an auto bump to the `1.0.0` version, even if there are breaking changes.
That way, you can keep adding features in the development stage and decide yourself, when your API is stable.

## Bump hooks

### Pre bump hooks

Creating git tag automatically is great but sometimes you need to edit some file with the new version number,
or perform some additional checks before doing so. 

A typical example is editing your project manifest in your package manager configuration file.
You can run pre bump commands with the `{{version}}` alias to reference the newly created version :

```toml
# cog.toml
pre_bump_hooks = [
    "cargo bump {{version}}",
    "cargo build --release",
]
```

When running `cog bump` these command will be run before creating the version commit.
Assuming we are bumping to `1.1.0`, the `{{version}}` alias will be replaced with `1.1.0`.

### Post bump hooks

You can tell `cog` to run commands after the bump.

```toml
# cog.toml
post_bump_hooks = [
    "git push",
    "git push origin {{version}}",
    "cargo publish"
]
```  

### Version DSL

It is common to bump your development branch version package manifest after creating a new release. 
A typical example in the java world would be to bump your maven snapshot on your development branch after a release.  

```toml
# cog.toml
post_bump_hooks = [
    "git push",
    "git push origin {{version}}",
    "git checkout develop",
    "git rebase master",
    "mvn versions:set -DnewVersion={{version+minor-SNAPSHOT}}",
    "coco chore \"bump snapshot to {{version+1minor-SNAPSHOT}}\"",
    "git push",
]
```

As you can see we are bumping the manifest using a small DSL. It as only a few keywords:
- start with the `version` keyword.
- followed by the `+` operator.
- `major`, `minor` and `patch` to specify the kind of increment you want. 
  Then an optional amount, default being one (`version+1minor` and `version+minor` being the same).
- followed by any number of `+{amount}{kind}` (exemple: `version+2major+1patch`)
- ended by any alphanumeric character (SemVer additional labels for pre-release and build metadata), here `-SNAPSHOT`.

### Builtin git hooks

To protect your commit history, and your git remote, cog have builtins 
[git hooks](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks). 

You can install them all by running:
```
cog install-hook all
```

Or one by one, specifying the hook name:

1. Pre-push hook

    Enabling this hook will run `cog check` before pushing to remote.
    ```
    cog install-hooks pre-push
    ```

2. Pre-commit hook

    Enabling this hook will run `cog verify` before creating a new commit.

    ```
    cog install-hook pre-commit
    ```

## Github action

You can run cog check on github action using  [cocogitto-action](https://github.com/oknozor/cocogitto-action).

## Contributing

Found a bug, have a suggestion for a new feature ? Please read the contribution guideline and submit an issue.

## Licence

All the code in this repository is released under the MIT License, for more information take a look at the [LICENSE](LICENSE) file.
