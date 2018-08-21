pub mod git;
pub mod hg;

use std::path::Path;

pub trait Vcs {
    /// Clone a new repo to path
    fn clone(&self, url: &str, path: &Path, bare: bool, use_proxy: bool);

    /// Update an existed repo in path
    fn update(&self, path: &Path, use_proxy: bool);
}
