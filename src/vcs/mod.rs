pub mod git;
pub mod hg;
pub mod util;

use std::path::Path;

use super::metadata::Proxy;

pub trait Vcs {
    /// Clone a new repo to path.
    fn clone(&self, url: &str, path: &Path, bare: bool, proxy: Option<Proxy>);

    /// Update an existed repo in path.
    fn update(&self, path: &Path, bare: bool, proxy: Option<Proxy>);
}

/// Clone a new repo to path by vcs.
pub fn clone(vcs: &str, url: &str, path: &Path, bare: bool, proxy: Option<Proxy>) {
    match vcs {
        "git" => git::Git::new().clone(url, path, bare, proxy),
        "hg" => hg::Hg::new().clone(url, path, bare, proxy),
        _ => panic!("Not supported vcs {}.", vcs),
    }
}

/// Update an existed repo in path by vcs.
pub fn update(vcs: &str, path: &Path, bare: bool, proxy: Option<Proxy>) {
    match vcs {
        "git" => git::Git::new().update(path, bare, proxy),
        "hg" => hg::Hg::new().update(path, bare, proxy),
        _ => panic!("Not supported vcs {}.", vcs),
    }
}
