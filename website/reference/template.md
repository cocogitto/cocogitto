# Changelog template reference

For a detailed guide on how to write a template changelog refer to [tera's documentation](https://keats.github.io/tera/docs/).

You can also take a look at the [built-in templates](https://github.com/cocogitto/cocogitto/tree/main/src/conventional/changelog/template) in cocogitto repository.

## Context

### Release

- `commits` :
    * **Type:** [`Array<Commit>`](/reference/template.html#commit)
    * **Description:** commits contained in the release
    * **Nullable:** `false`

- `version` :
    * **Type:** [`GitRef`](/reference/template.html#gitref)
    * **Description:** tag name or/and git oid of the current release tip
    * **Nullable:** `false`

- `from`
    * **Type:** [`GitRef`](/reference/template.html#gitref)
    * **Description:** tag name or/and git oid of the commit preceding the release
    * **Nullable:** `false`

- `date`
    * **Type:** `Date`
    * **Description:** date of the release
    * **Nullable:** `false`

### Commit

- `id`:
    * **Type:** `String`, `SHA-1`
    * **Description:** commit `SHA-1`
    * **Nullable:** `false`

- `author`:
    * **Type:** `String`
    * **Description:** the name of the [commit author](/reference/template.html#authors) on the remote platform
    * **Nullable:** `true`

- `signature`:
    * **Type:** `String`
    * **Description:** the git signature of the [commit author](/reference/template.html#authors)
    * **Nullable:** `false`

- `type`:
    * **Type:** `String`
    * **Description:** the conventional commit type of the commit
    * **Nullable:** `false`

- `date`:
    * **Type:** `Date`
    * **Description:** the date of the commit
    * **Nullable:** `false`

- `scope`:
    * **Type:** `String`
    * **Description:** the scope of the commit
    * **Nullable:** `true`

- `summary`:
    * **Type:** `String`
    * **Description:** the conventional commit message summary
    * **Nullable:** `false`

- `body`:
    * **Type:** `String`
    * **Description:** the conventional commit message body
    * **Nullable:** `true`

- `breaking_change`:
    * **Type:** `boolean`
    * **Description:** is the commit marked as a breaking change
    * **Nullable:** `false`

- `footer`:
    * **Type:** [`Array<Footer>`](/reference/template.html#footer)
    * **Description:** the conventional commit footers
    * **Nullable:** `false`

### GitRef

- `tag`:
    * **Type:** `String`
    * **Description:** a SemVer tag name, with an optional [`tag_prefix`](/reference/config.html#tag_prefix). `null` if the version is
      pointing to unreleased changes.
    * **Nullable:** `true`

- `id`:
    * **Type:** `Sting`, `SHA-1`
    * **Description:** the id of the latest commit in the release. This can be `null` only when using `cog bump`, because it
      generates a changelog before creating the target version.
    * **Nullable:** `true`

### Footer

- `token`:
    * **Type:** `String`
    * **Description:** the footer token
    * **Nullable:** `false`
- `content`:
    * **Type:** `String`
    * **Description:** the footer content
    * **Nullable:** `false`

### Remote

- `platform`:
    * **Type:** `String`
    * **Description:** url to the configured git platform in the form `https://{remote}` (
      see: [Config -> Changelog -> Remote](/reference/config.html#remote))
    * **Nullable:** `true`
- `owner`:
    * **Type:** `String`
    * **Description:** name of the repository owner (see: [Config -> Changelog -> Owner](/reference/config.html#owner))
    * **Nullable:** `true`
- `repository_url`: `false`
    * **Type:** `String`
    * **Description:** url to the repository in the form `https://{remote}/{owner}/{repository}`(
      see: [Config -> Changelog -> Repository](/reference/config.html#repository))
    * **Nullable:** `true`

## Filters

In addition to the [tera built-in filters](https://keats.github.io/tera/docs/#built-in-filters), you can use the following:

- `unscoped`
  * **Description:** filter unscoped commits from releases commits. Example:
  * **Example:**
  ```tera
      {% for commit in commits | unscoped %}                                                                       
          {% if commit.author %}                                                                                         
              {% set author = "@" ~ commit.author %}                                                                     
          {% else %}                                                                                                     
              {% set author = commit.signature %}                                                                        
          {% endif %}                                                                                                    
              - {{ commit.id }} - {{ commit.summary }} - {{ author }}
      {% endfor %}    
  ```
- `upper_first`
  * **Description:** capitalize the first letter of a string
  * **Example:**
  ```tera
     {% for type, typed_commits in commits | sort(attribute="type")| group_by(attribute="type") %}                           
      #### {{ type | upper_first }}
     {% endfor %}
  ```
