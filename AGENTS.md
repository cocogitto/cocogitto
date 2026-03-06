# AI Agents Guide for Cocogitto Repository

This document provides guidelines for AI agents working in the Cocogitto repository, covering coding practices, testing, formatting, and contribution workflows.

## Repository Overview

Cocogitto is a CLI toolbox for Conventional Commits and SemVer specifications, providing:
- Conventional commit creation and validation
- Automatic version bumping and changelog generation
- Git repository management with conventional commit support
- Monorepo support

## Coding Practices

### Language and Tooling
- **Primary Language**: Rust
- **Formatter**: `rustfmt` (run with `cargo fmt --all`)
- **Linter**: `clippy` (run with `cargo clippy`)
- **Error Handling**:
  - Avoid `unwrap()`, use `expect("why")` instead
  - Public API errors: use `anyhow!` macro
  - Private errors: define in corresponding `error.rs` files

### Code Structure
- **Source Code**: `src/` directory
- **Tests**: `tests/` directory
- **Binaries**: `src/bin/` directory
- **Conventional Commit Logic**: `src/conventional/` directory
- **Git Operations**: `src/git/` directory

## Testing

### Test Coverage
- **Minimum Coverage**: 80%
- **Coverage Drop Limit**: No more than 1% drop allowed
- **Test Organization**: Follows Rust book conventions

### Test Types

#### Unit Tests
- Test private functions where they are defined
- Use `#[cfg(test)]` modules within source files
- Example structure:
```rust
#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_function() {
        // Test implementation
    }
}
```

#### Integration Tests
- Located in `tests/` directory
- Use `#[sealed_test]` macro for git repository tests
- **lib_tests**: Public API tests
- **cog_tests**: CLI integration tests

### Git Repository Testing
- Use `sealed_test` crate for isolated test environments
- Set up test repositories with `tests/helpers.rs`
- Use `run_cmd!`/`run_fun!` macros for shell commands

### Test Example
```rust
#[sealed_test]
fn test_commit_creation() -> Result<()> {
    // Arrange
    let repo = git_init_no_gpg()?;
    
    // Act
    let oid = repo.commit("feat: test commit")?;
    
    // Assert
    assert!(oid.is_ok());
    Ok(())
}
```

## Formatting and Linting

### Formatting
- Run `cargo fmt --all` before committing
- Follow rustfmt configuration in repository

### Linting
- Run `cargo clippy` before committing
- Fix all clippy warnings
- Consider using `cargo clippy --fix` for automatic fixes

### Git Hooks
- Install with `cog install-hook --all`
- Automatically runs formatters and linters

## Contribution Workflow

### Before Submitting PR
1. **Discuss**: Talk about features/bugs on issues or Discord
2. **Draft PRs**: Welcome for early feedback
3. **Run Checks**:
   - `cargo fmt --all`
   - `cargo clippy`
   - All tests pass
   - Coverage maintained

### Commit Messages
- Follow Conventional Commits specification
- Use `cog commit` for consistent messages
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, etc.
- Include scope when relevant: `feat(cli): message`
- Use `-B` flag for breaking changes

### Pull Request Requirements
- Reference related issues
- Include tests for new functionality
- Update documentation if needed
- Maintain backward compatibility (or document breaking changes)

## Configuration

### cog.toml
- Main configuration file
- Define commit types, bump hooks, changelog templates
- Example sections:
```toml
[commit_types]
hotfix = { changelog_title = "Hotfixes", order = 0 }

[bump_profiles.hotfix]
pre_bump_hooks = [
    "echo 'Running hotfix bump'"
]

[changelog]
path = "CHANGELOG.md"
template = "remote"
```

## Documentation

### Documentation Structure
- **User Guide**: `website/guide/`
- **Reference**: `website/reference/`
- **Configuration**: `website/reference/config.md`
- **Templates**: `website/reference/template.md`

### Documentation Standards
- Use markdown format
- Include code examples
- Link to relevant sections
- Keep up-to-date with code changes

## Version Bumping

### Auto Bump Rules
- `fix`: PATCH version
- `feat`: MINOR version  
- `BREAKING CHANGE`: MAJOR version
- Customizable in `cog.toml`

### Bump Process
1. Calculate next version from commits
2. Run pre-bump hooks
3. Update changelog
4. Create version commit
5. Create git tag
6. Run post-bump hooks

### Bump Hooks
- **Pre-bump**: Run before version commit
- **Post-bump**: Run after version commit
- Use version aliases: `{{version}}`, `{{latest}}`
- Example:
```toml
pre_bump_hooks = [
    "cargo build --release",
    "echo 'Bumping to {{version}}'"
]
```

## Changelog Generation

### Templates
- Uses Tera templating engine
- Built-in templates: `default`, `remote`, `full_hash`
- Custom templates supported

### Changelog Structure
```markdown
## [version] - [date]

### Features
- feat: description (commit_hash) - author

### Bug Fixes
- fix: description (commit_hash) - author
```

