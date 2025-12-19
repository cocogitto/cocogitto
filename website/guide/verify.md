---
editLink: true
---

# Sandbox

`cog verify` checks an arbitrary input string against the conventional commit specification.
It will not create any commit.

**Example:**

```bash
❯ cog verify "Your Mother Was A Hamster, And Your Father Smelt Of Elderberries"
Error: Missing commit type separator `:`

Caused by:
     --> 1:5
      |
    1 | Your Mother Was A Hamster, And Your Father Smelt Of Elderberries
      |     ^---
      |
      = expected scope or type_separator
```

## Reading from a file

You can also verify a commit message from a file:

```bash
❯ cog verify --file commit_message.txt
```

## Reading from stdin

To read from stdin, use `-` as the file argument:

```bash
❯ echo "feat(grid): Add lightcycle battles to the grid" | cog verify --file -
Add lightcycle battles to the grid (not committed) - now
	Author: Kevin Flynn
	Type: feat
	Scope: grid
```

This is particularly useful in CI/CD pipelines or when integrating with other tools:

```bash
❯ git log -1 --pretty=%B | cog verify --file -
```
