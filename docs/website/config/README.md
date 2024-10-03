# Configuration reference

The config reference list all value that can be set in the `cog.toml` file at the root of a repository.

## General

### `from_latest_tag`

- Type: `boolean`
- Optional: `true`
- Default: `false`
- Description: Whether to only consider commits since the latest SemVer tag.
- Example:
  ```toml
  from_latest_tag = true
  ```

### `tag_prefix`

- Type: `String`
- Optional: `true`
- Description: Set a tag prefix value for cocogitto. For instance if you have a `v` as a tag prefix, cocogitto will
  generate versions starting with `v` and commands like `cog changelog` will pick only those versions.
- Example:
  ```toml
  tag_prefix = "v"
  ```
- Also see:

  [User guide -> Tag prefix](../guide/#tag-prefix)

### `branch_whitelist`

- Type: `Array<String>`
- Optional: `true`
- Description: A list of glob patterns to allow bumping only on matching branches.
- Example:
  ```toml
  branch_whitelist = [ "main", "release/**" ]
  ```
- Also see:

  [User guide -> Branch whitelist](../guide/#branch-whitelist)

### `ignore_merge_commits`

- Type: `boolean`
- Optional: `true`
- Default: `false`
- Description: Whether to ignore or to lint merge commits.
- Example:
  ```toml
  ignore_merge_commits = true
  ```
- Also see:

  [User guide -> Deal with merge commit](../guide/#deal-with-merge-commit)

## Commit Types

- Type: `Hashmap<String, CommitType>`
- Optional: `true`
- Description: Extend the allowed commit types, creating a new `cog commit` command, and allowing generation of changelog entries for the
  given type. Can also be used to override the default commit types.
- Example:
  ```toml
  [commit_types]
  hotfix = { changelog_title = "Hotfixes" }
  chore = { changelog_title = "Hotfixes", omit_from_changelog = true }
  release = { changelog_title = "Releases" }
  feat = {} # disabled
  ```
- Also see:

  [User guide -> Custom commit types](../guide/#custom-commit-types)

### `changelog_title`

- Type: `String`
- Optional: `false`
- Description: Change the changelog title for the given commit type.
- Example:
  ```toml
  [commit_types]
  chore = { changelog_title = "Misc" }
  hotfix = { changelog_title = "Hot fix" }
  ```

### `omit_from_changelog`

- Type: `Bool`
- Optional: `true`
- Default value: `false`
- Description: Do not generate changelog entries for the given commit type.
- Example:
  ```toml
  [commit_types]
  chore = { changelog_title = "Chore", omit_from_changelog = true }
  ```

### `bump_patch`

- Type: `Bool`
- Optional: `true`
- Default value: `false`
- Description: Commits of this type will bump the patch version of a tag.
- Example:
  ```toml
  [commit_types]
  chore = { changelog_title = "Chore", bump_patch = true }
  ```

### `bump_minor`

- Type: `Bool`
- Optional: `true`
- Default value: `false`
- Description: Commits of this type will bump the minor version of a tag.
- Example:
  ```toml
  [commit_types]
  chore = { changelog_title = "Chore", bump_minor = true }
  ```

## Bump config

### `pre_bump_hooks`

- Type: `Array<String>`
- Optional: `true`
- Description: An array of commands to execute before a version bump.
- Example:
  ```toml
  pre_bump_hooks = [
      "sh -c \"./ci/check_branch.sh\"",
      "cargo test -- --test-threads 1",
      "cargo clippy",
      "cargo build --release",
      "cargo fmt --all",
      "cargo bump {{version}}",
  ]
  ```
- Also see:

  - [User guide -> Automatic Versioning](../guide/#auto-bump)
  - [User guide -> Pre-bump hooks](../guide/#pre-bump-hooks)
  - [User guide -> Version DSL](../guide/#version-dsl)

### `post_bump_hooks`

- Type: `Array<String>`
- Optional: `true`
- Description: An array of commands to execute after a version bump.
- Example:
  ```toml
  post_bump_hooks = [
      "git push",
      "git push origin {{version}}",
      "cargo package",
      "cargo publish"
  ]
  ```
- Also see:

  - [User guide -> Automatic Versioning](../guide/#auto-bump)
  - [User guide -> Post-bump hooks](../guide/#post-bump-hooks)
  - [User guide -> Version DSL](../guide/#version-dsl)

### `bump_profiles`

- Type: `Hashmap<String, BumpProfile>`
- Optional: `true`
- Description: Add additional [pre-bump](./#pre_bump_hooks) and [post-bump](./#post_bump_hooks) hooks profile.
  a profile can be used with the `cog bump --hook-profile <profile_name>` flag.
- Example:

  ```toml
  [bump_profiles.hotfix]
  pre_bump_hooks = [
      "cargo build --release",
      "cargo fmt --all",
      "cargo bump {{version}}",
  ]

  post_bump_hooks = [
      "cargo package",
      "cargo publish"
  ]
  ```

- Also see:

  - [User guide -> Automatic Versioning](../guide/#auto-bump)
  - [User guide -> Post-bump hooks](../guide/#post-bump-hooks)
  - [User guide -> Pre-bump hooks](../guide/#pre-bump-hooks)
  - [User guide -> Version DSL](../guide/#version-dsl)
  - [User guide -> Bump profiles](../guide/guide/#bump-profiles)

### `skip_ci`

- Type: `String`
- Optional: `true`
- Description: A "skip-ci" string to add to the commits when using the `bump` or `commit` commands. Default value is `[skip ci]`.
- Example:
  ```toml
  skip_ci = "[ci-skip]"
  ```
- Also see:
  - [User guide -> Make Cocogitto skip CI CD](../guide/#make-cocogitto-skip-ci-cd)

### `skip_untracked`

- Type: `boolean`
- Optional: `true`
- Default value: `false`
- Description: Allows to perform bump even if there are untracked or uncommited changes.
- Example:
  ```toml
  skip_untracked = true
  ```
- Also see:
  - [User guide -> Skip untracked or uncommited changes](../guide/#skip-untracked-or-uncommited-changes)

### `disable_changelog`

- Type: `boolean`
- Optional: `true`
- Default value: `false`
- Description: Disable changelog generation when bumping.
- Example:
  ```toml
  disable_changelog = true
  ```

### `disable_bump_commit`

- Type: `boolean`
- Optional: `true`
- Default value: `false`
- Description: Cocogitto will not create a bump commit and will instead tag the latest commit.
- Example:
  ```toml
  disable_bump_commit = true
  ```
- Also see:
  - [User guide -> Disable bump commit creation](../guide/#disable-bump-commit-creation)

## Changelog

- Type: `Changelog`
- Optional: `true`
- Description: Set the configuration for the changelog generation.
- Example:
  ```toml
  [changelog]
  path = "CHANGELOG.md"
  template = "remote"
  remote = "github.com"
  repository = "cocogitto"
  owner = "cocogitto"
  authors = [
    { signature = "Paul Delafosse", username = "oknozor" },
    { signature = "Jack Dorland", username = "jackdorland" },
    { signature = "Mike Lubinets", username = "mersinvald" },
    { signature = "Marcin Puc", username = "tranzystorek-io" },
    { signature = "Renault Fernandes", username = "renaultfernandes" },
    { signature = "Pieter Joost van de Sande", username = "pjvds" },
    { signature = "orhun", username = "orhun" },
    { signature = "Danny Tatom", username = "its-danny" },
  ]
  ```
- Also see:
  - [User guide -> Changelog](../guide/#changelogs)

### `path`

- Type: `String`
- Optional: `true`
- Default value: `"CHANGELOG.md"`
- Description: Path the repository markdown changelog.
- Example:
  ```toml
  [changelog]
  path = "my_changelog.md"
  ```
- Also see:

  - [User guide -> Automatic Versioning](../guide/#auto-bump)
  - [User guide -> Changelog](../guide/#changelogs)

### `template`

- Type: `String`
- Optional: `true`
- Default value: `"default"`
- Description: Name of the builtin template to use for changelog generation or path to a custom template.
  Note that `remote`, `repository` and `owner` are mandatory if the "remote" built-in template is used or if your
  custom template make use of those variables.
- Built-in templates : `default`, `remote`, `full_hash`
- Example:
  ```toml
  [changelog]
  template = "full_hash"
  ```
- Also see:

  - [User guide -> Changelog](../guide/#changelogs)
  - [User guide -> Built-in templates](../guide/#built-in-templates)
  - [User guide -> Custom templates](../guide/#custom-templates)

### `package_template`

- Type: `String`
- Optional: `true`
- Default value: `"package_default"`
- Description: Name of the builtin template to use for package changelog generation or path to a custom template.
  Note that `remote`, `repository` and `owner` are mandatory if the "remote" built-in template is used or if your
  custom template make use of those variables.
- Built-in templates : `package_default`, `package_remote`, `package_full_hash`
- Example:
  ```toml
  [changelog]
  package_template = "package_full_hash"
  ```

### `remote`

- Type: `String`
- Optional: `true`
- Description: Domain name of the git platform hosting the repository, used for Markdown link generation in changelogs.
  when provided `repository` and `owner` are also required.
- Example:
  ```toml
  [changelog]
  template = "remote"
  remote = "github.com"
  repository = "cocogitto"
  owner = "cocogitto"
  ```
- Also see:

  - [User guide -> Changelog](../guide/#changelogs)
  - [User guide -> Built-in templates](../guide/#buiLt-in-templates)
  - [User guide -> Custom templates](../guide/#custom-templates)

### `repository`

- Type: `String`
- Optional: `true`
- Description: Name of the repository on the remote git platform.
- Example:
  ```toml
  [changelog]
  template = "remote"
  remote = "github.com"
  repository = "cocogitto"
  owner = "cocogitto"
  ```
- Also see:

  - [User guide -> Changelog](../guide/#changelogs)
  - [User guide -> Built-in templates](../guide/#buiLt-in-templates)
  - [User guide -> Custom templates](../guide/#custom-templates)

### `owner`

- Type: `String`
- Optional: `true`
- Description: Owner of the repository on the remote git platform.
- Example:
  ```toml
  [changelog]
  template = "remote"
  remote = "github.com"
  repository = "cocogitto"
  owner = "cocogitto"
  ```
- Also see:

  - [User guide -> Changelog](../guide/#changelogs)
  - [User guide -> Built-in templates](../guide/#buiLt-in-templates)
  - [User guide -> Custom templates](../guide/#custom-templates)

### `authors`

- Type: `Array<Author>`
- Optional: `true`
- Description: A list of commit authors with their git signature and git platform username to generate Markdown links in changelogs.
- Example:
  ```toml
    [changelog]
    authors = [
      { signature = "Paul Delafosse", username = "oknozor" },
      { signature = "Jack Dorland", username = "jackdorland" },
      { signature = "Mike Lubinets", username = "mersinvald" },
      { signature = "Marcin Puc", username = "tranzystorek-io" },
      { signature = "Renault Fernandes", username = "renaultfernandes" },
      { signature = "Pieter Joost van de Sande", username = "pjvds" },
      { signature = "orhun", username = "orhun" },
      { signature = "Danny Tatom", username = "its-danny" },
  ]
  ```
- Also see:

  - [User guide -> Changelog](../guide/#changelogs)
  - [User guide -> Built-in templates](../guide/#buiLt-in-templates)

## Mono-repository config

### `monorepo_version_separator`

- Type: `String`
- Optional: `true`
- Description: Set a package tag separator. For instance if you have a `-` as package separator, cocogitto will
  generate monorepo package version starting with the package name followed by the optional prefix and package version (ex: `my-package-v1.0.0`)
- Example:
  ```toml
  monorepo_version_separator = "-"
  ```

### `pre_package_bump_hooks`

- Type: `Array<String>`
- Optional: `true`
- Description: An array of commands executed before every package bump.
- Example:
  ```toml
  pre_bump_hooks = [
      "cargo build --release",
      "cargo fmt --all",
      "cargo set-version {{version}}",
  ]
  ```

### `post_package_bump_hooks`

- Type: `Array<String>`
- Optional: `true`
- Description: An array of commands executed after every package bump.
- Example:
  ```toml
  post_bump_hooks = [
      "cargo build --release",
      "cargo fmt --all",
      "cargo set-version {{version}}",
  ]
  ```

## Mono-repository packages

- Type: `Hashmap<String, MonoRepoPackage>`
- Optional: `true`
- Description: Add packages that will be included when doing `cog bump`.
- Example:
  ```toml
  [packages]
  my_package = { 
    path = "packages/my_package",
    public_api = false,
    include = [],
    ignore = [],
    pre_bump_hooks = [],
    post_bump_hooks = []
  }
  my_other_package = {
    path = "packages/my_other_package",
  }
  ```
 - See also
   [User guide -> Packages configuration](../guide/README.md#packages-configuration)

### `path`

- Type: `String`
- Optional: `false`
- Description: Set the package path.
- Example:
  ```toml
  [packages]
  my_package = { path = "packages/my_package" }
  ```

### `include`

- Type: `Array<String>`
- Optional: `true`
- Description: An array of additional path globs to consider part of the package.
  These additional paths will trigger a package version bump in addition to the path in `path`.
  The globs are evaluated with [globset](https://crates.io/crates/globset) with the `literal_separator` option enabled.
- Example:
  ```toml
  [packages]
  my_package = { path = "packages/my_package", include = ["common/**"] }
  ```

### `ignore`

- Type: `Array<String>`
- Optional: `true`
- Description: An array of path globs to ignore as part of the package.
  These paths will never trigger a package version bump (even if they normally would based on `path` and `include`).
  The globs are evaluated with [globset](https://crates.io/crates/globset) with the `literal_separator` option enabled.
- Example:
  ```toml
  [packages]
  my_package = { path = "packages/my_package", include = ["packages/my_package/.github/**"] }
  ```

### `changelog_path`

- Type: `String`
- Optional: `true`
- Default: `{path}/CHANGELOG.md`
- Description: Overrides the default changelog path, allowing to set an absolute path.
- Example:
  ```toml
  [packages]
  my_package = { path = "packages/my_package", changelog_path = "changelogs/my_package.md" }
  ```

### `public_api`

- Type: `boolean`
- Optional: `true`
- Default: `true`
- Description: If set to `false` the package will not trigger global version bump.
- Example:
  ```toml
  [packages]
  my_package = { path = "packages/my_package", public_api = false }
  ```
- Also see:

  [User guide -> Package configuration](../guide/#packages-configuration)

### `pre_bump_hooks`

- Type: `Array<String>`
- Optional: `true`
- Description: An array of commands to execute before a package bump.
- Example:

  ```toml
  pre_bump_hooks = [
      "cargo build --release",
      "cargo fmt --all",
      "cargo set-version {{version}}",
  ]
  ```

- Also see:

  - [User guide -> Automatic Versioning](../guide/#auto-bump)
  - [User guide -> Automatic versioning for monorepo](../guide/#packages-hooks)
  - [User guide -> Post-bump hooks](../guide/#post-bump-hooks)
  - [User guide -> Version DSL](../guide/#version-dsl)

### `post_bump_hooks`

- Type: `Array<String>`
- Optional: `true`
- Description: An array of commands to execute after a version bump.
- Example:
  ```toml
  post_bump_hooks = [
      "echo {{latest}} bumped to {{version}}",
  ]
  ```
- Also see:

  - [User guide -> Automatic Versioning](../guide/#auto-bump)
  - [User guide -> Automatic versioning for monorepo](../guide/#packages-hooks)
  - [User guide -> Post-bump hooks](../guide/#post-bump-hooks)
  - [User guide -> Version DSL](../guide/#version-dsl)

### `bump_profiles`

- Type: `Hashmap<String, BumpProfile>`
- Optional: `true`
- Description: Add additional per package [pre-bump](./#pre_bump_hooks) and [post-bump](./#post_bump_hooks) hooks profile.
  a profile can be used with the `cog bump --hook-profile <profile_name>` flag.
- Example:

  ```toml
  [packages.my-package]
  path = "packages/my-package"

  [bump_profiles.hotfix]
  pre_bump_hooks = [
      "cargo build --release",
      "cargo fmt --all",
      "cargo set-version {{version}}",
  ]

  post_bump_hooks = [
      "cargo package",
  ]
  ```
