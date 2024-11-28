---
title: Conventional commit log
editLink: true
prev:
  text: Rewrite non-compliant commits
  link: '/guide/edit'
next:
  text: Changelogs
  link: '/guide/changelog'
---

# Conventional commit log

`cog log` is just like `git log` but it displays additional conventional commit information, such as commit scope,
commit type etc.

[![asciicast](https://asciinema.org/a/ssH4yRSlc28Rb9dHEDN7TowGe.svg)](https://asciinema.org/a/ssH4yRSlc28Rb9dHEDN7TowGe)

::: tip
You can filter the log content with the following flags:

- `-B`: display breaking changes only
- `-t`: filter on commit types
- `-a`: filter on commit authors
- `-s`: filter on commit scopes
- `-e`: ignore non compliant commits

Those flag can be combined to achieve complex search in your commit history:

```bash
cog log --author "Paul Delafosse" "Mike Lubinets" --type feat --scope cli --no-error
```

:::