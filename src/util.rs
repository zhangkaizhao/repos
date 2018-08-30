use std::fs;
use std::path::Path;

use url::Url;

/// Get repo server (host[:port]) from repo url.
pub fn repo_server_from_url(url: &str) -> String {
    let parsed = Url::parse(url).unwrap();
    let host = parsed.host_str().unwrap();
    let port = parsed.port();
    // URL scheme is ignored.
    let server = match port {
        None => host.to_owned(),
        _ => host.to_owned() + ":" + &port.unwrap().to_string(),
    };
    server.to_string()
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
        println!(
            "Found repo directory: {}. Try to delete it...",
            &local_relpath
        );
        fs::remove_dir_all(relpath).unwrap_or_else(|why| {
            println!(
                "Failed to delete repo directory: {}: {:?}.",
                &local_relpath,
                why.kind()
            );
        });
        // Notify
        println!("Local repo directory: {} is deleted.", &local_relpath);
    } else if relpath.exists() {
        // Delete it whatever.
        println!(
            "The repo path: {} is not a directory. Try to delete it whatever...",
            &local_relpath
        );
        fs::remove_file(relpath).unwrap_or_else(|why| {
            println!(
                "Failed to delete repo path: {}: {:?}.",
                &local_relpath,
                why.kind()
            );
        });
        // Notify
        println!("Local repo path: {} is deleted.", &local_relpath);
    } else {
        // Repo directory does not exist.
        println!("The repo directory: {} does not exists.", &local_relpath);
    }
    println!("Please manually delete repo from metadata file.");
}

/// Generate http proxy url.
pub fn gen_proxy_url(scheme: &str, host: &str, port: u16) -> String {
    let proxy_url = scheme.to_owned() + "://" + host + ":" + &port.to_string();
    proxy_url.to_string()
}

/// Cleanup empty directories under root directory.
pub fn cleanup_empty_subdirs(root_dir: &Path) {
    _cleanup_empty_subdirs(root_dir, root_dir.clone());
}

/// Cleanup empty directories under current directory.
fn _cleanup_empty_subdirs(dir: &Path, root_dir: &Path) {
    let entries = fs::read_dir(dir).unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_dir() {
            // Ignore hidden directory which starts with ".", e.g. ".git".
            if !path.file_name().unwrap().to_str().unwrap().starts_with(".") {
                _cleanup_empty_dirs(&path, &root_dir.clone());
            }
        }
    }
}

/// Cleanup empty directories including current directory.
fn _cleanup_empty_dirs(dir: &Path, root_dir: &Path) {
    if dir == root_dir {
        // The parent directory may be root directory.
        _cleanup_empty_subdirs(&dir, &root_dir);
    } else {
        // Try to remove empty directory.
        match fs::remove_dir(&dir) {
            Ok(_) => {
                println!("Empty directory {} is removed.", dir.display());
                // Then try to remove parent empty directory.
                let parent_dir = dir.parent().unwrap();
                _cleanup_empty_dirs(&parent_dir, &root_dir.clone());
            }
            Err(_) => {
                // Not empty directory, continue to its sub-directories.
                _cleanup_empty_subdirs(&dir, &root_dir);
            }
        }
    }
}
