## Conventional commits

`coco` allows you to easily create commits respecting the
[conventional commits specification](https://www.conventionalcommits.org/en/v1.0.0/). It comes with a set of predefined
arguments named after conventional commit types and
[Angular commit convention](https://github.com/angular/angular/blob/22b96b9/CONTRIBUTING.md#-commit-message-guidelines)
:  `feat`, `fix`, `style`, `build`, `refactor`, `ci`, `test`, `perf`, `chore`, `revert`, `docs`.

As described in the specification conventional commits messages are structured as follows :

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

All `coco` commit type subcommands follows the same structure :

```
coco {type} {message} [optional scope] [optional body] [optional footer]
```

 You need to remember that `coco` commit scope comes after the commit description. 
 This allows using positional arguments instead of typing flags (ex: `coco -t {type} -s {scope} -m {message}... and so on`).

**Example :**
If you want to create the following commit : `feat: add awesome feature` :

```shell script
coco feat "add awesome feature"
```

### Helpful error messages

Using `coco` should prevent a wide range of error in your conventional commit message. But if you still made a mistake
`coco` will display an error explaining what went wrong :

```
❯ coco feat "add ability to parse arrays" "sco(pe"
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

All `coco` argument are positional except the optional `-B` flag used to create breaking changes commits :

```shell script
coco fix -B "add fix a nasty bug" cli
```

This would create the following [breaking change](https://www.conventionalcommits.org/en/v1.0.0/#commit-message-with--to-draw-attention-to-breaking-change)
commit : `fix(cli)!: fix a nasty bug`.

`coco` use the `!` notation to denote breaking changes commit because it can be easily seen in your git log, however if
you manually create breaking changes commits with [the footer notation](https://www.conventionalcommits.org/en/v1.0.0/#commit-message-with-description-and-breaking-change-footer)
cocogitto tools will still pick them.

