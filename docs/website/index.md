---
home: true
heroImage: logo.png
tagline: The conventional commit toolbox 
actionText: Getting Started →
actionLink: /guide/
features:
- title: Verified commits️
  details: Create conventional compliant commits at ease.
- title: Automatic Version bump and changelog
  details: Automatically bump version and generate changelog with your own custom steps and workflows.
- title: Release profiles
  details: Your branching model requires different steps for releases, prerelease, hotfix ? We got you covered !
- title: Depends only on libgit2
  details: Cocogitto has two standalone binary, the only system dependency is libgit2.
- title: Conventional git log
  details: Search your commit history matching conventional commit items such as scope and commit type.
- title: Enforce conventional commits via github actions
  details: Check your commit compliance on every push to the remote and create release from your CI pipeline or using our Github bot.
footer: MIT Licensed | Copyright © 2020 Paul Delafosse
---

## Installation

### Cargo

Cocogitto is available on [crates.io](https://crates.io/crates/cocogitto) :

```shell script
cargo install cocogitto
```

### Archlinux

```shell script
pacman -S cocogitto
```

### Nixos

```shell script
nix-env -iA nixos.cocogitto
```

### Void Linux

```shell script
xbps-install cocogitto
```

## Shell completions

Before getting started you might want to install shell completions (Note that this is not needed for the official archlinux package).
Supported shells are `bash`, `elvish`, `fish` and `zsh`.

Example installing completions:

```sh
# Bash
cog generate-completions bash > ~/.local/share/bash-completion/completions/cog

# Bash (macOS/Homebrew)
cog generate-completions bash > $(brew --prefix)/etc/bash_completion.d/cog.bash-completion

# Fish
mkdir -p ~/.config/fish/completions
cog generate-completions fish > ~/.config/fish/completions/cog.fish

# Zsh
cog generate-completions zsh > ~/.zfunc/_cog
```

## Introduction

Cocogitto comes with a single binary named `cog`.

Use the `--help`  to display options and usage about a specific subcommand :

```shell
cog --help
cog changelog --help
# And so on...
```

Note that if you do not care about automatic release, changelog generation and just want
to create conventional commit message you can jump to the [conventional commits section](./guide/#conventional_commits).

## Conventional commits

`cog commit` is primarily meant to be used as a replacement for the `git commit` command.
It will produce commits with messages respecting the conventional commits specification with
little effort.

**Example :**

```sh
# With git commit
git commit -m "feat: implement the parser specification"

# With cocogitto
cog commit feat "implement the parser specification"
```

Using `cog commit` while working on a project using conventional commits is less verbose than good old git cli and prevent
typos and common mistake when writing conventional commit messages.

See [User guide -> Conventional commits](./guide/#conventional_commits) for more information.


## Repository management

While local commit are made with the `cog commit` command, other `cog` subcommands are meant to manage your repository 
both locally and in a CI context. For an in depth guide on how to use it see [User guide](./guide).

## GitHub integration

### GitHub Action

Anything you can do with `cog` can be done in your CI pipeline with [cocogitto-action](https://github.com/cocogitto/cocogitto-action). 

See [Github integration -> GitHub action](./ci_cd/#github-action) for more info.

### GitHub Bot

To help your contributors respect the specification [cocogitto-bot](https://github.com/apps/cocogitto-bot)
can decorate your PR with conventional commits status checks.

See [Github integration -> GitHub bot](./ci_cd/#github-action) for more info.







