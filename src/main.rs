extern crate clap;

use clap::{App, SubCommand};

mod cmd;

fn main() {
    let matches = App::new("My ADHD Pal")
        .version("0.1.0")
        .author("Alessandro Stranieri <alessandro.stranieri@gmail.com>")
        .about("Your pal against ADHD")
        .subcommand(SubCommand::with_name("start"))
        .subcommand(SubCommand::with_name("stop"))
        .subcommand(SubCommand::with_name("current"))
        .subcommand(SubCommand::with_name("list"))
        .subcommand(SubCommand::with_name("mood"))
        .get_matches();

    match matches.subcommand() {
        ("stop", Some(_)) => cmd::stop(),
        ("start", Some(_)) => cmd::start(),
        ("current", Some(_)) => cmd::current(),
        ("list", Some(_)) => cmd::list(),
        ("mood", Some(_)) => cmd::mood(),
        _ => {} // TODO print help
    }
}
