extern crate toml;

extern crate repos;

use std::env;

fn proxy(_repos: &repos::repos::Repos) {
    let proxy = _repos.proxy();
    println!("Proxy configuration:");
    println!("* scheme: {}", proxy.scheme);
    println!("* host: {}", proxy.host);
    println!("* port: {}", proxy.port);
}

fn main() {
    let _repos = repos::repos::Repos::new();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let subcommand = &args[1];
            match &subcommand[..] {
                "proxy" => proxy(&_repos),
                _ => println!("Sorry, not implemented yet!"),
            }
        },
        _ => println!("Usage: repos subcommand [arguments]"),
    }
}
