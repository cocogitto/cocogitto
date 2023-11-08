# Configuration reference

The config reference list all value that can be set in the `cog.toml` file at the root of a repository. 

## General

### `tag_prefix`

- Type: `String`
- Optional: `true`
- Description: set a tag prefix value for cocogitto. For instance if you have a `v` as a tag prefix, cocogitto will
  generate version starting with `v` and commands like `cog changelog` will pick only those versions.
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
- Description: whether to ignore or to lint merge commits.
- Example:
    ```toml
    ignore_merge_commits = true
    ```
- Also see:

  [User guide -> Deal with merge commit](../guide/#deal-with-merge-commit)

## Commit Types

- Type: `Hashmap<String, CommitType>`
- Optional: `true`
- Description: extend the allowed commit types, creating a new `cog commit` command and allowing to generate changelog entries for the
    given type. Can also be used to override the default commit types. 
- Example: 
    ```toml
    [commit_types]
    hotfix = { changelog_title = "Hotfixes" }
    chore = { changelog_title = "Hotfixes", omit_from_changelog = true }
    release = { changelog_title = "Releases" }
    ```
- Also see: 
    
    [User guide -> Custom commit types](../guide/#custom-commit-types)

### `changelog_title`
- Type: `String`
- Optional: `false`
- Description: change the changelog title for the given commit type.
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
- Description: do not generate changelog entries for the given commit type.
- Example:
    ```toml
    [commit_types]
    chore = { changelog_title = "Chore", omit_from_changelog = true }
    ```
## Bump config

### `pre_bump_hooks`

- Type: `Array<String>`
- Optional: `true`
- Description: an array of command to execute before a version bump.
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

    * [User guide -> Automatic Versioning](../guide/#auto-bump)
    * [User guide -> Pre-bump hooks](../guide/#pre-bump-hooks)
    * [User guide -> Version DSL](../guide/#version-dsl)

### `post_bump_hooks`

- Type: `Array<String>`
- Optional: `true`
- Description: an array of command to execute after a version bump.
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

    * [User guide -> Automatic Versioning](../guide/#auto-bump)
    * [User guide -> Post-bump hooks](../guide/#post-bump-hooks)
    * [User guide -> Version DSL](../guide/#version-dsl)

### `bump_profiles` 


- Type: `Hashmap<String, BumpProfile>`
- Optional: `true`
- Description: add additional [pre-bump](./#pre_bump_hooks) and [post-bump](./#post_bump_hooks) hooks profile.
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

    * [User guide -> Automatic Versioning](../guide/#auto-bump)
    * [User guide -> Post-bump hooks](../guide/#post-bump-hooks)
    * [User guide -> Pre-bump hooks](../guide/#pre-bump-hooks)
    * [User guide -> Version DSL](../guide/#version-dsl)
    * [User guide -> Bump profiles](../guide/guide/#bump-profiles)

### `skip_ci` 

- Type: `String`
- Optional: `true`
- Description: "skip_ci" string that is appended to the end of the bump commit. It can also be 
  specified using `cog bump --skip-ci <skip_ci_string>`
- Example:
    ```toml
    skip_ci = "[skip-ci]"
    ```
- Also see:
    * [User guide -> Make Cocogitto skip CI CD](../guide/#make-cocogitto-skip-ci-cd)

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
    * [User guide -> Skip untracked or uncommited changes](../guide/#skip-untracked-or-uncommited-changes)

## Changelog

### `path`

- Type: `String`
- Optional: `true`
- Default value: `"CHANGELOG.md"`
- Description: path the repository markdown changelog.
- Example:
    ```toml
    [changelog]
    path = "my_changelog.md"
    ```
- Also see:

    * [User guide -> Automatic Versioning](../guide/#auto-bump)
    * [User guide -> Changelog](../guide/#changelogs)

### `template`

- Type: `String`
- Optional: `true`
- Default value: `"default"`
- Description: name of the builtin template to use for changelog generation or path to a custom template.
  Note that `remote`, `repository` and `owner` are mandatory if the "remote" built-in template us used or if your
  custom template make use of those variables. 
- Built-in templates : `default`, `remote`, `full_hash`
- Example:
    ```toml
    [changelog]
    template = "full_hash"
    ```
- Also see:

    * [User guide -> Changelog](../guide/#changelogs)
    * [User guide -> Built-in templates](../guide/#built-in-templates)
    * [User guide -> Custom templates](../guide/#custom-templates)

### `remote`

- Type: `String`
- Optional: `true`
- Description: domain name of the git platform hosting the repository, used for Markdown link generation in changelogs.
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

    * [User guide -> Changelog](../guide/#changelogs)
    * [User guide -> Built-in templates](../guide/#buiLt-in-templates)
    * [User guide -> Custom templates](../guide/#custom-templates)

### `repository`

- Type: `String`
- Optional: `true`
- Description: name of the repository on the remote git platform.
- Example:
    ```toml
    [changelog]
    template = "remote"
    remote = "github.com"
    repository = "cocogitto"
    owner = "cocogitto"
    ```
- Also see:

    * [User guide -> Changelog](../guide/#changelogs)
    * [User guide -> Built-in templates](../guide/#buiLt-in-templates)
    * [User guide -> Custom templates](../guide/#custom-templates)

### `owner`

- Type: `String`
- Optional: `true`
- Description: owner of the repository on the remote git platform.
- Example:
    ```toml
    [changelog]
    template = "remote"
    remote = "github.com"
    repository = "cocogitto"
    owner = "cocogitto"
    ```
- Also see:

    * [User guide -> Changelog](../guide/#changelogs)
    * [User guide -> Built-in templates](../guide/#buiLt-in-templates)
    * [User guide -> Custom templates](../guide/#custom-templates)

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

    * [User guide -> Changelog](../guide/#changelogs)
    * [User guide -> Built-in templates](../guide/#buiLt-in-templates)

## Mono-repository config

### `monorepo_version_separator`

- Type: `String`
- Optional: `true`
- Description: set a package tag separator. For instance if you have a `-` as package separator, cocogitto will
  generate monorepo package version starting with the package name followed by the optional prefix and package version (ex: `my-package-v1.0.0`)
- Example:
    ```toml
    monorepo_version_separator = "-"
    ```
### `pre_package_bump_hooks`

- Type: `Array<String>`
- Optional: `true`
- Description: an array of command executed before every package bump.
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
- Description: an array of command executed after every package bump.
- Example:
    ```toml
    pre_bump_hooks = [
        "cargo build --release",
        "cargo fmt --all",
        "cargo set-version {{version}}",
    ]
    ```
  
## Mono-repository packages

### `path`
 
- Type: `String`
- Optional: `false`
- Description: set the package path.
- Example:
    ```toml
    [packages]
    my_package = { path = "packages/my_package" }
   ```
  
### `changelog_path`

- Type: `String`
- Optional: `true`
- Default: `{path}/CHANGELOG.md`
- Description: overrides the default changelog path, allowing to set an absolute path.
- Example:
    ```toml
    [packages]
    my_package = { path = "packages/my_package", changelog_path = "changelogs/my_package.md" }
   ```
### `public_api`
- Type: `boolean`
- Optional: `true`
- Default: `true`
- Description: if set to false package will not trigger global version bump.
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
- Description: an array of command to execute before a package bump.
- Example:
    ```toml
    pre_bump_hooks = [
        "cargo build --release",
        "cargo fmt --all",
        "cargo set-version {{version}}",
    ]
    ```

- Also see:

  * [User guide -> Automatic Versioning](../guide/#auto-bump)
  * [User guide ->  Automatic versioning for monorepo](../guide/#packages-hooks)
  * [User guide -> Post-bump hooks](../guide/#post-bump-hooks)
  * [User guide -> Version DSL](../guide/#version-dsl)
  
### `post_bump_hooks`
 
- Type: `Array<String>`
- Optional: `true`
- Description: an array of command to execute after a version bump.
- Example:
    ```toml
    post_bump_hooks = [
        "echo {{latest}} bumped to {{version}}",
    ]
    ```
- Also see:

  * [User guide -> Automatic Versioning](../guide/#auto-bump)
  * [User guide ->  Automatic versioning for monorepo](../guide/#packages-hooks)
  * [User guide -> Post-bump hooks](../guide/#post-bump-hooks)
  * [User guide -> Version DSL](../guide/#version-dsl)
  
### `bump_profiles`

- Type: `Hashmap<String, BumpProfile>`
- Optional: `true`
- Description: add additional per package [pre-bump](./#pre_bump_hooks) and [post-bump](./#post_bump_hooks) hooks profile.
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





