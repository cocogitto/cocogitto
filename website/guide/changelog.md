---
editLink: true
---

# Changelogs

`cog changelog` can generate changelog automatically.

Let's assume the following history:

```git
* e3ff26a - (HEAD -> master) feat!: implement parser specification <Paul Delafosse>
* 78dedea - feat: a commit <Paul Delafosse>
* c361eea - feat: say hello to the world <Paul Delafosse>
* 6d014b4 - chore: initial commit <Paul Delafosse>
```

Let us now get a changelog:

```bash
cog changelog
```

```markdown
## 0.2.0 - 2021-11-10

#### Features

- **(hello)** say hello to the galaxy - (da4af95) - Paul Delafosse

#### Refactoring

- **(hello)** say hello to the martians - (22db158) - Paul Delafosse

---

## 0.1.0 - 2021-11-10

#### Features

- implement parser specification - (e3ff26a) - Paul Delafosse
- a commit - (78dedea) - Paul Delafosse
- say hello to the world - (c361eea) - Paul Delafosse
```

As you can see above a changelog is generated for each semver compliant tag.

::: tip
You can specify a custom changelog range or tag like so:

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

## Built-in templates

A raw changelog is nice, but its even nicer to generate some links for repository hosted on git web platforms
such as GitHub. To do this you can use the `--template` or `t` flag. Cocogitto comes with three pre built templates:

### `default`

The default template we saw in the previous section

### `full_hash`

A changelog template tailored for GitHub releases

```bash
cog changelog --template full_hash
```

```markdown
#### Features

- da4af95b223bb8942ffd289d1a62d930c80d7bbd - **(hello)** say hello to the galaxy - @oknozor

#### Refactoring

- 22db158f6c75aa5e9e7d4ed4a5b5af7b147453d7 - **(hello)** say hello to the martians - @oknozor

---

#### Features

- e3ff26a8247b9690ce241e9843eea595bcac8d06 - implement parser specification - @oknozor
- 78dedeaf5e7222cd338627f7ee982e271a3f9a4c - a commit - Paul Delafosse
- c361eeae958a0a28041aecfed10091dc0e6768dd - say hello to the world - @oknozor
```

Below is the changelog as rendered by GitHub release, notice how the committer git signature as been replaced
by their GitHub username. To do that you need to tell cocogitto about your contributor's username in `cog.toml`:

```toml
  [changelog]
  authors = [
      { username = "oknozor", signature = "Paul Delafosse" }
  ]
```

![Github release changelog screenshot](/github-release-changelog.png)

### `remote`

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
To avoid typing the remote information and changelog template everytime you can set some default values in `cog.toml`.

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

## Custom templates

If you are not happy with the default you can create your own changelog template.
Internally cocogitto uses [tera](https://tera.netlify.app/) template engine to render changelogs.

Also see [template reference](/reference/template).

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

