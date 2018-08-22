use url::Url;

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
