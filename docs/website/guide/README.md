## Repository initialization

To get the most out of cocogitto you need to have a `cog.toml` config at the root of your repository.
You can create this file manually or generate the default one with `cog init`.

### Create a new repository

```bash
mkdir my_repo && cd my_repo
cog init
```

`cog init` works like `git init` except it create a template `cog.toml` config file,
and an initial commit with the following message : `chore: initial commit`.

You can specify the target path of the repository you want to create :

```bash
cog init my_repo
```

### Initialize an existing repo

Running `cog init` on an existing repository will just create a template configuration without creating any commit :

```bash
git init my_repo && cd my_repo
cog init
```

```bash
❯ git status
On branch master
Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
	new file:   cog.toml
```
## Conventional commits

`cog commit` allows you to easily create commits respecting the
[conventional commits specification](https://www.conventionalcommits.org/en/v1.0.0/). It comes with a set of predefined
arguments named after conventional commit types and
[Angular commit convention](https://github.com/angular/angular/blob/22b96b9/CONTRIBUTING.md#-commit-message-guidelines)
: (`feat`, `fix`, `style`, `build`, `refactor`, `ci`, `test`, `perf`, `chore`, `revert`, `docs`).

As described in the specification conventional commits messages are structured as follows :

```conventional_commit
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

All `cog commit` type subcommands follow the same structure :

```sh
cog commit [FLAGS] <type> <message> [scope]
```

Note that the commit scope comes after the commit description.
This allows using positional arguments instead of typing a scope flag every time.

**Example :**

If you want to create the following commit : `feat: add awesome feature` :

```sh
# With cog
cog commit feat "add awesome feature"

# With git
git commit -m "feat: add awesome feature"
```

### Helpful error messages

Using `cog commit` should prevent a wide range of error in your conventional commit message. But if you still made a mistake,
`cog` will display an error explaining what went wrong :

```bash
❯ cog commit feat "add ability to parse arrays" "sco(pe"
Error: A scope value cannot contains inner parenthesis

Caused by:
     --> 1:9
      |
    1 | feat(sco(pe): add ability to parse arrays
      |         ^---
      |
      = expected no_parenthesis
```

### Breaking changes

All `cog commit` arguments are positional except the optional `-B` flag used to create breaking changes commits :

```bash
cog commit fix -B "add fix a nasty bug" cli
```

This would create the following [breaking change](https://www.conventionalcommits.org/en/v1.0.0/#commit-message-with--to-draw-attention-to-breaking-change)
commit : `fix(cli)!: fix a nasty bug`.

`cog commit` use the `!` notation to denote breaking changes commit because it can be easily seen in your git log, however if
you manually create breaking changes commits with [the footer notation](https://www.conventionalcommits.org/en/v1.0.0/#commit-message-with-description-and-breaking-change-footer)
cocogitto tools will still pick them.


### Commit Body and footers

If you need to create more complex commit messages with body and footers use the `--edit` flag.

**Example:**

```bash
cog commit refactor -e -B "drop support for Node 6" runtime 
```

This would open the following commit message template in your `$EDITOR` of choice :

```editor:line-numbers
# Enter the commit message for your changes.
# Lines starting with # will be ignored, and empty body/footer are allowed.
# Once you are done, save the changes and exit the editor.
# Remove all non-comment lines to abort.
#
# WARNING: This will be marked as a breaking change!
refactor(runtime): drop support for Node 6

# Message body


# Message footer
# For example, foo: bar
```

Upon save a commit will be created with the body and footer typed.

::: tip
There are to kind of footer separators in conventional commits : `token: message` and `token #message`.

GitHub automatically link issues prefixed with a hash.

**Example:**
```
    feat(changelog): add full_hash changelog template

    Closes #127
```
:::


### Custom commit types

**Allowing custom commit types:**
By default `cog commit` supports standard conventional commits type `feat`, `fix` plus the angular commit types: `build`, `ci`,
`revert`, `docs`, `test`, `style`, `chore`, `perf`. If you want to use more types you can add them to a file named
`cog.toml` in your repository root directory :

```toml
[commit_types]
hotfix = { changelog_title = "Hotfixes" }
release = { changelog_title = "Releases" }
```

The above config would generate a `cog commit hotfix` and `cog commit release` subcommands following the same structure as the default ones.

**Overriding existing commit types:**

Existing commit type can be overridden just like custom ones: 
```toml
[commit_types]
feat = { changelog_title = "➕ Additional features" }
fix = { changelog_title = "🪲 Releases" }
```

**Omit commits from changelog:**

If you want to make changelog more concise you can skip some commit types with the `omit_from_changelog` option. 

```toml
[commit_types]
chore = { changelog_title = "", omit_from_changelog = true }
ci = { changelog_title = "", omit_from_changelog = true }
perf = { changelog_title = "", omit_from_changelog = true }
```

### Deal with merge commits

By default, git will write the following message to merge commit : `Merge my 'branch'`. These merge commits do not respect 
the Conventional Commits specification, and we strongly advise avoiding them by setting the following in your `.gitconfig` : 

```toml
[merge]
  ff = only
```

That said you can simply make Cocogitto ignore merge commits by setting the following in your `cog.toml`: 
```toml
ignore_merge_commits = true
```

## Check commit history

Running `cog check` will check your commit history against the conventional commit specification :

```bash
❯ cog check
No errored commits
```

Let us create an invalid commit :
```bash
git commit -m "Your Mother Was A Hamster, And Your Father Smelt Of Elderberries"
```

And check our commit history again :
```bash
❯ cog check
Error:
Found 1 non compliant commits in db5151..HEAD:

________________________________________________________

Errored commit: db5151486a41f1b694fd8f90144dd02c87268988 <Paul Delafosse>
	Commit message: 'Your Mother Was A Hamster, And Your Father Smelt Of Elderberries'
	Error: Missing commit type separator `:`
	
	Caused by:
	     --> 1:5
	      |
	    1 | Your Mother Was A Hamster, And Your Father Smelt Of Elderberries
	      |     ^---
	      |
	      = expected scope or type_separator
```


::: tip
You can check your history, starting from the latest tag using `--from-latest-tag` or `-l` flag.  
This is useful when your git repo started to use conventional commits from a certain point in history and you
don't care about editing old commits.
:::

## Managing git-hooks

Cocogitto provide a way to share [git hooks](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks). 

First you need to set up some hooks in `cog.toml`:

```toml
# Embedded git-hooks script
[git_hooks.commit-msg]
script = """#!/bin/sh
set -e
cog verify --file $1
cog check
cargo fmt -v --all --check
cargo clippy
"""

# Or file path
[git_hooks.pre-push]
path = "hooks/pre-push.sh"
```

::: warning
Not that unlike `git commit`, `cog commit` will not pick a default shell when running hooks. Make sure to provide
a shebang in your hook definition.
:::

Now that our hook are defined in cocogitto's config they can be installed with `cog install-hook`. 

**Install all git-hooks:**
```bash
❯ cog install-hook --all
```

**Install a single hook:**
```bash
❯ cog install-hook commit-msg
```

## Sandbox

`cog verify` checks an arbitrary input string against the conventional commit specification.
It will not create any commit.

**Example:**
```bash
❯ cog verify "Your Mother Was A Hamster, And Your Father Smelt Of Elderberries"
Error: Missing commit type separator `:`

Caused by:
     --> 1:5
      |
    1 | Your Mother Was A Hamster, And Your Father Smelt Of Elderberries
      |     ^---
      |
      = expected scope or type_separator
```

## Rewrite non-compliant commits

::: danger
Using `cog edit` will modify your commit history and change the commit SHA of edited commit
and their child.
:::

Once you have spotted invalid commits you can quickly fix your commit history by running `cog edit`.
This will perform an automatic rebase, cycling through each malformed commit, and letting you edit them with your `$EDITOR`
of choice.

**Example :**

```editor:line-numbers
# Editing commit c2bb56b93821ff34282f322be4d2231f53b9ada8
# Replace this message with a conventional commit compliant one
# Save and exit to edit the next errored commit
Your Mother Was A Hamster, And Your Father Smelt Of Elderberries
```

::: tip
You can use the `--from-latest-tag` or `-l` to edit only commits since the latest tag in your repository.
:::

## Conventional commit log

`cog log` is just like `git log` but it displays additional conventional commit information, such as commit scope,
commit type etc.


[![asciicast](https://asciinema.org/a/ssH4yRSlc28Rb9dHEDN7TowGe.svg)](https://asciinema.org/a/ssH4yRSlc28Rb9dHEDN7TowGe)


::: tip
You can filter the log content with the following flags :

- `-B` : display breaking changes only
- `-t` : filter on commit types
- `-a` : filter on commit authors
- `-s` : filter on commit scopes
- `-e` : ignore non compliant commits

Those flag can be combined to achieve complex search in your commit history :

```bash
cog log --author "Paul Delafosse" "Mike Lubinets" --type feat --scope cli --no-error
```
:::

## Changelogs 

`cog changelog` can generate changelog automatically. 

Let's assume the following history : 

```git
* e3ff26a - (HEAD -> master) feat!: implement parser specification <Paul Delafosse>
* 78dedea - feat: a commit <Paul Delafosse>
* c361eea - feat: say hello to the world <Paul Delafosse>
* 6d014b4 - chore: initial commit <Paul Delafosse>
```

Let us now get a changelog : 
```bash
cog changelog
```

```markdown
## 0.2.0 - 2021-11-10
#### Features
- **(hello)** say hello to the galaxy - (da4af95) - Paul Delafosse
#### Refactoring
- **(hello)** say hello to the martians - (22db158) - Paul Delafosse
- - -
## 0.1.0 - 2021-11-10
#### Features
- implement parser specification - (e3ff26a) - Paul Delafosse
- a commit - (78dedea) - Paul Delafosse
- say hello to the world - (c361eea) - Paul Delafosse
```

As you can see above a changelog is generated for each semver compliant tag.

::: tip
You can specify a custom changelog range or tag like so :
```bash
# Display the changelog between `^1` and `2.0.0`
cog changelog --at 2.0.0

# From `8806a5` to `1.0.0`
cog changelog 8806a5..1.0.0

# From `8806a5` to `HEAD`
cog changelog 8806a55..

# From first commit to `1.0.0`
cog changelog 8806a5..1.0.0
```
:::

### Built-in templates

A raw changelog is nice, but its even nicer to generate some links for repository hosted on git web platforms 
such as GitHub. To do this you can use the `--template` or `t` flag. Cocogitto comes with three pre built templates: 

#### `default`

The default template we saw in the previous section

#### `full_hash`

A changelog template tailored for GitHub releases
```bash
cog changelog --template hull_hash
```

```markdown
  #### Features
  - da4af95b223bb8942ffd289d1a62d930c80d7bbd - **(hello)** say hello to the galaxy - @oknozor
  #### Refactoring
  - 22db158f6c75aa5e9e7d4ed4a5b5af7b147453d7 - **(hello)** say hello to the martians - @oknozor
  - - -
  #### Features
  - e3ff26a8247b9690ce241e9843eea595bcac8d06 - implement parser specification - @oknozor
  - 78dedeaf5e7222cd338627f7ee982e271a3f9a4c - a commit - Paul Delafosse
  - c361eeae958a0a28041aecfed10091dc0e6768dd - say hello to the world - @oknozor
```
  
Below is the changelog as rendered by GitHub release, notice how the committer git signature as been replaced 
by their github username. To do that you need to tell cocogitto about your contributor's username in `cog.toml`:

```toml
  [changelog]
  authors = [
      { username = "oknozor", signature = "Paul Delafosse" }
  ]
  ```
<img :src="$withBase('github-release-changelog.png')" alt="Github release changelog screenshot">

#### `remote`

A template generating links for web platform hosted repository.
  
```bash
cog changelog --at 0.1.0 -t remote --remote github.com --owner oknozor --repository  cocogitto
```

As you can see below a changelog is generated with full links to issues, tags, diff and usernames according
to the provided remote, owner and repository flags. 

```markdown
## [0.1.0](https://github.com/oknozor/cocogitto/compare/6d014b40f552fc1ad08f574fe33355175b0783ff..0.1.0) - 2021-11-11
#### Features
- implement parser specification - ([e3ff26a](https://github.com/oknozor/cocogitto/commit/e3ff26a8247b9690ce241e9843eea595bcac8d06)) - [@oknozor](https://github.com/oknozor)
- a commit - ([78dedea](https://github.com/oknozor/cocogitto/commit/78dedeaf5e7222cd338627f7ee982e271a3f9a4c)) - [@oknozor](https://github.com/oknozor)
- say hello to the world - ([c361eea](https://github.com/oknozor/cocogitto/commit/c361eeae958a0a28041aecfed10091dc0e6768dd)) - [@oknozor](https://github.com/oknozor)
```
::: tip 
To avoid typing the remote information and changelog template everytime you can set  some default values in `cog.toml`.

Here is the config used by cocogitto itself. 

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
:::

### Custom templates

If you are not happy with the default you can create your own changelog template. 
Internally cocogitto uses [tera](https://tera.netlify.app/) template engine to render changelogs. 

Also see [template reference](../template).

**Example:**

```tera
{% for type, typed_commits in commits | sort(attribute="type")| group_by(attribute="type")%}                            
#### {{ type | upper_first }}
                                                                                                                        
    {% for scope, scoped_commits in typed_commits | group_by(attribute="scope") %}                                      
        {% for commit in scoped_commits | sort(attribute="scope") %}                                                    
            {% if commit.author %}                                                                                      
                {% set author = "@" ~ commit.author %}                                                                  
            {% else %}                                                                                                  
                {% set author = commit.signature %}                                                                     
            {% endif %}                                                                                                 
            - {{ commit.id }} - **({{ scope }})** {{ commit.summary }} - {{ author }}
        {% endfor %}                                                                                                    
    {% endfor %}                                                                                                        
    {% for commit in typed_commits | unscoped %}                                                                        
        {% if commit.author %}                                                                                          
            {% set author = "@" ~ commit.author %}                                                                      
        {% else %}                                                                                                      
            {% set author = commit.signature %}                                                                         
        {% endif %}                                                                                                     
            - {{ commit.id }} - {{ commit.summary }} - {{ author }}
    {% endfor %}                                                                                                        
{% endfor %}                                                                                                            
```


## Automatic versioning

The purpose of conventional commits is to be able to bump your project version and changelog
automatically. Cocogitto allow you to do this with the `cog bump` command.

The `bump` subcommand will execute the following steps :

1. Calculate the next version based on the commit types since the latest tag.
2. Execute a set configuration defined pre-bump hooks.
3. Append the changes for this version to `CHANGELOG.md`.
4. Create a version commit containing changes made during the previous steps.
5. Create a git tag on the version commit.
6. Execute a set of configuration defined post-bump hook.

### Auto bump

`cog bump` will calculate the next version based on your commit history since the latest
semver tag. Once a tag number as been calculated it will create a tagged commit containing 
the changelog for this new tag. 

**Example:** 

Assuming we are working on the following git repository :
```git
* 8e08b78 - (HEAD -> master) feat: another cool feature <Paul Delafosse>
* 490b846 - docs: add some documentation <Paul Delafosse>
* 8bc0d28 - fix: fix a ugly bug <Paul Delafosse>
* a0c9050 - feat: add awesome feature <Paul Delafosse>
* 6d014b4 - chore: initial commit <Paul Delafosse>
```

Let us now create a version :
```bash
❯ cog bump --auto
Warning: using 'cog bump' with the default configuration.
You may want to create a 'cog.toml' file in your project root to configure bumps.

Failed to get current version, falling back to 0.0.0
Skipping irrelevant commits:
	- docs: 1

Found feature commit 8e08b7
Found bug fix commit 8bc0d2
Found feature commit a0c905
Bumped version: ... -> 0.1.0
```

If we look again at our git log :
```git
* 76c0ffd - (HEAD -> master, tag: 0.1.0) chore(version): 0.1.0 (2 minutes ago) <Paul Delafosse>
... 
```

Also, a `CHANGELOG.md` file have been created using the `default` template: 

```markdown
# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.
- - -

## 0.1.0 - 2021-11-11
#### Bug Fixes
- fix a ugly bug - (8bc0d28) - Paul Delafosse
#### Documentation
- add some documentation - (490b846) - Paul Delafosse
#### Features
- another cool feature - (8e08b78) - Paul Delafosse
- add awesome feature - (a0c9050) - Paul Delafosse

- - -
Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).
```

Also see [template config](/config) if you need to change the default changelog template.

::: tip
Sometimes getting a version number automatically is not what you want.
Cocogitto let you specify the target version with the following flags : 
- `--auto` : choose the next version for you (based on feature commit, bug fixes commit and BREAKING_CHANGE commit).
- `--major` : increment the MAJOR version.
- `--minor` : increment the MINOR version.
- `--patch` : increment the PATCH version.
- `--version <version>` : set version manually ( ex : `cog bump --version 3.2.1`).
- `--pre <metadata>` : set the [pre-release metatada](https://semver.org/#spec-item-9).

**Example:**
```bash
cog bump --major --pre "beta.1"
# 1.2.3 -> 2.0.0-beta.1
```
:::

**Note:** `cog bump --auto` treats `0.y.z` versions specially,
i.e. it will never do an auto bump to the `1.0.0` version, even if there are breaking changes.
That way, you can keep adding features in the development stage and decide yourself, when your API is stable.

#### Dry run

If you just need to get the next version number without performing the automatic bump use the `--dry-run` flag : 

```shell
cog bump --dry-run --auto 
```

:::tip
The `dry-run` flag can be helpful when writing shell scritps using cocogitto. Since only the version number will 
output to stdout so you can do the following: 

```shell
VERSION=$(cog bump --dry-run --auto) # -> VERSION=1.0.0
```
:::

### Bump hooks

#### Pre bump hooks

Creating git tag automatically is great, but sometimes you need to edit some file with the new version number,
or perform some additional checks before doing so.

A typical example is editing your project manifest in your package manager configuration file.
You can run pre bump commands with the `latest` and `version` aliases to reference respectively
the latest known tag and the target version.

**Example:**

When adding the following hooks to `cog.toml`, the hook commands will be run before creating the version commit : 

```toml
# cog.toml
pre_bump_hooks = [
    "cargo build --release",
    "echo 'bumping from {{latest}} to {{version}}'",
    "cargo bump {{version}}",
]
```

And result in the following bump: 

```bash
❯ cog bump --auto
Skipping irrelevant commits:

Found feature commit 9055d9
   Compiling my_awesome_repo v0.1.0 (/home/okno/_Workshop/MyRepos/my_awesome_repo)
    Finished release [optimized] target(s) in 0.86s
bump from 0.1.0 to 0.2.0
Bumped version: 0.1.0 -> 0.2.0
```

::: tip 
If any of the pre-bump command fails cocogitto will abort the bump. 
Any changes made to the repository during the pre-bump phase will be stashed under `cog_bump_{{version}}`.

**Example:**

cog.toml :
```toml
pre_bump_hooks = [
  "cargo build --release",
  "echo 'bump from {{latest}} to {{version}}'",
  "exit 1" # Fail on purpose here
]
```

run :
```bash
❯ cog bump --auto
Skipping irrelevant commits:

Found bug fix commit a0de11
   Compiling my_awesome_repo v0.2.0 (/home/okno/_Workshop/MyRepos/my_awesome_repo)
    Finished release [optimized] target(s) in 0.22s
bump from 0.2.0 to 0.2.1
Error: prehook run `exit 1` failed
	All changes made during hook runs have been stashed on `cog_bump_0.2.1`
	you can run `git stash apply stash@0` to restore these changes.
```
:::


#### Post bump hooks

Post-bump hooks works exactly like pre-bum hooks. They are run after the version has been created 
and are typically used to push changes to the remote, publish packages and apply branching model strategies.

```toml
# cog.toml
post_bump_hooks = [
    "git push",
    "git push origin {{version}}",
    "cargo publish"
]
```  

::: danger
There is no rollback procedure for post-bump hook, on failure cog will abort and leave the repository
in an undefined state. 
We are working on allowing user to define there rollback procedures, but it is not there yet. 
:::

#### Version DSL

It is common to bump your development branch version package manifest after creating a new release.
A typical example in the java world would be to bump your maven snapshot on your development branch after a release.

```toml
# cog.toml
post_bump_hooks = [
    "git push",
    "git push origin {{version}}",
    "git checkout develop",
    "git rebase master",
    "mvn versions:set -DnewVersion={{version+minor-SNAPSHOT}}",
    "cog commit chore \"bump snapshot to {{version+1minor-SNAPSHOT}}\"",
    "git push",
]
```

As you can see we are bumping the manifest using a small DSL. It as only a few keywords :
- start with the one of `version`,`version_tag`, `latest`, `latest_tag` or `package` keyword.
- followed by the `+` operator.
- `major`, `minor` and `patch` to specify the kind of increment you want.
  Then an optional amount, default being one (`version+1minor` and `version+minor` being the same).
- followed by any number of `+{amount}{kind}` (exemple: `version+2major+1patch`)
- ended by any alphanumeric character (SemVer additional labels for pre-release and build metadata), here `-SNAPSHOT`.

#### Bump profiles

For some branching model or release cadence you might want to bump your versions with
different hooks. 

To do so you can define alternate profile hooks in `cog.toml` :

```toml
[bump_profiles.hotfix]
pre_bump_hooks = [
  # Ensure we are performing a bump from the latest release branch
  """
    [[ "$(git rev-parse --abbrev-ref HEAD)" == "release/{{latest}}" ]] && echo "On branch release/{{latest}}" || exit 1
    """,
]

post_bump_hooks = []
```

Once your custom hook profile is set you can call it with the `--hook-profile` flag :

```bash 
❯ cog bump -h hotfix --auto
Skipping irrelevant commits:

Found feature commit 5b21b3
Found bug fix commit a0de11
[[ $(git rev-parse --abbrev-ref HEAD) == release/0.2.0 ]] && echo On branch release/0.2.0 || exit 1
On branch release/0.2.0
Bumped version: 0.2.0 -> 0.3.0
```

Note that for the sake of readability in this documentation, the above example use a oneliner 
to check the current branch but you would probably want to can a shell script instead :  

```toml
pre_bump_hooks = [
  """
    sh -c "./check_branch.sh"
    """,
]
```

### Branch whitelist

It is a common practice to bump always from the same set of branches. For instance, you might want to allow bumping only
on branch `main` and branches prefixed with `release/`. 

To do so add the following to your `cog.toml` : 

```toml
branch_whitelist = [
  "main",
  "release/**"
]
```

## Automatic versioning for monorepo

Managing versions for mono-repository is slightly different from the standard Cocogitto flow. 
Instead of the [standard bump steps](#automatic-versioning) using `cog bump --auto` on a mono-repository will 
perform the following actions: 

1. Calculate next version for each package based on commits that changes the package content.
2. Calculate a global version based on the created package versions and the commit that does not belong to a specific package.  
3. Execute global pre-bump hooks.
4. Execute per package pre-bump hooks.
5. Append the changes for each package to `{package_path}/CHANGELOG.md`.
6. Append global changes and a list of package version to `/CHANGELOG.md`.
7. Create a version commit containing changes made during the previous steps.
8. Create global git tag on the version commit.
9. Create a tag for each new package version on the version commit. 
10. Execute per package post-bump hooks.
11. Execute global post-bump hooks.

### Mono-repository bump

When using `cog bump` in a mono-repository context, it behaves slightly differently. 
 
- `cog bump --auto`: creates a tag per changed packages since their respective latest releases and creates a global 
    mono-repository tag.
- `cog bump` used why manual bump flags such as `--minor`, `--major`, `--patch` or `--version` will only
    create the monorepo version without bumping packages.

- `cog bump --package=my_package --auto` creates a single package tag from the latest package tag

:::tip
We strongly advise to use automatic bump whenever possible. Manual bump should only be used when there are changes that 
Cocogitto is not able to detect (ex: a breaking change occurring in a package via updating a global dependency).
:::


### Packages configuration

To set up mono-repository support you only need to define a list of package in your `cog.toml`
config. Once packages are defined, `cog` will automatically scan your packages during automatic version bump.

**Example:**

A real life example from [oknozor/gill](https://github.com/oknozor/gill/blob/main/cog.toml). 

```toml
[packages]
gill-app = { path = "crates/gill-app" }
gill-authorize-derive = { path = "crates/gill-authorize-derive", public_api = false }
gill-db = { path = "crates/gill-db", public_api = false }
gill-git = { path = "crates/gill-git", public_api = false }
gill-git-server = { path = "crates/gill-git-server" }
gill-markdown = { path = "crates/gill-markdown", public_api = false }
gill-settings = { path = "crates/gill-settings" }
gill-syntax = { path = "crates/gill-syntax" }
gill-web-markdown = { path = "crates/gill-web-markdown" }
syntect-plugin = { path = "crates/syntect-plugin", public_api = false }
```

:::tip
If some of your packages does not belong to your project public API use `public_api = false` to prevent `--auto` bump 
from updating the global project version. 
:::

#### Packages hooks

When creating a monorepo version Cocogitto will execute the pre-bump and post-bump hooks normally. Additionally, it will
run `pre_package_bump_hooks` and `post_package_bump_hooks` before and after each package bump.
To override these you can define per package hooks. 

**Example:**
```shell
## Pre hooks executed before each package bump, here we use a cargo command to bump rust package manifest
pre_package_bump_hooks = [
    "echo 'upgrading {{package}}' to {{version}}",
    "cargo set-version {{version}}"
]

[packages]
rust-package-one = { path = "packages/rust-one" }
rust-package-two = { path = "packages/rust-two" }
## We have a java project in the mono-repository so we override the default pre-hook
java-package = { path = "packages/java-package", pre_bump_hooks = [ "mvn build" ] }
```

:::tip
Note that for package hooks, you can use the `package` variable from version DSL to get the current package name.
:::

### Bump hook recipes

#### Cargo library projects

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

#### Cargo executable projects

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

#### Java Maven projects

A recipe for Java Maven projects.
Notably, the version bump is also included in the `pom.xml` project manifest  by running `mvn versions:set`
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

## Tag prefix

It is common to use a tag prefix when creating version in your repository. This is described in the [SemVer specification
FAQ](https://semver.org/#is-v123-a-semantic-version). This convention provide a handy way to distinguish between release 
versions and tags that does not represent releases. 

To tell `cog` to pick only version starting with a prefix set this in your `cog.toml` file : 

```toml
tag_prefix = "v"
```

## Make Cocogitto skip CI CD

Cocogitto will create a commit when performing a bump, which can trigger your CI/CD if you have one. Some CI/CD tools support a "skip_ci" string that you can add to a commit which will skip the pipeline execution. To do so with `cog`, you can use the `skip_ci` configuration in your `cog.toml` file or the `cog bump --skip-ci <string>` option to add a "skip_ci" pattern your CI/CD tool supports.

**Example:**
```toml
skip_ci = "[skip ci]"
```

or using the `cog bump` command :

```bash
❯ cog bump --skip-ci "[skip ci]"
```

**Result:**

```bash
❯ git log
commit 213d08c8c1e12ba7d59497e6eda436a3ce63d87c (HEAD -> main, tag: 1.0.0)
Author: John Doe <jon.doe@unknown.com>
Date: Tue Mar 7 15:06:18 2023 +0200
    chore(version): 1.0.0 [skip ci]
```

Note that if both `skip_ci` configuration and `--skip-ci` option are used, `cog` will take the `--skip-ci` option.

## Skip untracked or uncommited changes

By default, Cocogitto will abort bump if there are uncommited or untracked changes. You can change this behavior using the `skip_untracked` configuration in the `cog.toml` file or the `--skip-untracked` option of the `bump` command. If so, the warning will be printed to `stderr` and the bump will continue.

## Get the current version

It's sometime needed to display the current version for scripting purpose. 
You can print the latest SemVer tag on your repo with the `get-version` subcommand: 

```bash
❯ cog get-version
Current version:
5.3.1
```

To silence the additional info and get only the version use the `-v` flag: 
```bash
❯ cog -v get-version
5.3.1
```

If working on a monorepo you can also specify the target package: 
```bash
❯ cog -v get-version --package gill-db
0.1.0
```

Finally, if you need the command to print a version no matter the state of your repository, you can provide a fallback: 
```bash
❯ cog get-version --fallback 0.1.0
0.1.0
```


