use crate::db::conn::establish_connection;
use crate::db::models::Mood;


pub fn mood() {
    let dbc = establish_connection()
    .unwrap_or_else(|_| panic!("Error connecting to DB"));

    match dbc.get_moods() {
        Ok(moods) => self::print_moods(&moods),
        Err(e) => println!("Error retrieving moods: {}", e),
    }
}

fn print_moods(moods: &[Mood]) {
    match moods.len(){
        0 => println!("No moods found"),
        _ => {
            for mood in moods {
                println!("{} | {}", mood.name, mood.value)
            }
        }
    }
}
