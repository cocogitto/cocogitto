---
editLink: true
---

# Check commit history

Running `cog check` will check your commit history against the conventional commit specification:

```bash
❯ cog check
No errored commits
```

Let us create an invalid commit:

```bash
git commit -m "Your Mother Was A Hamster, And Your Father Smelt Of Elderberries"
```

And check our commit history again:

```bash
❯ cog check
Error:
Found 1 non compliant commits in db5151..HEAD:

________________________________________________________

Errored commit: db5151486a41f1b694fd8f90144dd02c87268988 <Paul Delafosse>
	Commit message: 'Your Mother Was A Hamster, And Your Father Smelt Of Elderberries'
	Error: Missing commit type separator `:`

	Caused by:
	     --> 1:5
	      |
	    1 | Your Mother Was A Hamster, And Your Father Smelt Of Elderberries
	      |     ^---
	      |
	      = expected scope or type_separator
```

::: tip
You can check your history, starting from the latest tag using `--from-latest-tag` or `-l` flag.
This is useful when your git repo started to use conventional commits from a certain point in history and you
don't care about editing old commits.
:::