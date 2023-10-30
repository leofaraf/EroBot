use std::env;
use diesel::sql_types::Integer;
use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;

pub mod models;
pub mod schema;

use self::schema::person::dsl::*;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}",
                                   database_url))
}

fn get_persons() {
    let connection = establish_connection();
}
