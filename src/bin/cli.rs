extern crate toml;

extern crate repos;

use std::env;

fn main() {
    let manager = match repos::manager::Manager::new() {
        Ok(manager) => manager,
        Err(err) => {
            println!("{}", err.to_string());
            return ();
        }
    };
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let subcommand = &args[1];
            match &subcommand[..] {
                "sync" => manager.sync_all(),
                "topics" => manager.topics(),
                "stats" => manager.stats(),
                "cleanup" => manager.cleanup(),
                "proxy" => manager.proxy(),
                _ => unimplemented!(),
            }
        }
        3 => {
            let subcommand = &args[1];
            let argument = &args[2];
            match &subcommand[..] {
                "sync" => manager.sync(&argument),
                "remove" => manager.remove(&argument),
                "topic" => manager.topic(&argument),
                "search" => manager.search(&argument),
                _ => unimplemented!(),
            }
        }
        _ => println!("Usage: repos subcommand [argument]"),
    }
}