## Git Integration

### Commit Validation
- `cog check`: Validate commit history
- `cog check --from-latest-tag`: Check from last tag
- Identifies non-compliant commits

### Commit Editing
- `cog edit`: Rewrite non-compliant commits
- Interactive editing process
- Maintains commit history integrity

### Log Filtering
- `cog log`: Conventional commit log
- Filter by type, author, scope
- Show breaking changes only

## Monorepo Support

### Configuration
- Define packages in `cog.toml`
- Independent versioning per package
- Shared changelog or per-package changelogs

### Bumping Strategy
- Calculate versions per package
- Respect package dependencies
- Custom bump profiles per package

## CI/CD Integration

### GitHub Actions
- Use `cocogitto-action` for automated releases
- Example workflow:
```yaml
- name: Semver release
  uses: cocogitto/cocogitto-action@main
  with:
    release: true
    git-user: 'Cog Bot'
```

### GitHub Bot
- `cocogitto-bot`: PR validation
- Checks conventional commit compliance
- Zero-configuration setup

## Mandatory Practices for AI Agents

### Coding Rules
- **No AI Traces**: Leave no comments or traces indicating AI involvement in the code
- **Test Convention**: Use the `// Arrange`, `// Act`, `// Assert` pattern in all tests (as seen in existing project tests)

### Code Analysis
1. **Read First**: Always read relevant files before modifying
2. **Search Patterns**: Use `grep` to find usage examples
3. **Follow Conventions**: Match existing code style
4. **Minimal Changes**: Only modify what's necessary

### Test-Driven Development (TDD)

**Mandatory TDD Workflow:**

1. **Write Integration Test First**:
   - Create test in appropriate `tests/` directory
   - Write test that describes desired behavior
   - Use minimal production code stubs to make test compile
   - Verify test fails (red phase)

2. **Implement Minimal Production Code**:
   - Write only enough code to make test pass
   - Keep implementation simple and focused
   - Avoid over-engineering

3. **Refactor**:
   - Improve code quality while keeping tests green
   - Add unit tests for edge cases
   - Optimize performance if needed

**TDD Example:**
```rust
// 1. First write failing integration test
#[sealed_test]
fn test_new_feature() -> Result<()> {
    // Arrange
    let repo = git_init_no_gpg()?;
    
    // Act - this should fail initially
    let result = repo.new_feature("test");
    
    // Assert
    assert_that!(result).is_ok();
    Ok(())
}

// 2. Add minimal implementation to make it compile
impl Repository {
    pub fn new_feature(&self, _name: &str) -> Result<()> {
        unimplemented!()
    }
}

// 3. Run test - should fail
// 4. Implement real functionality
impl Repository {
    pub fn new_feature(&self, name: &str) -> Result<()> {
        // Actual implementation
        Ok(())
    }
}
```

### Testing Strategy
1. **Integration Tests First**: Start with high-level behavior tests
2. **Unit Tests Second**: Add detailed tests for edge cases
3. **Run Tests Frequently**: Verify after each small change
4. **Check Coverage**: Ensure coverage maintained

### Quality Checks for Each Code Change
After every code change, run these commands to ensure quality:

```bash
# Run all tests
cargo nextest run

# Run linter
cargo clippy

# Format code
cargo fmt --all
```

All commands should return exit code 0 before committing changes.

### Documentation Updates
1. **Identify Affected Docs**: Find relevant guide sections
2. **Update Examples**: Keep code examples current
3. **Add New Sections**: For new features
4. **Maintain Structure**: Follow existing documentation patterns

### Error Handling
1. **Use Proper Errors**: Define in `error.rs` files
2. **Avoid Panics**: Use `expect()` with descriptive messages
3. **Handle Edge Cases**: Test error conditions
4. **Provide Context**: Helpful error messages

## Common Tasks

### Adding New Feature
1. Create implementation in appropriate module
2. Add unit tests in same file
3. Create integration tests in `tests/`
4. Add CLI command if needed
5. Update documentation
6. Add to changelog configuration

### Fixing Bug
1. Identify root cause
2. Create minimal fix
3. Add regression test
4. Verify fix doesn't break existing functionality
5. Update documentation if needed

### Adding Commit Type
1. Add to `commit_types` in `cog.toml`
2. Define changelog title and order
3. Update documentation
4. Add examples

## Resources

- **Conventional Commits**: https://www.conventionalcommits.org/
- **SemVer**: https://semver.org/
- **Rust Documentation**: https://doc.rust-lang.org/
- **Cocogitto Docs**: https://docs.cocogitto.io/
- **GitHub Repository**: https://github.com/cocogitto/cocogitto

## Getting Help

- **Issues**: https://github.com/cocogitto/cocogitto/issues
- **Discord**: https://discord.gg/WeZRdhtuf6
- **Documentation**: https://docs.cocogitto.io/
