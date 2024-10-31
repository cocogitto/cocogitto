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