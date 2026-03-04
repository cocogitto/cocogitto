# Migration Guide from 6.5.0 to 7.0.0

This guide covers the breaking changes introduced in Cocogitto 7.0.0 and how to migrate your configuration.

## Main Breaking Change: Monorepo Configuration Structure

### Change: Monorepo Packages Configuration Moved

**What changed**: The monorepo packages configuration has been moved from `[packages]` to `[monorepo.packages]` in `cog.toml`.

**Impact**: If you're using monorepo features, your configuration will need to be updated.

### Migration Steps

#### Before (6.5.0):
```toml
[packages]
my-package = { path = "crates/my-package", changelog_path = "crates/my-package/CHANGELOG.md" }
another-package = { path = "crates/another-package" }
```

#### After (7.0.0):
```toml
[monorepo.packages]
my-package = { path = "crates/my-package", changelog_path = "crates/my-package/CHANGELOG.md" }
another-package = { path = "crates/another-package" }
```

## Other Notable Changes

### 1. Package Resolver Implementation

The monorepo system now uses a new dependency resolver for better workspace dependency resolution. This change is mostly internal, but you may notice:
- More accurate dependency resolution
- Better handling of workspace dependencies
- Improved performance for large monorepos

### 2. Worktree Support

Fixed worktree support by using `git_dir` instead of `repo_dir`. This improves compatibility with Git worktrees.

### 3. Changelog Improvements

- Added support for GitHub-specific trailers (like `Co-authored-by`) in changelogs
- Fixed version date formatting in changelogs
- Simplified changelog template rendering

### 4. Verify Command Enhancements

- Added stdin support via `--file -` for the verify command
- Added scope validation to the verify command

### 5. Pre-release Improvements

New features for better pre-release management:
- `--auto-pre` flag for automatic pre-release incrementing
- `--pre-pattern` flag to specify pre-release patterns
- `pre_pattern` can now be specified in `cog.toml`

## Configuration Examples

### Basic Monorepo Configuration

**Before 6.5.0:**
```toml
[packages]
core = { path = "crates/core" }
cli = { path = "crates/cli" }
```

**After 7.0.0:**
```toml
[monorepo.packages]
core = { path = "crates/core" }
cli = { path = "crates/cli" }
```

### Complete Monorepo Configuration

**Before 6.5.0:**
```toml
[packages]
core = { 
    path = "crates/core", 
    changelog_path = "crates/core/CHANGELOG.md",
    version = "1.0.0"
}
cli = { 
    path = "crates/cli",
    changelog_path = "crates/cli/CHANGELOG.md",
    version = "2.0.0"
}

[bump_profiles.default]
hooks = ["cargo test"]
```

**After 7.0.0:**
```toml
[monorepo.packages]
core = { 
    path = "crates/core", 
    changelog_path = "crates/core/CHANGELOG.md",
    version = "1.0.0"
}
cli = { 
    path = "crates/cli",
    changelog_path = "crates/cli/CHANGELOG.md",
    version = "2.0.0"
}

[bump_profiles.default]
pre_bump_hooks = ["cargo test"]
```

## Testing Your Migration

After updating your configuration:

1. **Test package detection**:
   ```bash
   cog bump --dry-run
   ```

2. **Verify changelog generation**:
   ```bash
   cog changelog
   ```

3. **Check monorepo commands**:
   ```bash
   cog bump --package my-package --dry-run
   ```

## Troubleshooting

If you encounter issues:

1. **"No packages found"**: Ensure you've moved your packages under `[monorepo.packages]`
2. **Configuration errors**: Check your TOML syntax, especially the nested structure
3. **Command failures**: Verify all package paths are correct and accessible

## Rollback Plan

If you need to rollback to 6.5.0:

```bash
# Using cargo
cargo install cocogitto --version 6.5.0

# Don't forget to revert your cog.toml changes
```

## Summary

The main change in 7.0.0 is the restructuring of monorepo configuration. Most other changes are additions or internal improvements that shouldn't require configuration changes.

**Key action**: Move your `[packages]` configuration to `[monorepo.packages]` in your `cog.toml` file.
