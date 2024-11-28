
## Bump hook recipes

### Cargo library projects

A recipe for Cargo projects with a git-ignored `Cargo.lock` file, aka library projects.

Prerequisites:

- `cargo-edit`

Hooks:

```toml
pre_bump_hooks = [
  "cargo build --release",          # verify the project builds
  "cargo set-version {{version}}",  # bump version in Cargo.toml
]
post_bump_hooks = [
  "git push",
  "git push {{version}}",
]
```

### Cargo executable projects

A recipe for Cargo projects with a managed `Cargo.lock` file, aka executable projects.
Notably, the version bump is also included in the lockfile by running `cargo check`
and then staging the change before creating the bump commit.

Prerequisites:

- `cargo-edit`

Hooks:

```toml
pre_bump_hooks = [
  "cargo build --release",          # verify the project builds
  "cargo set-version {{version}}",  # bump version in Cargo.toml
  "cargo check --release",
  "git add :/Cargo.lock",           # stage version bump in Cargo.lock
]
post_bump_hooks = [
  "git push",
  "git push {{version}}",
]
```

### Java Maven projects

A recipe for Java Maven projects.
Notably, the version bump is also included in the `pom.xml` project manifest by running `mvn versions:set`
and then staging the change before creating the bump commit.

You can also run `mvn deploy` if this phase is configured in your `pom.xml`.

Hooks:

```toml
pre_bump_hooks = [
  "mvn versions:set -DnewVersion={{version}}",
  "mvn clean package",
]

post_bump_hooks = [
  "mvn deploy", # Optional
  "git push origin {{version}}",
  "git push"
]

```