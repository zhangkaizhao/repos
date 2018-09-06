use std::path::Path;
use std::process::Command;

use super::super::metadata::Proxy;
use super::util::gen_proxy_env_vars;
use super::Vcs;

pub struct Git;

impl Git {
    /// Create a new `Git`.
    pub fn new() -> Self {
        Git
    }
}

impl Vcs for Git {
    fn clone(&self, url: &str, path: &Path, bare: bool, proxy: Option<Proxy>) {
        // Build command arguments.
        let mut args = Vec::new();
        args.push("clone");
        if bare {
            args.push("--bare");
        }
        args.push(url);
        args.push(path.to_str().unwrap());

        let proxy_env_vars = gen_proxy_env_vars(proxy);
        match Command::new("git")
            .args(&args)
            .envs(&proxy_env_vars)
            .spawn()
        {
            Ok(mut child) => match child.wait() {
                Ok(_status) => {}
                Err(err) => println!("Failed to clone repository: {}", err.to_string()),
            },
            Err(err) => println!("Failed to execute git clone: {}", err.to_string()),
        }
    }

    fn update(&self, path: &Path, bare: bool, proxy: Option<Proxy>) {
        // Build command arguments.
        let mut args = Vec::new();
        if bare {
            args.push("fetch");
        } else {
            args.push("pull");
        }

        let proxy_env_vars = gen_proxy_env_vars(proxy);
        match Command::new("git")
            .args(&args)
            .current_dir(path)
            .envs(&proxy_env_vars)
            .spawn()
        {
            Ok(mut child) => match child.wait() {
                Ok(_status) => {}
                Err(err) => println!("Failed to update repository: {}", err.to_string()),
            },
            Err(err) => {
                if bare {
                    println!("Failed to execute git fetch: {}", err.to_string());
                } else {
                    println!("Failed to execute git pull: {}", err.to_string());
                }
            }
        }
    }
}
