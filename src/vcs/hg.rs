use std::path::Path;
use std::process::Command;

use super::super::metadata::Proxy;
use super::util::gen_proxy_env_vars;
use super::Vcs;

pub struct Hg;

impl Hg {
    /// Create a new `Hg`.
    pub fn new() -> Self {
        Hg
    }
}

impl Vcs for Hg {
    fn clone(&self, url: &str, path: &Path, bare: bool, proxy: Option<&Proxy>) {
        // Build command arguments.
        let mut args = Vec::new();
        args.push("clone");
        if bare {
            args.push("--noupdate");
        }
        args.push(url);
        args.push(path.to_str().unwrap());

        let proxy_env_vars = gen_proxy_env_vars(proxy);
        match Command::new("hg").args(&args).envs(&proxy_env_vars).spawn() {
            Ok(mut child) => match child.wait() {
                Ok(_status) => {}
                Err(err) => println!("Failed to clone repository because of: {}", err.to_string()),
            },
            Err(err) => println!("Failed to execute hg clone because of: {}", err.to_string()),
        }
    }

    fn update(&self, path: &Path, bare: bool, proxy: Option<&Proxy>) {
        // Build command arguments.
        let mut args = Vec::new();
        args.push("pull");
        if !bare {
            args.push("--update");
        }

        let proxy_env_vars = gen_proxy_env_vars(proxy);
        match Command::new("hg")
            .args(&args)
            .current_dir(path)
            .envs(&proxy_env_vars)
            .spawn()
        {
            Ok(mut child) => match child.wait() {
                Ok(_status) => {}
                Err(err) => println!(
                    "Failed to update repository because of: {}",
                    err.to_string()
                ),
            },
            Err(err) => println!("Failed to execute hg pull because of: {}", err.to_string()),
        }
    }
}
