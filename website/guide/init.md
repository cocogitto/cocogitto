---
editLink: true
---

# Repository initialization

To get the most out of cocogitto you need to have a `cog.toml` config at the root of your repository.
You can create this file manually or generate the default one with `cog init`.

## Create a new repository

```bash
mkdir my_repo && cd my_repo
cog init
```

`cog init` works like `git init` except it create a template `cog.toml` config file,
and an initial commit with the following message: `chore: initial commit`.

You can specify the target path of the repository you want to create:

```bash
cog init my_repo
```

## Initialize an existing repo

Running `cog init` on an existing repository will just create a template configuration without creating any commit:

```bash
git init my_repo && cd my_repo
cog init
```

```bash
❯ git status
On branch master
Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
	new file:   cog.toml
```