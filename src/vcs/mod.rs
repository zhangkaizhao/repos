pub mod git;
pub mod hg;
pub mod util;

use std::path::Path;

use super::metadata::Proxy;

pub trait Vcs {
    /// Clone a new repository to path.
    fn clone(&self, url: &str, path: &Path, bare: bool, proxy: Option<&Proxy>);

    /// Update an existed repository in path.
    fn update(&self, path: &Path, bare: bool, proxy: Option<&Proxy>);
}

/// Clone a new repository to path by vcs.
pub fn clone(vcs: &str, url: &str, path: &Path, bare: bool, proxy: Option<&Proxy>) {
    match vcs {
        "git" => git::Git::new().clone(url, path, bare, proxy),
        "hg" => hg::Hg::new().clone(url, path, bare, proxy),
        _ => println!("Unsupported vcs '{}'.", vcs),
    }
}

/// Update an existed repository in path by vcs.
pub fn update(vcs: &str, path: &Path, bare: bool, proxy: Option<&Proxy>) {
    match vcs {
        "git" => git::Git::new().update(path, bare, proxy),
        "hg" => hg::Hg::new().update(path, bare, proxy),
        _ => println!("Unsupported vcs '{}'.", vcs),
    }
}
