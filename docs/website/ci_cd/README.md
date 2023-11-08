## GitHub action

[cocogitto-action](https://github.com/cocogitto/cocogitto-action) can be used in your GitHub action CI/CD to perform release,
check commits against the conventional commits specification and create changelogs. 

::: warning
The action runs on x86 linux runner only. 
:::

### Conventional commits check

By default, if no additional argument is provided cocogitto-action will just check that all your commits are conventional
commits compliant : 
```yaml
on: [push]

jobs:
  cog_check_job:
    runs-on: ubuntu-latest
    name: check conventional commit compliance
    steps:
      - uses: actions/checkout@main
        with:
          fetch-depth: 0

      - name: Conventional commits check
        uses: oknozor/cocogitto-action@v3
```

::: warning
You need to use the checkout action with 
[`fetch-depth: 0`](https://github.com/actions/checkout#fetch-all-history-for-all-tags-and-branches) option to get the 
full git history before running cocogitto-action. 
:::

If you want the action to check only commits since the latest SemVer tag you can do the following: 
```yaml
  - name: Conventional commits check
    uses: oknozor/cocogitto-action@v3
    with:
      check-latest-tag-only: true
```

### Conventional commits release

To create a release with cocogitto-action simply add the `release` option : 

```yaml
  - name: Semver release
    uses: oknozor/cocogitto-action@v3
    with:
      release: true
      git-user: 'Cog Bot'
      git-user-email: 'mycoolproject@org.org'
```

This will run `cog bump --auto` during the step execution. 

If you need to use the created version number later in your job you can access it like so : 
```yaml
  - name: Print version
    run: "echo '${{ steps.release.outputs.version }}'"
```
::: tip
You can set the `git-user` and `git-user-email` options to override the default git signature 
for the release commit.

You might also want to use a dedicated GitHub account to perform the release, this can be done using the checkout action:
```yaml
      - uses: actions/checkout@v3
        with:
          fetch-depth: 0
          ssh-key: ${{ secrets.SERVICE_ACCOUNT_SSH_KEY }}
          
      # Perform release ... 
```
:::

Also see:
* [User guide -> Cog check](../guide/#check-commit-history)

### GitHub release changelog

Depending on the options provided the action will run check and/or create a release but if you need to perform some custom steps
you can directly use `cog`. 

```yaml
on:
  workflow_dispatch:
    branches: main
    
jobs:
  release:
    name: Perform release
        - uses: actions/checkout@v3
            with:
              fetch-depth: 0

        - name: Cocogitto release
          id: release
          uses: oknozor/cocogitto-action@v3
          with:
            release: true
              git-user: 'Cog Bot'
              git-user-email: 'mycoolproject@org.org'

        - name: Generate Changelog
          run: cog changelog --at ${{ steps.release.outputs.version }} -t full_hash > GITHUB_CHANGELOG.md

        - name: Upload github release
          uses: softprops/action-gh-release@v1
          with:
            body_path: GITHUB_CHANGELOG.md
            tag_name: ${{ steps.release.outputs.version }}
```

Also see:
* [User guide -> Automatic Versioning](../guide/#auto-bump)


Note that you can disable the `check` option if needed.

###  Action reference

Here are all the inputs available through `with`:

| Option                  | Description                                                                | Default    |
| -------------------     | -------------------------------------------------------------------------- | -------    |
| `check`                 | Check conventional commit compliance with `cog check`                      |   `true`   |
| `check-latest-tag-only` | Check conventional commit compliance with `cog check --from-latest-tag`    |   `false`  |
| `release`               | Perform a release using `cog bump --auto`                                  |   `false`  |
| `git-user`              | Set the git `user.name` to use for the release commit                      |   `cog-bot`|
| `git-user-email`        | Set the git `user.email` to use for the release commit                     |  `cog@demo.org`|

## GitHub Bot

Cocogitto also has a GitHub bot that decorate pull request with status check. 

To install it just go to [github.com/apps/cocogitto-bot](https://github.com/apps/cocogitto-bot) and click "Configure".
Add the desired repository and grant the required permission.

Once it is done cocogitto-bot will comment on every pull-request events : 

**Example:**

Success: 

![cocogitto bot success example](./cog-bot-ok.png)

Failure:

![cocogitto bot failure example](./cog-bot-ko.png)

::: tip 
You can [make status check mandatory](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/troubleshooting-required-status-checks) 
to enforce conventional commits in your  pull-requests.  
:::
