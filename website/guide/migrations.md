# Migration Guides

This page contains migration guides for major version updates of Cocogitto.

## Available Migration Guides

### [Migration from 6.5.0 to 7.0.0](/guide/migration_6_5_to_7_0)

The main breaking change in 7.0.0 is the restructuring of monorepo configuration:
- Monorepo packages configuration moved from `[packages]` to `[monorepo.packages]`
- Improved dependency resolver for workspace dependencies
- Better performance for large monorepos

### Future Migrations

As new major versions are released, migration guides will be added here to help you upgrade smoothly.

## General Migration Tips

1. **Backup your configuration**: Always backup your `cog.toml` before migrating
2. **Test locally**: Try the new version in a local environment first
3. **Check CI/CD**: Verify your continuous integration pipelines work with the new version
4. **Review changelog**: Read the full changelog for the version you're migrating to
5. **Community support**: Join the Discord server if you need help with migration

## Rollback Strategy

If you encounter issues during migration:

1. **Keep old version**: Don't uninstall the old version until migration is complete
2. **Revert configuration**: Keep backups of your configuration files
3. **Test thoroughly**: Verify all workflows work before deploying to production

```bash
# Install specific version if needed
cargo install cocogitto --version X.Y.Z
```

## Need Help?

If you encounter issues during migration:
- Check the [GitHub issues](https://github.com/cocogitto/cocogitto/issues) for known problems
- Join our [Discord server](https://discord.gg/WeZRdhtuf6) for community support
- Create a new issue if you find a bug or need clarification
