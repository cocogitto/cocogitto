# Introduction

Cocogitto comes with two standalone binaries : `coco` and `cog`.

Both of them and their subcommands have a `--help`  to display options and usage : 

```shell script
cog --help
cog changelog --help
 # And so on...
```

Note that if you do not care about automatic release, changelog generation and just want 
to create conventional commit message you can jump to the [conventional commits section](conventional-commits.md)

## Conventional commits  with `coco`

`coco` is primarily meant to be used as a replacement for the `git commit` command.
It will produce commits with messages respecting the conventional commits specification with
little effort.

**Example :**

```shell script
# With git commit
git commit -m "feat: implement the parser specification"

# With coco
coco feat "implement the parser specification"
```

Using `coco` while working on a project using conventional commits is less verbose than good old git cli and prevent
typos and common mistake when writing conventional commit messages. 

## Repository management with `cog`

While local commit are made with the `coco` binary, `cog` is meant to manage your repository both locally and in a 
CI context. Each subcommands will be explained further in the next chapters.

| Command | Description |
|:--------|:------------|
| check         | Verify all commit message against the conventional commit specification | 
| init          | Install cog config files                                                | 
| edit          | Rename all invalid commit message in the repo (interactive rebase)      | 
| log           | Like git log but for conventional commits                               | 
| verify        | Verify a single commit message                                          | 
| changelog     | Display a changelog for a given commit oid range                        | 
| bump          | Commit changelog from latest tag to HEAD and create a new tag           | 
| install-hook  | Add conventional git hooks to the repository                            |