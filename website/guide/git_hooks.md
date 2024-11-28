---
editLink: true
---

# Managing git hooks

Cocogitto provide a way to share [git hooks](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks).

First you need to set up some hooks in `cog.toml`:

```toml
# Embedded git-hooks script
[git_hooks.commit-msg]
script = """#!/bin/sh
set -e
cog verify --file $1
cog check
cargo fmt -v --all --check
cargo clippy
"""

# Or file path
[git_hooks.pre-push]
path = "hooks/pre-push.sh"
```

::: warning
Note that unlike `git commit`, `cog commit` will not pick a default shell when running hooks. Make sure to provide
a shebang in your hook definition.
:::

Now that our hook are defined in cocogitto's config they can be installed with `cog install-hook`.

**Install all git-hooks:**

```bash
❯ cog install-hook --all
```

**Install a single hook:**

```bash
❯ cog install-hook commit-msg
```