---
editLink: true
---

## Make Cocogitto skip CI CD

The `--skip-ci` option of the `bump` and `commit` commands offers the possibility to skip CI/CD by adding a "skip-ci" string your commits. The default string used by Cocogitto is `[skip ci]` but you can override it with your own string:

- Using the `skip_ci` configuration in the `cog.toml`.
- Using the `--skip-ci-override` option of the `bump` and `commit` commands.

Note that the `--skip-ci-override` option has precedence over the `skip_ci` configuration in the `cog.toml`.

**Example:**

```bash
❯ cog bump --skip-ci
```

**Result:**

```bash
❯ git log
commit xxx (HEAD -> main, tag: 1.0.0)
Author: John Doe <jon.doe@unknown.com>
Date: Tue Mar 7 15:06:18 2023 +0200
    chore(version): 1.0.0 [skip ci]
```

**Example with `cog.toml` configuration:**

```toml
skip_ci = "[ci-skip]"
```

```bash
❯ cog bump --skip-ci
```

**Result:**

```bash
❯ git log
commit xxx (HEAD -> main, tag: 1.0.0)
Author: John Doe <jon.doe@unknown.com>
Date: Tue Mar 7 15:06:18 2023 +0200
    chore(version): 1.0.0 [ci-skip]
```

**Another example using the `--skip-ci-override` option:**

```bash
❯ cog bump --skip-ci-override "[ci-skip]"
```

**Result:**

```bash
❯ git log
commit xxx (HEAD -> main, tag: 1.0.0)
Author: John Doe <jon.doe@unknown.com>
Date: Tue Mar 7 15:06:18 2023 +0200
    chore(version): 1.0.0 [ci-skip]
```

## Skip untracked or uncommited changes

By default, Cocogitto will abort bump if there are uncommited or untracked changes. You can change this behavior using 
the `skip_untracked` configuration in the `cog.toml` file or the `--skip-untracked` option of the `bump` command. If so, 
the warning will be printed to `stderr` and the bump will continue.

## Disable bump commit creation

When bumping by default, Cocogitto will create a commit that will include the Changelog(s), the tag(s) and any file 
updated in the `pre_bump_hooks`.

**Example with a monorepo**:

```bash
❯ git log
commit xxx (HEAD -> main, tag: 0.1.0, tag: one-0.1.0)
Author: John Doe <jon.doe@unknown.com>
Date: Tue Mar 7 15:06:18 2023 +0200
    chore(version): bump packages
```

To disable its creation, you can use the `disable_bump_commit` configuration in the `cog.toml` or 
the `--disable-bump-commit` option of the `cog bump` command. In that case, Cocogitto will create the tag(s) on the 
latest commit and will not commit the files mentionned above. They will have to be manually commited, for example using 
the `post_bump_hooks`.

## Get the current version

It's sometime needed to display the current version for scripting purpose.
You can print the latest SemVer tag on your repo with the `get-version` subcommand:

```bash
❯ cog get-version
Current version:
5.3.1
```

To silence the additional info and get only the version use the `-v` flag:

```bash
❯ cog -v get-version
5.3.1
```

If working on a monorepo you can also specify the target package:

```bash
❯ cog -v get-version --package gill-db
0.1.0
```

Finally, if you need the command to print a version no matter the state of your repository, you can provide a fallback:

```bash
❯ cog get-version --fallback 0.1.0
0.1.0
```