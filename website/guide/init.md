---
editLink: true
---

# Repository initialization

To get the most out of cocogitto you need to have a `cog.toml` config at the root of your repository.
You can create this file manually or generate the default one with `cog init`.

## Adding the `cog.toml` config file

### ...to initialize a new repository with cocogitto

```bash
mkdir my_repo && cd my_repo
cog init
```

`cog init` works like `git init` except it creates a template `cog.toml` config file,
and an initial commit with the following message: `chore: initial commit`.

You can specify the target path of the repository you want to create:

```bash
cog init my_repo
```

### ...to setup cocogitto in an existing repo

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

## Setting the tag prefix

Cocogitto requires setting a specific tag prefix. For most conventional-commits
versioning schemes this is just the string `v`. However, it can be different
based on your use case.

For this reason, add the `tag_prefix` option to the `cog.toml` file and set it
to your preferred prefix.

Example:

```toml
# The tag_prefix allows discovering the existing tags
tag_prefix = "v"

```
