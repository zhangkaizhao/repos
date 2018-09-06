# repos

A simple command tool to manage local repositories.

## Note

The metadata of local repository should be managed by user self manually.
The `repos` command tool only reads the metadata for operations and never modifies it.

Please take a look at https://github.com/toml-lang/toml to get a quick introduction of toml format.

## Tutorial

### Prepare your metadata file

See `Repos.sample.toml` for sample.

### Add a new repoistory

1. Put new repository metadata to `Repos.toml`
2. Run `repos sync {repo_url}`

### Update an existed repository

Run `repos sync {repo_url}`.

### Remove an existed repository

1. Run `repos remove {repo_url}`
2. Delete repository metadata from `Repos.toml`

### Update all repositories in metadata

Run `repos sync`.

### List repositories of a topic

Run `repos topic {topic}`.

### List all topics with count of their repositories

Run `repos topics`.

### Output stats of all repositories

Run `repos stats`.

###  Clean up broken local repositories

Run `repos cleanup`.

### Search local repositories

Run `repos search {key word}`.

### Output proxy configuration

Run `repos proxy`.

### Change proxy configuration

Edit `proxy` section in metadata file.

## Metadata

See `Repos.template.toml` for overview.

### repository

- `url`: follow usage of vcs
- `vcs`: version control system. choices: git, hg
- `allow_sync`: whether synchronized to local, or just marked in metadata
- `bare`: whether bare
- `use_proxy`: whether using proxy for sync
- `topics`: topics belong to

### proxy

- `scheme`: choices: http, socks5
- `host`
- `port`

## Local files and directories

These files and directories are all in current/working directory (`pwd`).

### Metadata file

`Repos.toml`

### repository directory

`{host}/{path_to_repo}`

e.g. repository directory of url `https://example.com/org/repo.git` is `example.com/org/repo`.

## Sub-commands

* `sync`: update or clone if a repository url provided, else synchronize all repositories in metadata
* `remove`: remove directory of a local repository
* `topics`: list all topics with count of their repositories
* `topic`: list repositories of a topic in metadata
* `stats`: output stats of all repositories in metadata
* `cleanup`: clean up broken repositories
* `search`: search local repositories by key word
* `proxy`: output proxy configuration

Examples:

```
repos sync https://github.com/org/repo.git
repos sync
repos remove https://github.com/org/repo.git
repos topics
repos topic rust
repos stats
repos cleanup
repos search key_word
repos proxy
```

## Limitations

There are some limitations now.

* Relative URLs without base (scp-like syntax) are not supported.

  e.g. `[user@]host.xz:path/to/repo.git` or `[user@]host.xz:~/path/to/repo.git`

* The `bare` attribute of a repository is only used for cloning the repository.
* Proxy configuration is global. This may be changed in the future.
* Vcs only supports git and hg(Mercurial) now.
