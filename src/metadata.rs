use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use toml;

#[derive(Clone, Deserialize)]
pub struct Metadata {
    pub repos: HashMap<String, Repo>,
    pub proxy: Proxy,
}

#[derive(Clone, Deserialize)]
pub struct Repo {
    pub vcs: String,
    pub cloned: bool,
    pub bare: bool,
    pub use_proxy: bool,
    pub topics: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct Proxy {
    pub scheme: String,
    pub host: String,
    pub port: u16,
}

pub fn load(path: &Path) -> Metadata {
    let path_display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path_display, why.to_string()),
        Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("Couldn't read {}: {}", path_display, why.to_string()),
        Ok(_) => println!("Read metadata from {} successfully.", path_display),
    }

    loads(&content)
}

pub fn loads(content: &str) -> Metadata {
    let metadata: Metadata = toml::from_str(&content).unwrap();
    metadata
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
        cloned = true
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
        let md = loads(&TEMP_CONTENT);

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
        let md = load(&filepath);

        assert_eq!(md.proxy.scheme, "socks5");
        assert_eq!(md.proxy.host, "127.0.0.1");
        assert_eq!(md.proxy.port, 1080);
    }
}
