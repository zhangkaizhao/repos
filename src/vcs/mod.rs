pub mod git;
pub mod hg;

use std::path::Path;

pub trait Vcs {
    /// Clone a new repo to path.
    fn clone(&self, url: &str, path: &Path, bare: bool, use_proxy: bool);

    /// Update an existed repo in path.
    fn update(&self, path: &Path, use_proxy: bool);
}

/// Clone a new repo to path by vcs.
pub fn clone(vcs: &str, url: &str, path: &Path, bare: bool, use_proxy: bool) {
    match vcs {
        "git" => git::Git::new().clone(url, path, bare, use_proxy),
        "hg" => hg::Hg::new().clone(url, path, bare, use_proxy),
        _ => panic!("Not supported vcs: {}", vcs),
    }
}

/// Update an existed repo in path by vcs.
pub fn update(vcs: &str, path: &Path, use_proxy: bool) {
    match vcs {
        "git" => git::Git::new().update(path, use_proxy),
        "hg" => hg::Hg::new().update(path, use_proxy),
        _ => panic!("Not supported vcs: {}", vcs),
    }
}
