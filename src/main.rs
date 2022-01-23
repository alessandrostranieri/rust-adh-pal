extern crate clap;

use clap::{arg, App, AppSettings};

mod cmd;

fn main() {
    let matches = App::new("adh-pal")
        .version("0.1.0")
        .author("Alessandro Stranieri <alessandro.stranieri@gmail.com>")
        .about("Your pal against ADHD")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("start")
                .about("Start an activity")
                .arg(arg!(<ACTIVITY> "The activity to start"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(App::new("stop").about("Start an activity"))
        .subcommand(
            App::new("current")
                .about("Display the current activity")
                .arg(
                    arg!(-c --clear "Clear the current activity.")
                ),
        )
        .subcommand(
            App::new("list")
                .about("List all completed activities")
                .arg(arg!(<NUM> "Number of activities")),
        )
        .subcommand(
            App::new("mood")
            .about("Record the current mood")
            .arg(arg!(<MOOD> "Clear the current activity"))
            .setting(AppSettings::ArgRequiredElseHelp),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("start", _)) => cmd::start(),
        Some(("stop", _)) => cmd::stop(),
        Some(("current", _)) => cmd::current(),
        Some(("list", _)) => cmd::list(),
        Some(("mood", _)) => cmd::mood(),
        _ => {} // TODO print help
    }
}
