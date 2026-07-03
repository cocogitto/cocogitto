---
editLink: true
---

# Repository initialization

To get the most out of cocogitto you need to have a `cog.toml` config at the root of your repository.
You can create this file manually or generate the default one with `cog init`.

<div style="position:relative;padding-bottom:56.25%;height:0;overflow:hidden;max-width:100%;margin:1.5rem 0;border-radius:10px;border:1px solid rgba(128,128,128,0.2);box-shadow:0 4px 20px rgba(0,0,0,0.15);">
  <iframe src="https://bisque.cloud/p/github/cocogitto-cocogitto" title="Cocogitto — narrated walkthrough" loading="lazy"
    style="position:absolute;top:0;left:0;width:100%;height:100%;border:0;"
    allow="autoplay; fullscreen; encrypted-media" allowfullscreen></iframe>
</div>

## Create a new repository

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

