# repos

A simple command line tool to manage local repositories.

## Notes

* Metadata file is managed by user self manually

The metadata of local repository should be managed by user self manually.
The `repos` command line tool only reads the metadata for operations and never modifies it.

Please take a look at https://github.com/toml-lang/toml to get a quick introduction of toml format.

* Vcs support is done by spawning vcs process

Currently, the `repos` command line tool calls vcs process for synchronizing repositories.

Please make sure to put vcs command line programs in the path environment before using `repos` command line to synchronizing repositories.

## Building

First install Rust https://www.rust-lang.org/install.html .

Then,

```sh
cargo build --release
```

Copy the built
`target/release/repos` (`target\release\repos.exe` in Windows)
command line executable program to any place for use.

## Tutorial

### Prepare your metadata file

See `Repos.sample.toml` for sample.

### Add a new repository

1. Put new repository metadata to `Repos.toml`
2. Run `repos sync {repo_url}`

### Update an existed repository

Run `repos sync {repo_url}`.

### Remove an existed repository

1. Run `repos remove {repo_url}`
2. Delete repository metadata from `Repos.toml`

### Update all repositories

Run `repos sync`.

### List repositories of a topic

Run `repos topic {topic}`.

### List all topics with count of their repositories

Run `repos topics`.

### Output stats of all repositories

Run `repos stats`.

###  Clean up unused resources

Run `repos cleanup`.

### Search repositories

Run `repos search {keyword}`.

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

* `sync`: update or clone if a repository url provided, else synchronize all repositories
* `remove`: remove local directory of a repository
* `topics`: list all topics with count of their repositories
* `topic`: list repositories of a topic
* `stats`: output stats of all repositories
* `cleanup`: clean up unused resources
* `search`: search repositories by keyword
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
