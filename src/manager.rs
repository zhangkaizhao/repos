/// Subcommands:
/// - sync [url]
/// - remove [url]
/// - topics
/// - topic [topic]
/// - stats
/// - cleanup
/// - search [keyword]
/// - proxy
use std::collections::BTreeMap;
use std::path::Path;

use remove_empty_subdirs::remove_empty_subdirs;

use metadata::{self, Metadata};
use util;
use vcs;

pub struct Manager {
    root_dir: &'static str,
    metadata: Metadata,
}

impl Manager {
    /// Create a new `Manager`.
    pub fn new() -> Result<Self, String> {
        let root_path = Path::new(".");
        let root_dir = root_path.to_str().unwrap();

        let md_file = "Repos.toml";
        let md_path = Path::new(&md_file);

        match metadata::load(&md_path) {
            Ok(md) => {
                let manager = Manager {
                    root_dir: root_dir,
                    metadata: md,
                };
                Ok(manager)
            }
            Err(err) => Err(format!(
                "Couldn't load metadata because of: {}",
                err.to_string()
            )),
        }
    }

    fn _sync(&self, url: &str, repo: &metadata::Repo) {
        if !repo.allow_sync {
            println!(
                "No need to sync respository '{}' because of disallowed.",
                url
            );
            return ();
        }

        let local_relpath = util::repo_url_to_relpath(url).unwrap();
        let relpath = Path::new(&local_relpath);
        let _vcs = &repo.vcs;
        let bare = repo.bare;
        let use_proxy = repo.use_proxy;
        let proxy = match use_proxy {
            true => Some(&self.metadata.proxy),
            false => None,
        };
        if relpath.is_dir() {
            // Update
            vcs::update(_vcs, &relpath, bare, proxy);
        } else {
            // Clone
            vcs::clone(_vcs, &url, &relpath, bare, proxy);
        }
    }

    /// Update an existed repository or clone a new repository.
    pub fn sync(&self, url: &str) {
        // 1. Check whether repo exists in metadata.
        // 1.1 Warn repo does not exist in metadata, should be manually added first.
        // 2. Clone if repo directory does not exist.
        // 3. Update if repo directory exists.
        let repositories = &self.metadata.repos;
        if repositories.contains_key(url) {
            let repo = repositories.get(url).unwrap();
            // Check if url +/- '.git' exists.
            if let Some(alternative_url) = util::gen_alternative_url(&repo.vcs, url) {
                if repositories.contains_key(&alternative_url) {
                    // Warn if alternative url exists in metadata.
                    println!(
                        "Warning: repository with alternative url '{}' exists already.",
                        alternative_url
                    );
                    return ();
                }
            }
            self._sync(&url, &repo);
        } else {
            // Warn if no same url or alternative url exists in metadata.
            println!("Repository '{}' has not been put in metadata yet.", url);
        }
    }

    /// Update all repositories.
    pub fn sync_all(&self) {
        // 1. Read each repo from metadata.
        // 2. Update each repo.
        for (url, repo) in &self.metadata.repos {
            println!("Sync repository '{}'...", &url);
            self._sync(&url, &repo);
        }
    }

    /// Remove local directory of a repository.
    pub fn remove(&self, url: &str) {
        // 1. Check whether repo exists in metadata.
        // 1.1 Warn repo exists in metadata, should be manually deleted first.
        // 2. Delete repo directory.
        // 3. Notify user to manually delete repo from metadata file.
        let repositories = &self.metadata.repos;
        if repositories.contains_key(url) {
            let local_relpath = util::repo_url_to_relpath(url).unwrap();
            let relpath = Path::new(&local_relpath);
            util::delete_repo_relpath(relpath);
        } else {
            // warn
            println!("Repository '{}' has not been put in metadata yet.", url);
        }
    }

    /// Output topics with count of their repositories.
    pub fn topics(&self) {
        // repos count by each topic
        let mut topic_repo_counts: BTreeMap<&str, i32> = BTreeMap::new();
        for (_, repo) in &self.metadata.repos {
            let topics = &repo.topics;
            for topic in topics {
                let counter = topic_repo_counts.entry(&topic).or_insert(0);
                *counter += 1;
            }
        }
        let topics_count = topic_repo_counts.len();
        println!("There are {} topics:", topics_count);
        for (topic, counter) in &topic_repo_counts {
            println!("* {}: {} repositories", &topic, counter);
        }
    }

