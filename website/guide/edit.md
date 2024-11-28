---
title: Rewrite non-compliant commits
editLink: true
prev:
  text: Sandbox
  link: '/guide/verify'
next:
  text: 'Conventional commit log'
  link: '/guide/log'
---

# Rewrite non-compliant commits

::: danger
Using `cog edit` will modify your commit history and change the commit SHA of edited commit
and their child.
:::

Once you have spotted invalid commits you can quickly fix your commit history by running `cog edit`.
This will perform an automatic rebase, cycling through each malformed commit, and letting you edit them with your `$EDITOR`
of choice.

**Example:**

```editor   
# Editing commit c2bb56b93821ff34282f322be4d2231f53b9ada8
# Replace this message with a conventional commit compliant one
# Save and exit to edit the next errored commit
Your Mother Was A Hamster, And Your Father Smelt Of Elderberries
```

::: tip
You can use the `--from-latest-tag` or `-l` to edit only commits since the latest tag in your repository.
:::
