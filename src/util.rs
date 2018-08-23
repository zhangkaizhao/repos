use std::fs;
use std::path::Path;

use url::Url;

/// Get repo host from repo url.
pub fn repo_host_from_url(url: &str) -> String {
    let parsed = Url::parse(url).unwrap();
    let host = parsed.host_str().unwrap();
    host.to_string()
}

/// Convert repo url to relative repo directory.
pub fn repo_url_to_relpath(url: &str) -> String {
    let parsed = Url::parse(url).unwrap();
    let host = parsed.host_str().unwrap();
    let port = parsed.port();
    let path = parsed.path();
    let relpath = match port {
        None => host.to_owned() + path,
        _ => host.to_owned() + ":" + &port.unwrap().to_string() + path,
    };
    relpath.trim_right_matches(".git").to_string()
}

/// Delete repo relative path.
pub fn delete_repo_relpath(relpath: &Path) {
    let local_relpath = relpath.to_str().unwrap();
    if relpath.is_dir() {
        // Delete repo directory
        println!("Found repo directory: {}. Try to delete it...", &local_relpath);
        fs::remove_dir_all(relpath).unwrap_or_else(|why| {
            println!("Failed to delete repo directory: {}: {:?}.", &local_relpath, why.kind());
        });
        // TODO recurve removing empty directory.
        // Notify
        println!("Local repo directory: {} is deleted.", &local_relpath);
    } else if relpath.exists() {
        // Delete it whatever.
        println!("The repo path: {} is not a directory. Try to delete it whatever...",
                 &local_relpath);
        fs::remove_file(relpath).unwrap_or_else(|why| {
            println!("Failed to delete repo path: {}: {:?}.", &local_relpath, why.kind());
        });
        // Notify
        println!("Local repo path: {} is deleted.", &local_relpath);
    } else {
        // Repo directory does not exist.
        println!("The repo directory: {} does not exists.", &local_relpath);
    }
    println!("Please manually delete repo from metadata file.");
}
