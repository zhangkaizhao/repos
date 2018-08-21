use std::path::Path;
use std::process::Command;

use super::Vcs;

pub struct Git;

impl Git {
    /// Create a new `Git`.
    pub fn new() -> Self {
        Git
    }
}

impl Vcs for Git {
    fn clone(&self, url: &str, path: Path, bare: bool, use_proxy: bool) {
        // proxy in env
        let mut child = Command::new("git").arg("clone").arg(url).arg(path).spawn().unwrap();
        let _result = child.wait().unwrap();
        Ok()
    }

    fn update(&self, path: Path, use_proxy: bool) {
        // proxy in env
        // change current directory
        let mut child = Command::new("git").arg("pull").spawn().unwrap();
        let _result = child.wait().unwrap();
        Ok()
    }
}
