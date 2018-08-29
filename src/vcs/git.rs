use std::path::Path;
use std::process::Command;

use super::super::metadata::Proxy;
use super::util::gen_proxy_env_vars;
use super::Vcs;

pub struct Git;

impl Git {
    /// Create a new `Git`.
    pub fn new() -> Self {
        Git
    }
}

impl Vcs for Git {
    fn clone(&self, url: &str, path: &Path, bare: bool, proxy: Option<Proxy>) {
        let proxy_env_vars = gen_proxy_env_vars(proxy);
        let mut child = Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(path)
            .envs(&proxy_env_vars)
            .spawn()
            .unwrap();
        let _result = child.wait().unwrap();
        ()
    }

    fn update(&self, path: &Path, proxy: Option<Proxy>) {
        let proxy_env_vars = gen_proxy_env_vars(proxy);
        let mut child = Command::new("git")
            .arg("pull")
            .current_dir(path)
            .envs(&proxy_env_vars)
            .spawn()
            .unwrap();
        let _result = child.wait().unwrap();
        ()
    }
}
