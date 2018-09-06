/// Repos subcommands:
/// - sync
/// - remove
/// - topics
/// - topic
/// - stats
/// - cleanup
/// - search
/// - proxy
use std::collections::BTreeMap;
use std::path::Path;

use remove_empty_subdirs::remove_empty_subdirs;

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

        // let md_file = root_dir.to_owned() + "/Repos.toml";
        let md_file = "Repos.toml";
        let md_path = Path::new(&md_file);
        let md = metadata::load(&md_path);

        Repos {
            root_dir: root_dir,
            metadata: md,
        }
    }

    fn _sync(&self, url: &str, repo: &metadata::Repo) {
        if !repo.allow_sync {
            println!("No need to sync `{}` because of disallowed.", url);
            return ();
        }

        let local_relpath = util::repo_url_to_relpath(url);
        let relpath = Path::new(&local_relpath);
        let _vcs = &repo.vcs;
        let bare = repo.bare;
        let use_proxy = repo.use_proxy;
        let proxy = match use_proxy {
            true => Some(self.metadata.proxy.clone()),
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

    /// Update an existed repo or clone a new repo.
    pub fn sync(&self, url: &str) {
        // 1. Check whether repo exists in metadata.
        // 1.1 Warn repo does not exist in metadata, should be manually added first.
        // 2. Clone if repo directory does not exist.
        // 3. Update if repo directory exists.
        let repositories = self.metadata.repos.clone();
        if repositories.contains_key(url) {
            // Check if url +/- '.git' exists.
            let alternative_url = if url.ends_with(".git") {
                url.trim_right_matches(".git").to_string()
            } else {
                url.to_string() + ".git"
            };
            if repositories.contains_key(&alternative_url) {
                // Warn if alternative url exists in metadata.
                panic!(
                    "Warning: repository with alternative url `{}` exists already.",
                    alternative_url
                );
            }

            let repo = repositories.get(url).unwrap();
            self._sync(&url, &repo);
        } else {
            // Warn if no same url or alternative url exists in metadata.
            panic!("Repo `{}` has not been put in metadata yet.", url);
        }
    }

    pub fn sync_all(&self) {
        // 1. Read each repo from metadata.
        // 2. Update each repo.
        let repositories = self.metadata.repos.clone();
        for (url, repo) in &repositories {
            // TODO handle subprocess exceptions.
            println!("Sync repo `{}`...", &url);
            self._sync(&url, &repo);
        }
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
        // repos count by each topic
        let repositories = self.metadata.repos.clone();
        let mut topic_repo_counts: BTreeMap<&str, i32> = BTreeMap::new();
        for (_, repo) in &repositories {
            let topics = &repo.topics;
            for topic in topics {
                let counter = topic_repo_counts.entry(&topic).or_insert(0);
                *counter += 1;
            }
        }
        let topics_count = topic_repo_counts.len();
        println!("There are {} topics now:", topics_count);
        for (topic, counter) in &topic_repo_counts {
            println!("* {}: {} repositories", &topic, counter);
        }
    }

    pub fn topic(&self, _topic: &str) {
        // list all repos in topic
        let repositories = self.metadata.repos.clone();
        let mut urls = Vec::new();
        for (url, repo) in &repositories {
            let topics = &repo.topics;
            if topics.contains(&_topic.to_string()) {
                urls.push(url);
            }
        }
        println!("Topic: {} has {} repositories:", _topic, urls.len());
        urls.sort_unstable();
        for url in urls {
            println!("* {}", url)
        }
    }

    pub fn stats(&self) {
        // total repos count
        // repos count by vcs
        // repos count by each topic
        // repos by server (host[:port])

        // TODO repos allow synced but not cloned yet?

        let repositories = self.metadata.repos.clone();

        let mut repositories_count = 0;
        let mut vcs_repo_counts: BTreeMap<&str, i32> = BTreeMap::new();
        let mut topic_repo_counts: BTreeMap<&str, i32> = BTreeMap::new();
        let mut server_repo_counts: BTreeMap<String, i32> = BTreeMap::new();

        for (url, repo) in &repositories {
            repositories_count += 1;

            let vcs = &repo.vcs;
            let vcs_counter = vcs_repo_counts.entry(&vcs).or_insert(0);
            *vcs_counter += 1;

            let topics = &repo.topics;
            for topic in topics {
                let counter = topic_repo_counts.entry(&topic).or_insert(0);
                *counter += 1;
            }

            let server = util::repo_server_from_url(&url);
            let server_counter = server_repo_counts.entry(server).or_insert(0);
            *server_counter += 1;
        }

        println!("There are totally {} repositories.", repositories_count);

        let topics_count = topic_repo_counts.len();
        println!("There are {} topics now:", topics_count);
        for (topic, counter) in &topic_repo_counts {
            println!("* {}: {} repositories", &topic, counter);
        }

        let servers_count = server_repo_counts.len();
        println!("There are {} servers now:", servers_count);
        for (server, counter) in &server_repo_counts {
            println!("* {}: {} repositories", server, counter);
        }
    }

    pub fn cleanup(&self) {
        // TODO Find out repo directories which are not in metadata, then delete them.

        // cleanup unused empty directories
        let root_path = Path::new(self.root_dir);
        match remove_empty_subdirs(root_path) {
            Ok(()) => {}
            Err(err) => println!("Unexpected error: {:?}", err.to_string()),
        }
    }

    pub fn search(&self, keyword: &str) {
        // Query topics and repo_url in metadata to find out matched repos
        let repositories = self.metadata.repos.clone();
        let mut urls_by_url = Vec::new();
        let mut urls_by_topic: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for (url, repo) in &repositories {
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

    pub fn proxy(&self) {
        let proxy = self.metadata.proxy.clone();
        println!("Proxy configuration:");
        println!("* scheme: {}", proxy.scheme);
        println!("* host: {}", proxy.host);
        println!("* port: {}", proxy.port);
    }
}
