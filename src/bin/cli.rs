extern crate toml;

extern crate repos;

use std::env;

fn main() {
    let _repos = repos::repos::Repos::new();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let subcommand = &args[1];
            match &subcommand[..] {
                "sync" => _repos.sync_all(),
                "topics" => _repos.topics(),
                "stats" => _repos.stats(),
                "cleanup" => _repos.cleanup(),
                "proxy" => _repos.proxy(),
                _ => println!("Sorry, not implemented yet!"),
            }
        }
        3 => {
            let subcommand = &args[1];
            let argument = &args[2];
            match &subcommand[..] {
                "sync" => _repos.sync(&argument),
                "remove" => _repos.remove(&argument),
                "topic" => _repos.topic(&argument),
                "search" => _repos.search(&argument),
                _ => println!("Sorry, not implemented yet!"),
            }
        }
        _ => println!("Usage: repos subcommand [argument]"),
    }
}
