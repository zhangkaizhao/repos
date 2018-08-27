use std::path::Path;
use std::process::Command;

use super::Vcs;

pub struct Hg;

impl Hg {
    /// Create a new `Hg`.
    pub fn new() -> Self {
        Hg
    }
}

impl Vcs for Hg {
    fn clone(&self, url: &str, path: &Path, bare: bool, use_proxy: bool) {
        // proxy in env
        let mut child = Command::new("hg")
            .arg("clone")
            .arg(url)
            .arg(path)
            .spawn()
            .unwrap();
        let _result = child.wait().unwrap();
        ()
    }

    fn update(&self, path: &Path, use_proxy: bool) {
        // proxy in env
        let mut child = Command::new("hg")
            .arg("update")
            .current_dir(path)
            .spawn()
            .unwrap();
        let _result = child.wait().unwrap();
        ()
    }
}
