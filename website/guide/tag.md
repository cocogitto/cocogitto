---
editLink: true
---

# Tag prefix

It is common to use a tag prefix when creating version in your repository. This is described in the [SemVer specification
FAQ](https://semver.org/#is-v123-a-semantic-version). This convention provide a handy way to distinguish between release
versions and tags that does not represent releases.

To tell `cog` to pick only version starting with a prefix set this in your `cog.toml` file:

```toml
tag_prefix = "v"
```

Full tag including the prefix can be accessed the in configuration file with {{version_tag}}

tag_prefix = "v"

```toml
post_bump_hooks = [
"git push origin {{version_tag}}"
]
```