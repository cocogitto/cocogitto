# Packaging cocogitto

## Generating manpages

`cog` has a hidden subcommand to create manpages in a specified directory:

```console
cog generate-manpages "${PWD}/gen"
```

This creates the directory if it doesn't exist already (like `mkdir -p` on Linux),
then outputs generated manpages for `cog` as well as its subcommands into files in that directory.
