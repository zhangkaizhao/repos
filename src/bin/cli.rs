#[macro_use]
extern crate clap;
extern crate toml;

extern crate repos;

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let app = App::new("repos")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("sync")
                .about("update or clone if a repository url provided, else sync all repositories")
                .arg(
                    Arg::with_name("url")
                        .help("repository url")
                        .index(1)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("remove directory of a local repository")
                .arg(
                    Arg::with_name("url")
                        .help("repository url")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("topics")
                .about("list all topics with count of their repositories"),
        )
        .subcommand(
            SubCommand::with_name("topic")
                .about("list repositories of a topic")
                .arg(
                    Arg::with_name("topic")
                        .help("topic")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("stats").about("output stats of all repositories"))
        .subcommand(SubCommand::with_name("cleanup").about("clean up unused resources"))
        .subcommand(
            SubCommand::with_name("search")
                .about("search local repositories by keyword")
                .arg(
                    Arg::with_name("keyword")
                        .help("keyword to search")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("proxy").about("output proxy configuration"));

    let matches = app.get_matches();

    let manager = match repos::manager::Manager::new() {
        Ok(manager) => manager,
        Err(err) => {
            println!("{}", err.to_string());
            return ();
        }
    };

    match matches.subcommand() {
        ("sync", Some(sub_matches)) => {
            if let Some(url) = sub_matches.value_of("url") {
                manager.sync(url);
            } else {
                manager.sync_all();
            }
        }
        ("remove", Some(sub_matches)) => manager.remove(sub_matches.value_of("url").unwrap()),
        ("topics", Some(_sub_matches)) => manager.topics(),
        ("topic", Some(sub_matches)) => manager.topic(sub_matches.value_of("topic").unwrap()),
        ("stats", Some(_sub_matches)) => manager.stats(),
        ("cleanup", Some(_sub_matches)) => manager.cleanup(),
        ("search", Some(sub_matches)) => manager.search(sub_matches.value_of("keyword").unwrap()),
        ("proxy", Some(_sub_matches)) => manager.proxy(),
        (_, _) => unreachable!(),
    }
}
