# Packaging cocogitto

## Generating manpages

`cog` has a hidden subcommand to print generated manpage to STDOUT. To generate the main manpage, run:

```
cog generate-manpage cog > cog.1
```

You can also generate manpages for subcommands by specifying the subcommand name as argument, e.g.:

```
cog generate-manpage commit > cog-commit.1
cog generate-manpage install-hook > cog-install-hook.1
```
