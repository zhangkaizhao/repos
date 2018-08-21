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

    pub fn sync(&self, url: &str) {
    }

    pub fn remove(&self, url: &str) {
    }

    pub fn topics(&self, topic: &str) {
    }

    pub fn stats(&self) {
    }

    pub fn cleanup(&self) {
    }

    pub fn search(&self) {
    }

    pub fn proxy(&self) -> metadata::Proxy {
        let md = self.metadata.clone();
        md.proxy
    }
}