    /// List repositories of a topic.
    pub fn topic(&self, _topic: &str) {
        // list all repos in topic
        let mut urls = Vec::new();
        for (url, repo) in &self.metadata.repos {
            let topics = &repo.topics;
            if topics.contains(&_topic.to_string()) {
                urls.push(url);
            }
        }
        println!("Topic '{}' has {} repositories:", _topic, urls.len());
        urls.sort_unstable();
        for url in urls {
            println!("* {}", url)
        }
    }

    /// Output stats of all repositories.
    pub fn stats(&self) {
        // total/bare/sync_allowed/proxy_used/synced repos count
        // repos count by vcs
        // repos count by each topic
        // repos by server (host[:port])

        let mut repositories_count = 0;
        let mut bare_repo_count = 0;
        let mut sync_allowed_repo_count = 0;
        let mut synced_repo_count = 0;
        let mut proxy_used_repo_count = 0;
        let mut vcs_repo_counts: BTreeMap<&str, i32> = BTreeMap::new();
        let mut topic_repo_counts: BTreeMap<&str, i32> = BTreeMap::new();
        let mut server_repo_counts: BTreeMap<String, i32> = BTreeMap::new();

        for (url, repo) in &self.metadata.repos {
            repositories_count += 1;

            if repo.bare {
                bare_repo_count += 1;
            }

            if repo.allow_sync {
                sync_allowed_repo_count += 1;
            }

            if repo.use_proxy {
                proxy_used_repo_count += 1;
            }

            let local_relpath = util::repo_url_to_relpath(url).unwrap();
            let relpath = Path::new(&local_relpath);
            // Only check whether local relative path is a directory.
            if relpath.is_dir() {
                synced_repo_count += 1;
            }

            let vcs = &repo.vcs;
            let vcs_counter = vcs_repo_counts.entry(&vcs).or_insert(0);
            *vcs_counter += 1;

            let topics = &repo.topics;
            for topic in topics {
                let counter = topic_repo_counts.entry(&topic).or_insert(0);
                *counter += 1;
            }

            let server = util::repo_server_from_url(&url).unwrap();
            let server_counter = server_repo_counts.entry(server).or_insert(0);
            *server_counter += 1;
        }

        println!("There are {} repositories:", repositories_count);
        println!("* Bare: {} repositories", bare_repo_count);
        println!("* Sync allowed: {} repositories", sync_allowed_repo_count);
        println!("* Proxy used: {} repositories", proxy_used_repo_count);
        println!("* Synced: {} repositories", synced_repo_count);

        let vcs_count = vcs_repo_counts.len();
        println!("There are {} vcs:", vcs_count);
        for (vcs, counter) in &vcs_repo_counts {
            println!("* {}: {} repositories", &vcs, counter);
        }

        let topics_count = topic_repo_counts.len();
        println!("There are {} topics:", topics_count);
        for (topic, counter) in &topic_repo_counts {
            println!("* {}: {} repositories", &topic, counter);
        }

        let servers_count = server_repo_counts.len();
        println!("There are {} servers:", servers_count);
        for (server, counter) in &server_repo_counts {
            println!("* {}: {} repositories", server, counter);
        }
    }

    /// Clean up unused resources.
    pub fn cleanup(&self) {
        // TODO Find out repo directories which are not in metadata, then delete them.

        // cleanup unused empty directories
        let root_path = Path::new(self.root_dir);
        match remove_empty_subdirs(root_path) {
            Ok(()) => {}
            Err(err) => println!("Unexpected error: {}", err.to_string()),
        }
    }

    /// Search repositories by keyword.
    pub fn search(&self, keyword: &str) {
        // Query topics and repo_url in metadata to find out matched repos
        let mut urls_by_url = Vec::new();
        let mut urls_by_topic: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for (url, repo) in &self.metadata.repos {
            if url.contains(&keyword) {
                urls_by_url.push(url);
            }
            let topics = &repo.topics;
            for topic in topics {
                if topic.contains(&keyword) {
                    let mut _topics = urls_by_topic.entry(&url).or_insert(Vec::new());
                    _topics.push(topic);
                }
            }
        }

        urls_by_url.sort_unstable();
        println!(
            "There are {} repositories matched by url:",
            urls_by_url.len()
        );
        for url in urls_by_url {
            println!("* {}", &url);
        }

        println!(
            "There are {} repositories matched by topic:",
            urls_by_topic.len()
        );
        for (url, topics) in &urls_by_topic {
            println!("* {} ({})", &url, topics.join(", "));
        }
    }

    /// Output proxy configuration.
    pub fn proxy(&self) {
        let proxy = &self.metadata.proxy;
        println!("Proxy configuration:");
        println!("* scheme: {}", proxy.scheme);
        println!("* host: {}", proxy.host);
        println!("* port: {}", proxy.port);
    }
}
