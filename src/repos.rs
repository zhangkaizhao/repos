/// Repos subcommands:

/// - sync
/// - remove
/// - topics
/// - stats
/// - cleanup
/// - search
/// - proxy

use std::path::Path;

use metadata;
use util;
use vcs;

#[derive(Clone)]
pub struct Repos {
    root_dir: &'static str,
    metadata: metadata::Metadata,
}

impl Repos {
    /// Create a new `Repos`.
    pub fn new() -> Self {
        let root_path = Path::new(".");
        let root_dir = root_path.to_str().unwrap();

        // let md_file = root_dir.to_owned() + "/repos.toml";
        let md_file = "repos.toml";
        let md_path = Path::new(&md_file);
        let md = metadata::load(&md_path);

        Repos { root_dir: root_dir, metadata: md }
    }

    /// Update an existed repo or clone a new repo.
    pub fn sync(&self, url: &str) {
        // 1. Check whether repo exists in metadata.
        // 1.1 Warn repo does not exist in metadata, should be manually added first.
        // 2. Clone if repo directory does not exist.
        // 3. Update if repo directory exists.
        let repositories = self.metadata.repos.clone();
        if repositories.contains_key(url) {
            let repo = repositories.get(url).unwrap();
            let local_relpath = util::repo_url_to_relpath(url);
            let relpath = Path::new(&local_relpath);
            let _vcs = &repo.vcs;
            let bare = repo.bare;
            let use_proxy = repo.use_proxy;
            if relpath.is_dir() {
                // Update
                vcs::update(_vcs, &relpath, use_proxy);
            } else {
                // Clone
                vcs::clone(_vcs, &url, &relpath, bare, use_proxy);
            }
        } else {
            // warn
            panic!("Repo has not been put in metadata yet.");
        }
    }

    pub fn sync_all(&self) {
        // 1. Read each repo from metadata.
        // 2. Update each repo.
    }

    pub fn remove(&self, url: &str) {
        // 1. Check whether repo exists in metadata.
        // 1.1 Warn repo exists in metadata, should be manually deleted first.
        // 2. Delete repo directory.
        // 3. Notify user to manually delete repo from metadata file.
        let repositories = self.metadata.repos.clone();
        if repositories.contains_key(url) {
            let local_relpath = util::repo_url_to_relpath(url);
            let relpath = Path::new(&local_relpath);
            util::delete_repo_relpath(relpath);
        } else {
            // warn
            panic!("Repo has not been put in metadata yet.");
        }
    }

    pub fn topics(&self) {
        // repos count by each topics
    }

    pub fn topic(&self, _topic: &str) {
        // list all repos in topic
    }

    pub fn stats(&self) {
        // total repos count
        // repos count by vcs
        // repos by host?
    }

    pub fn cleanup(&self) {
        // Find out repo directories which are not in metadata, then delete them.
    }

    pub fn search(&self, keyword: &str) {
        // Query topics and repo_url in metadata to find out matched repos
    }

    pub fn proxy(&self) -> metadata::Proxy {
        let md = self.metadata.clone();
        md.proxy
    }
}
