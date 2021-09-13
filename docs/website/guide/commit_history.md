# Managing commit history

`cog` as several subcommands to examine and manipulate your commit history.

## Validate repository history compliance with the specification

Running `cog check` will check your commit history against the conventional commit specification :

```
❯ cog check
No errored commits
```

Let us create an invalid commit :
```shell script
git commit -m "Your Mother Was A Hamster, And Your Father Smelt Of Elderberries"
```

And check our commit history again :
```
❯ cog check
Error: ERROR - Your Mother Was A Hamster, And Your Father Smelt Of Elderberries - (c2bb56)
	cause: Missing commit type separator `:
```

Additionally, you can check your history, starting from the latest tag to HEAD using `from-latest-tag` flag.  
This is useful when your git repo started to use conventional commits from a certain point in history, and you
don't care about editing old commits.

## Rewrite non-compliant commits

Once you have spotted invalid commits you can quickly fix your commit history by running `cog edit`.
This will perform an automatic rebase and let you edit each malformed commit message with your `$EDITOR`
of choice.

**Example :**

`cog edit` will cycle to each malformed commit to rewrite their message : 

```
# Editing commit c2bb56b93821ff34282f322be4d2231f53b9ada8
# Replace this message with a conventional commit compliant one
# Save and exit to edit the next errored commit
Your Mother Was A Hamster, And Your Father Smelt Of Elderberries
```

⚠️ Beware that using `cog edit` will modify your commit history and change the commit SHA of edited commit
and their child.

## Conventional commits git log


`cog log` is like `git log` but it displays additional conventional commit information, such as commit scope,
commit type etc.


[![asciicast](https://asciinema.org/a/ssH4yRSlc28Rb9dHEDN7TowGe.svg)](https://asciinema.org/a/ssH4yRSlc28Rb9dHEDN7TowGe)

You can also filter the log content with the following flags (`cog log --help`) :

- `-B` : display breaking changes only
- `-t` : filter on commit type
- `-a` : filter on commit author
- `-s` : filter on commit scope
- `-e` : ignore errors

Those flag can be combined to achieve complex search in your commit history :

```shell script
cog log --author "Paul Delafosse" "Mike Lubinets" --type feat --scope cli --no-error
```

### Changelog summary

There are two ways to generate changelog with `cog` :

- To your repo `CHANGELOG.md` file with `cog bump` (see: [auto bump](versioning.md#auto-bump))
- To stdout with `cog changelog`.

    ```markdown
    ❯ cog changelog
    
    ## 0.30.0..HEAD - 2020-09-30
    
    
    ### Bug Fixes
    
    7f04a9 - fix ci cross build command bin args - Paul Delafosse
    
    ### Features
    
    fc7420 - move check edit to dedicated subcommand and fix rebase - Paul Delafosse
    1028d0 - remove config commit on init existing repo - Paul Delafosse
    
    ### Refactoring
    
    d4aa61 - change config name to cog.toml - Paul Delafosse
    ```
  
    You can specify a custom changelog range or tag like so : 
    ```shell
      # Display the changelog between `^1` and `2.0.0`
      cog changelog --at 2.0.0
  
      # Display the changelog between `8806a5` and `HEAD`
      # Note that shortened git oid are not supported yet for this command
      cog changelog --from 8806a55727b6c1767cca5d494599623fbb5dd1dd
  
      # Display the changelog between `8806a5` and `1.0.0`
      cog changelog --from 8806a55727b6c1767cca5d494599623fbb5dd1dd --to 1.0.0é
    ```
