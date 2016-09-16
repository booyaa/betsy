extern crate happv;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;
extern crate dotenv;
extern crate ini;

use clap::{App, SubCommand};
use happv::AppVeyor;
use dotenv::dotenv;
use std::env;
use ini::Ini;

const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const APP_ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    dotenv().ok();
    env_logger::init().unwrap();

    let app = App::new(APP_NAME)
                  .version(APP_VERSION)
                  .author(APP_AUTHOR)
                  .about(APP_ABOUT)
                  .subcommand(SubCommand::with_name("enable").about("Enable a project"))
                  .subcommand(SubCommand::with_name("disable").about("Disable a project"))
                  .subcommand(SubCommand::with_name("list").about("Lists projects"));

    let matches = app.get_matches(); //doesn't work _from_safe_borrow();

    let token = env::var("APPVEYOR").expect("APPVEYOR env variable was not found!");

    if matches.is_present("enable") {
        enable(&token);
    } else if matches.is_present("disable") {
        disable(&token);
    } else if matches.is_present("list") {
        list(&token);
    } else {
        println!("{}", matches.usage());
        // moved app.print_help();
    }
}

fn list(token: &str) {
    let happv = AppVeyor::new(&token);
    let result = happv.get_projects();

    if result.is_ok() {
        println!("Projects (slug names):");
        for i in result.unwrap() {
            println!("\t{}", i.slug);
        }
    } else {
        println!("Error retrieving projects!");
        std::process::exit(1);
    }
}

fn enable(token: &str) {
    let repository_name = get_repository_name();
    debug!("repository: {:?}", repository_name);

    let happv = AppVeyor::new(&token);
    let result = happv.add_project("gitHub".to_string(), repository_name);

    if result.is_ok() {
        println!("Project has been enabled!");

    } else {
        println!("Error enabling project!");
        std::process::exit(1);
    }
}

fn disable(token: &str) {
    let repository_name = get_repository_name();
    println!("repository: {:?}", repository_name);

    let happv = AppVeyor::new(&token);
    let result = happv.get_projects();

    let mut account_name = String::new();
    let mut project_slug = String::new();

    if result.is_ok() {
        for i in result.unwrap() {
            let repository_match = format!("{}/{}", i.account_name, i.slug);
            debug!("{} == {}", repository_match, repository_name);
            if repository_name == repository_match {
                account_name = i.account_name;
                project_slug = i.slug;
                break;
            }
        }
        if account_name.is_empty() || project_slug.is_empty() {
            println!("Couldn't find project on AppVeyor!");
            std::process::exit(1);
        }

        let result = happv.delete_project(account_name, project_slug);
        if result.is_ok() {
            println!("Project disabled!");
        } else {
            println!("Failed to disable project!");
            std::process::exit(1);
        }
    } else {
        println!("Error retrieving project list from AppVeyor!");
        std::process::exit(1);
    }
}


fn get_repository_name() -> String {
    let conf = Ini::load_from_file(".git/config").expect("Failed to open .git/config");

    let section = conf.section(Some("remote \"origin\""));

    if section.is_none() {
        println!("Failed to find remote in git config!");
        std::process::exit(1);
    }

    let repository_url = section.unwrap().get("url");

    if repository_url.is_none() {
        println!("Not git repository!");
        std::process::exit(1);
    }

    let repository_info: Vec<_> = repository_url.unwrap().split(':').collect();

    // 0 - git@github.com
    // 1 - booyaa/betsy.git
    if !repository_info[0].contains("github") {
        println!("Only GitHub supported at the moment!");
        std::process::exit(1);
    }
    debug!("repo_info[1]: {:#?}", repository_info[1]);
    let repository_name = repository_info[1].split(".").nth(0);

    if repository_name.is_none() {
        println!("Failed to extract repository name: {}", repository_info[1]);
        std::process::exit(1);
    }

    repository_name.unwrap().to_string()
}
