# Configuration reference

The config reference list all value that can be set in the `cog.toml` file at the root of a repository.

## `Settings`
- **Description :** Configuration structure for the Cocogitto tool.

 This struct defines the main configuration options for Cocogitto, including settings
 for version generation, changelog handling, commit conventions, hooks, and monorepo support.

  **Example :**
 ```toml
 # Basic settings
 from_latest_tag = true
 ignore_merge_commits = true

 # Changelog settings
 [changelog]
 path = "CHANGELOG.md"
 template = "remote"

 # Git hooks
 [git_hooks.pre-commit]
 script = "./scripts/pre-commit.sh"

 # Monorepo configuration
 [packages.my-package]
 path = "packages/my-package"
 ```
## `branch_whitelist`
- **Description :** A list of glob patterns to allow bumping only on matching branches.
- **Type :** `Array`
- **Default :**
```toml
branch_whitelist = []
```
- **Type :** `String`

## `bump_profiles`
- **Description :** Custom bump profiles configurations.
- **Type :** `Map<String, BumpProfile>`
- **Default :**
```toml
[bump_profiles]
```

## `changelog`
- **Description :** Changelog configuration.
- **Type :** [Changelog](#Changelog)
- **Default :**
```toml
[changelog]
authors = []
owner = "null"
package_template = "null"
path = "CHANGELOG.md"
remote = "null"
repository = "null"
template = "null"
```

## `commit_types`
- **Description :** Custom commit types configuration.
- **Type :** `Map<String, CommitConfig>`
- **Default :**
```toml
[commit_types]
```

## `disable_bump_commit`
- **Description :** Whether to create a bump commit or not.
- **Type :** `Boolean`
- **Default :**
```toml
disable_bump_commit = false
```

## `disable_changelog`
- **Description :** Whether to generate a changelog or not during bump.
- **Type :** `Boolean`
- **Default :**
```toml
disable_changelog = false
```

## `from_latest_tag`
- **Description :** Whether to only consider commits since the latest SemVer tag.
- **Type :** `Boolean`
- **Default :**
```toml
from_latest_tag = false
```

## `generate_mono_repository_global_tag`
- **Description :** Activate or deactivate global tag generation for mono-repository.
- **Type :** `Boolean`
- **Default :**
```toml
generate_mono_repository_global_tag = true
```

## `generate_mono_repository_package_tags`
- **Description :** Activate or deactivate package tag generation for mono-repository.
- **Type :** `Boolean`
- **Default :**
```toml
generate_mono_repository_package_tags = true
```

## `git_hooks`
- **Description :** Git hooks configuration.
- **Type :** `Map<GitHookType, GitHook>`
- **Default :**
```toml
[git_hooks]
```
ref #/$defs/GitHookType

## `ignore_fixup_commits`
- **Description :** Silently ignore fixup commits
- **Type :** `Boolean`
- **Default :**
```toml
ignore_fixup_commits = true
```

## `ignore_merge_commits`
- **Description :** A list of glob patterns to allow bumping only on matching branches.
- **Type :** `Boolean`
- **Default :**
```toml
ignore_merge_commits = false
```

## `monorepo_version_separator`
- **Description :** Specify the version separator character for mono-repository package's tags.
- **Type :** `String | Null`

## `packages`
- **Description :** Monorepo packages configuration.
- **Type :** `Map<String, MonoRepoPackage>`
- **Default :**
```toml
[packages]
```

## `post_bump_hooks`
- **Description :** Hooks that will be executed after a bump command in root dir.
- **Type :** `Array`
- **Default :**
```toml
post_bump_hooks = []
```
- **Type :** `String`

## `post_package_bump_hooks`
- **Description :** Hooks that will be executed after a bump command in package dir.
- **Type :** `Array`
- **Default :**
```toml
post_package_bump_hooks = []
```
- **Type :** `String`

## `pre_bump_hooks`
- **Description :** Hooks that will be executed before a bump command in root dir.
- **Type :** `Array`
- **Default :**
```toml
pre_bump_hooks = []
```
- **Type :** `String`

## `pre_package_bump_hooks`
- **Description :** Hooks that will be executed before a bump command in package dir.
- **Type :** `Array`
- **Default :**
```toml
pre_package_bump_hooks = []
```
- **Type :** `String`

## `scopes`
- **Description :** List of valid commit scopes.
- **Type :** `Array | Null`
- **Type :** `String`

## `skip_ci`
- **Description :** A "skip-ci" string to add to the commits when using the `bump` or `commit` commands.
 Default value is `[skip ci].
- **Type :** `String`
- **Default :**
```toml
skip_ci = "[skip ci]"
```

## `skip_untracked`
- **Description :** Allows to perform bump even if there are untracked or uncommitted changes.
- **Type :** `Boolean`
- **Default :**
```toml
skip_untracked = false
```

## `tag_prefix`
- **Description :** Set a tag prefix value for cocogitto. For instance if you have a `v`
 as a tag prefix, cocogitto will generate versions starting with `v` and
 commands like `cog changelog` will pick only those versions.
- **Type :** `String | Null`

## AuthorSetting
- **Description :** Configuration for mapping Git signatures to usernames.

 This struct defines the mapping between a Git commit signature (email address)
 and the corresponding username to use in changelog generation.

  **Example :**
 ```toml
 [[changelog.authors]]
 signature = "user@example.com"
 username = "githubuser"
 ```
### `signature` <Badge type="danger" text="required" />
- **Description :** The Git commit signature (typically an email address)
- **Type :** `String`

### `username` <Badge type="danger" text="required" />
- **Description :** The username to display in changelogs
- **Type :** `String`


## BumpProfile
- **Description :** A custom profile for configuring hooks that run before and after version bumps.

 Bump profiles allow defining different sets of hooks that can be selected
 when running bump commands.

  **Example :**
 ```toml
 [bump_profiles.production]
 pre_bump_hooks = ["./scripts/pre-release.sh"]
 post_bump_hooks = ["./scripts/post-release.sh"]
 ```
### `post_bump_hooks`
- **Description :** List of hooks to run after bumping the version
- **Type :** `Array`
- **Default :**
```toml
post_bump_hooks = []
```
- **Type :** `String`

### `pre_bump_hooks`
- **Description :** List of hooks to run before bumping the version
- **Type :** `Array`
- **Default :**
```toml
pre_bump_hooks = []
```
- **Type :** `String`


## Changelog
- **Description :** Configuration for changelog generation.

 This struct defines how the changelog should be generated,
 including templates, remote repository information, and author settings.

  **Example :**
 ```toml
 [changelog]
 template = "remote"
 path = "CHANGELOG.md"
 remote = "github.com"
 owner = "cocogitto"
 repository = "cocogitto"
 ```
### `authors`
- **Description :** Author mappings for changelog generation
- **Type :** `Array`
- **Default :**
```toml
authors = []
```
ref #/$defs/AuthorSetting

### `owner`
- **Description :** Repository owner/organization name
- **Type :** `String | Null`

### `package_template`
- **Description :** Template to use for package changelogs in monorepos
- **Type :** `String | Null`

### `path`
- **Description :** Path where changelog file should be written
- **Type :** `String`
- **Default :**
```toml
path = "CHANGELOG.md"
```

### `remote`
- **Description :** Remote Git repository URL (e.g. "github.com")
- **Type :** `String | Null`

### `repository`
- **Description :** Repository name
- **Type :** `String | Null`

### `template`
- **Description :** Template to use for changelog generation. Can be "remote", "full_hash" or a custom template path
- **Type :** `String | Null`


## CommitConfig
- **Description :** Configurations to create new conventional commit types or override behaviors of the existing ones.
### `bump_minor`
- **Description :** Allow for this commit type to bump the minor version.
- **Type :** `Boolean | Null`

### `bump_patch`
- **Description :** Allow for this commit type to bump the patch version.
- **Type :** `Boolean | Null`

### `changelog_title`
- **Description :** Define the title used in generated changelog for this commit type.
- **Type :** `String | Null`

### `omit_from_changelog`
- **Description :** Do not display this commit type in changelogs.
- **Type :** `Boolean | Null`

### `order`
- **Description :** Specify a sort order attribute for this commit type.
- **Type :** `Integer | Null`


## GitHook
- **Description :** A GitHook can be defined either as a script string that will be executed directly,
 or as a path to a script file that will be executed

## GitHookType
- **Description :** Represents the different types of Git hooks that can be configured.

 This enum defines all the standard Git hook types that can be used
 in the configuration. Each variant corresponds to a specific Git hook
 that gets triggered at different points in Git's execution.

  **Example :**
 ```toml
 [git_hooks.pre-commit]
 script = "./scripts/pre-commit.sh"
 ```
- **Type :** `String`
- **Possible values :** `applypatch-msg`, `pre-applypatch`, `post-applypatch`, `pre-commit`, `pre-merge-commit`, `pre-prepare-commit-msg`, `commit-msg`, `post-commit`, `pre-rebase`, `post-checkout`, `post-merge`, `pre-push`, `pre-auto-gc`, `post-rewrite`, `sendemail-validate`, `fsmonitor-watchman`, `p4-changelist`, `p4-prepare-changelist`, `p4-postchangelist`, `p4-pre-submit`, `post-index-change`

## MonoRepoPackage
- **Description :** Configuration for a package in a monorepo setup.

 This struct defines how a single package within a monorepo should be handled,
 including its location, included/excluded files, changelog settings, and bump behavior.

  **Example :**
 ```toml
 [packages.my-package]
 path = "packages/my-package"
 include = ["packages/my-package/**"]
 ignore = ["**/test/**"]
 changelog_path = "CHANGELOG.md"
 public_api = true
 bump_order = 1
 ```
### `bump_order`
- **Description :** Ordering of packages in the changelog, this affect in which order
 packages will be bumped.
- **Type :** `Integer | Null`

### `bump_profiles`
- **Description :** Custom profile to override `pre_bump_hooks`, `post_bump_hooks`.
- **Type :** `Map<String, BumpProfile>`
- **Default :**
```toml
[bump_profiles]
```

### `changelog_path`
- **Description :** Where to write the changelog.
- **Type :** `String | Null`

### `ignore`
- **Description :** List of globs for paths to ignore, relative to
 the repository root dir.
- **Type :** `Array`
- **Default :**
```toml
ignore = []
```
- **Type :** `String`

### `include`
- **Description :** List of globs for additional paths to include, relative to
 the repository root dir.
- **Type :** `Array`
- **Default :**
```toml
include = []
```
- **Type :** `String`

### `path`
- **Description :** The package path, relative to the repository root dir.
 Used to scan commits and set hook commands current directory.
- **Type :** `String`
- **Default :**
```toml
path = ""
```

### `post_bump_hooks`
- **Description :** Overrides `post_package_bump_hooks`.
- **Type :** `Array | Null`
- **Type :** `String`

### `pre_bump_hooks`
- **Description :** Overrides `pre_package_bump_hooks`.
- **Type :** `Array | Null`
- **Type :** `String`

### `public_api`
- **Description :** Bumping package marked as public api will increment
 the global monorepo version when using `cog bump --auto`.
- **Type :** `Boolean`
- **Default :**
```toml
public_api = true
```



