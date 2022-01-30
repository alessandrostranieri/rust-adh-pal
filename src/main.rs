#[macro_use]
extern crate diesel;
extern crate clap;

mod cmd;
mod db;

use clap::{arg, App, AppSettings};

use db::schema::mood::dsl::*;
use db::models::*;
use self::diesel::prelude::*;

fn main() {

    let dbc = db::establish_connection()
    .unwrap_or_else(|_| panic!("Error connecting to DB"));

    let results = mood
        .load::<Mood>(&dbc.conn)
        .expect("Error loading posts");

    println!("Displaying {} mood", results.len());
    for m in results {
        println!("{} | {}", m.value, m.name);
    }

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
                .arg(arg!([NUM] "Number of activities").validator(cmd::validate_num_activities)),
        )
        .subcommand(
            App::new("mood")
            .about("Record the current mood")
            .arg(arg!(<MOOD> "Clear the current activity"))
            .setting(AppSettings::ArgRequiredElseHelp),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("start", arguments)) => {
            let activity = arguments.value_of("ACTIVITY").expect("failed to extract activity");
            cmd::start(activity)
        },
        Some(("stop", _)) => cmd::stop(),
        Some(("current", arguments)) => {
            if arguments.is_present("clear") {
                cmd::clear_current()
            } else {
                cmd::current()
            }
        },
        Some(("list", arguments)) => {
            match arguments.value_of("NUM") {
                Some(num) => {
                    cmd::list_last(num.parse().unwrap())
                },
                None => cmd::list_all()
            }
        },
        Some(("mood", _)) => cmd::mood(),
        _ => {} // TODO print help
    }
}
