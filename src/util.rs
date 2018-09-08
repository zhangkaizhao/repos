use std::fs;
use std::path::Path;

use url::Url;

/// Validate repo url.
/// Notes:
/// * Relative URLs without base (scp-like syntax) are not supported.
///   e.g. `[user@]host.xz:path/to/repo.git` or `[user@]host.xz:~/path/to/repo.git`
pub fn validate_repo_url(url: &str) -> Result<(), String> {
    match Url::parse(url) {
        Ok(_parsed) => Ok(()),
        Err(err) => return Err(format!("{}", err.to_string())),
    }
}

/// Get repo server (host[:port]) from repo url.
pub fn repo_server_from_url(url: &str) -> Result<String, String> {
    let parsed = match Url::parse(url) {
        Ok(parsed) => parsed,
        Err(err) => return Err(format!("{}", err.to_string())),
    };
    let host = parsed.host_str().unwrap();
    let port = parsed.port();
    // URL scheme is ignored.
    let server = match port {
        None => host.to_owned(),
        _ => host.to_owned() + ":" + &port.unwrap().to_string(),
    };
    Ok(server.to_string())
}

/// Convert repo url to relative repo directory.
pub fn repo_url_to_relpath(url: &str) -> Result<String, String> {
    let parsed = match Url::parse(url) {
        Ok(parsed) => parsed,
        Err(err) => return Err(format!("{}", err.to_string())),
    };
    let host = parsed.host_str().unwrap();
    let port = parsed.port();
    let path = parsed.path();
    let relpath = match port {
        None => host.to_owned() + path,
        _ => host.to_owned() + ":" + &port.unwrap().to_string() + path,
    };
    // TODO bare repository needs ".git" suffix?
    Ok(relpath.trim_right_matches(".git").to_string())
}

/// Delete repo relative path.
pub fn delete_repo_relpath(relpath: &Path) {
    let local_relpath = relpath.to_str().unwrap();
    if relpath.is_dir() {
        // Delete repo directory
        println!(
            "Found repository directory {}. Try to delete it...",
            &local_relpath
        );
        match fs::remove_dir_all(relpath) {
            Ok(_) => {
                println!("Local repository directory {} is deleted.", &local_relpath);
                println!("Please manually delete repository from metadata file.");
            }
            Err(err) => println!(
                "Failed to delete repository directory {} because of: {}",
                &local_relpath,
                err.to_string()
            ),
        }
    } else if relpath.exists() {
        // Delete it whatever.
        println!(
            "The repository path {} is not a directory. Try to delete it whatever...",
            &local_relpath
        );
        match fs::remove_file(relpath) {
            Ok(_) => {
                println!("Local repository path {} is deleted.", &local_relpath);
                println!("Please manually delete repository from metadata file.");
            }
            Err(err) => println!(
                "Failed to delete repository path {} because of: {}",
                &local_relpath,
                err.to_string()
            ),
        }
    } else {
        // Repo directory does not exist.
        println!(
            "The repository directory {} does not exists.",
            &local_relpath
        );
    }
}

/// Generate http proxy url.
pub fn gen_proxy_url(scheme: &str, host: &str, port: u16) -> String {
    let proxy_url = scheme.to_owned() + "://" + host + ":" + &port.to_string();
    proxy_url.to_string()
}

/// Generate alternative url for vcs.
pub fn gen_alternative_url(vcs: &str, url: &str) -> Result<String, String> {
    if vcs == "git" {
        let alternative_url = if url.ends_with(".git") {
            url.trim_right_matches(".git").to_string()
        } else {
            url.to_string() + ".git"
        };
        Ok(alternative_url)
    } else {
        Err(format!("Unsupported vcs `{}`.", vcs))
    }
}
