extern crate happv;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;
extern crate dotenv;

use clap::{App, SubCommand};
use happv::AppVeyor;
use dotenv::dotenv;
use std::env;

const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const APP_ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");


fn main() {
    dotenv().ok();
    env_logger::init().unwrap();

    let matches = App::new(APP_NAME)
                      .version(APP_VERSION)
                      .author(APP_AUTHOR)
                      .about(APP_ABOUT)
                      .subcommand(SubCommand::with_name("enable").about("Enable a project"))
                      .subcommand(SubCommand::with_name("disable").about("Disable a project"))
                      .subcommand(SubCommand::with_name("list").about("Lists projects"))
                      .get_matches();

    let token = env::var("APPVEYOR").expect("APPVEYOR env variable was not found!");

    if matches.is_present("enable") {
        println!("enable project");
    } else if matches.is_present("disable") {
        println!("disable project");
    } else if matches.is_present("list") {
        list(&token);
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
    }
}
