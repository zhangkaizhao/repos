use std::path::Path;
use std::process::Command;

use super::super::metadata::Proxy;
use super::util::gen_proxy_env_vars;
use super::Vcs;

pub struct Hg;

impl Hg {
    /// Create a new `Hg`.
    pub fn new() -> Self {
        Hg
    }
}

impl Vcs for Hg {
    fn clone(&self, url: &str, path: &Path, bare: bool, proxy: Option<Proxy>) {
        let proxy_env_vars = gen_proxy_env_vars(proxy);
        let mut child = Command::new("hg")
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
        let mut child = Command::new("hg")
            .arg("update")
            .current_dir(path)
            .envs(&proxy_env_vars)
            .spawn()
            .unwrap();
        let _result = child.wait().unwrap();
        ()
    }
}
