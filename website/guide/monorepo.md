---
editLink: true
---

# Automatic versioning for monorepo

Managing versions for mono-repository is slightly different from the standard Cocogitto flow.
Instead of the [standard bump steps](#automatic-versioning) using `cog bump --auto` on a mono-repository will
perform the following actions:

1. Calculate next version for each package based on commits that changes the package content.
2. Calculate a global version based on the created package versions and the commit that does not belong to a specific package.
3. Execute global pre-bump hooks.
4. Execute per package pre-bump hooks.
5. Append the changes for each package to `{package_path}/CHANGELOG.md`.
6. Append global changes and a list of package version to `/CHANGELOG.md`.
7. Create a version commit containing changes made during the previous steps.
8. Create global git tag on the version commit.
9. Create a tag for each new package version on the version commit.
10. Execute per package post-bump hooks.
11. Execute global post-bump hooks.

## Mono-repository bump

When using `cog bump` in a mono-repository context, it behaves slightly differently.

- `cog bump --auto`: creates a tag per changed packages since their respective latest releases and creates a global
  mono-repository tag.
- `cog bump` used why manual bump flags such as `--minor`, `--major`, `--patch` or `--version` will only
  create the monorepo version without bumping packages.

- `cog bump --package=my_package --auto` creates a single package tag from the latest package tag

:::tip
We strongly advise to use automatic bump whenever possible. Manual bump should only be used when there are changes that
Cocogitto is not able to detect (ex: a breaking change occurring in a package via updating a global dependency).
:::

## Packages configuration

To set up mono-repository support you only need to define a list of package in your `cog.toml`
config. Once packages are defined, `cog` will automatically scan your packages during automatic version bump.

**Example:**

A real life example from [oknozor/gill](https://github.com/oknozor/gill/blob/main/cog.toml).

```toml
[packages]
gill-app = { path = "crates/gill-app" }
gill-authorize-derive = { path = "crates/gill-authorize-derive", public_api = false }
gill-db = { path = "crates/gill-db", public_api = false }
gill-git = { path = "crates/gill-git", public_api = false }
gill-git-server = { path = "crates/gill-git-server" }
gill-markdown = { path = "crates/gill-markdown", public_api = false }
gill-settings = { path = "crates/gill-settings" }
gill-syntax = { path = "crates/gill-syntax" }
gill-web-markdown = { path = "crates/gill-web-markdown" }
syntect-plugin = { path = "crates/syntect-plugin", public_api = false }
```

:::tip
If some of your packages does not belong to your project public API use `public_api = false` to prevent `--auto` bump
from updating the global project version.
:::

### Packages hooks

When creating a monorepo version Cocogitto will execute the pre-bump and post-bump hooks normally. Additionally, it will
run `pre_package_bump_hooks` and `post_package_bump_hooks` before and after each package bump.
To override these you can define per package hooks.

**Example:**

```shell
## Pre hooks executed before each package bump, here we use a cargo command to bump rust package manifest
pre_package_bump_hooks = [
    "echo 'upgrading {{package}}' to {{version}}",
    "cargo set-version {{version}}"
]

[packages]
rust-package-one = { path = "packages/rust-one" }
rust-package-two = { path = "packages/rust-two" }
## We have a java project in the mono-repository so we override the default pre-hook
java-package = { path = "packages/java-package", pre_bump_hooks = [ "mvn build" ] }
```

:::tip
Note that for package hooks, you can use the `package` variable from version DSL to get the current package name.
:::
