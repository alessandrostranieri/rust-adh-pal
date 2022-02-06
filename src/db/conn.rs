use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use super::models::*;
use super::schema::moods::dsl::*;

pub struct DbConnection{
    pub conn: SqliteConnection
}

impl DbConnection {
    pub fn get_moods(&self) -> Result<Vec<Mood>, String> {
        match moods.load::<Mood>(&self.conn) {
            Ok(ms) => Ok(ms),
            Err(_) => Err(String::from("Error retrieving moods")),
        }
    }
}

pub fn establish_connection() -> Result<DbConnection, String> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    Ok(DbConnection{conn: connection})
}
