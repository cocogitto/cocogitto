---
home: true
heroImage: https://v1.vuepress.vuejs.org/hero.png
tagline: CLI and automation for the conventional commits and SemVer specification. 
actionText: Getting Started →
actionLink: /guide/
features:
- title: Verified commits️
  details: Create conventional compliant commits at ease
- title: Automatic Version bump and changelog
  details: Automatically bump version and changelog generation with your own custom steps and workflows.
- title: Release profiles
  details: Your branching model requires different steps for releases, prerelease, hotfix ? We got you covered
- title: No dependency
  details: Cocogitto has two standalone binary, the only system dependency is git
- title: Conventional git log
  details: Search your commit history matching conventional commit items such as scope and commit type.
- title: Enforce conventional commits via github actions
  details: Check your commit compliance on every push to the remote and create release from your CI pipeline
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
yay -S cocogitto-bin
```

## Shell completions

Before getting started you might want to install shell completions for `cog` and `coco` commands.
Supported shells are `bash`, `elvish`, `fish` and `zsh`.

Example installing completions:

```shell script
# Bash
cog generate-completions bash > ~/.local/share/bash-completion/completions/cog
coco generate-completions bash > ~/.local/share/bash-completion/completions/coco

# Bash (macOS/Homebrew)
cog generate-completions bash > $(brew --prefix)/etc/bash_completion.d/cog.bash-completion
coco generate-completions bash > $(brew --prefix)/etc/bash_completion.d/coco.bash-completion

# Fish
mkdir -p ~/.config/fish/completions
cog generate-completions fish > ~/.config/fish/completions/cog.fish
coco generate-completions fish > ~/.config/fish/completions/coco.fish

# Zsh
cog generate-completions zsh > ~/.zfunc/_cog
coco generate-completions zsh > ~/.zfunc/_coco
```

