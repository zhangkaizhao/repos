use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use toml;

use super::util;

#[derive(Deserialize)]
pub struct Metadata {
    pub repos: HashMap<String, Repo>,
    pub proxy: Proxy,
}

#[derive(Deserialize)]
pub struct Repo {
    pub vcs: String,
    pub allow_sync: bool,
    pub bare: bool,
    pub use_proxy: bool,
    pub topics: Vec<String>,
}

#[derive(Deserialize)]
pub struct Proxy {
    pub scheme: String,
    pub host: String,
    pub port: u16,
}

/// Load metadata from file path.
pub fn load(path: &Path) -> Result<Metadata, String> {
    let path_display = path.display();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!(
                "Couldn't open {}: {}",
                path_display,
                err.to_string()
            ))
        }
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {}
        Err(err) => {
            return Err(format!(
                "Couldn't read {}: {}",
                path_display,
                err.to_string()
            ))
        }
    }

    loads(&content)
}

/// Load metadata from string slices.
pub fn loads(content: &str) -> Result<Metadata, String> {
    let md: Metadata = match toml::from_str(&content) {
        Ok(md) => md,
        Err(err) => {
            return Err(format!(
                "Couldn't parse as toml format because of: {}",
                err.to_string()
            ));
        }
    };

    // Validate repo url.
    let mut urls_errors: HashMap<String, String> = HashMap::new();
    for url in md.repos.keys() {
        match util::validate_repo_url(url) {
            Err(err) => {
                urls_errors.insert(url.to_string(), err.to_string());
            }
            Ok(_) => {}
        }
    }
    if !urls_errors.is_empty() {
        for (url, error) in urls_errors {
            println!("Url '{}' is unsupported because of: {}.", url, error);
        }
        Err("Unsupported repository urls found in metadata file.".to_string())
    } else {
        Ok(md)
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use std::io::Write;
    use std::path::Path;

    use super::{load, loads};

    static TEMP_CONTENT: &'static str = "
        [repos.'https://github.com/org/repo.git']
        vcs = 'git'
        allow_sync = true
        bare = false
        use_proxy = false
        topics = ['topic1', 'topic2']

        [proxy]
        scheme = 'socks5'
        host = '127.0.0.1'
        port = 1080
    ";

    #[test]
    fn md_loads() {
        let md = loads(&TEMP_CONTENT).unwrap();

        assert_eq!(md.proxy.scheme, "socks5");
        assert_eq!(md.proxy.host, "127.0.0.1");
        assert_eq!(md.proxy.port, 1080);
    }

    #[test]
    fn md_load() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "{}", &TEMP_CONTENT).unwrap();

        let path = tmpfile.path();

        let filepath = Path::new(path);
        let md = load(&filepath).unwrap();

        assert_eq!(md.proxy.scheme, "socks5");
        assert_eq!(md.proxy.host, "127.0.0.1");
        assert_eq!(md.proxy.port, 1080);
    }
}
