// initialize Database connection here.

use config::Config;
use mongodb::{Client, Collection, Database};
use once_cell::sync::OnceCell;
use std::env;

static DB: OnceCell<Database> = OnceCell::new();

    // this Database init function should be called once in main
pub async fn init_db() {
    if let Some(db) = DB.get() {
        println!("DB already initialized. Using existing database: {}", db.name());
        return;
    }
    let db_config = Config::builder()
        .add_source(config::File::with_name("settings_dev.toml").required(false))
        .build()
        .expect("Unable to read Database endpoint from file.");

    let uri = env::var("MONGODB_URI").unwrap_or_else(|_| {
        db_config
            .get_string("mongodb.uri")
            .expect("Unable to retrieve MongoURI URI from *toml file.")
    });

    let db_name = env::var("MONGODB_DATABASE").unwrap_or_else(|_| {
        db_config
            .get_string("mongodb.database")
            .expect("Unable to retrieve MongoDB database name from *toml file.")
    });

    let mut client_options = mongodb::options::ClientOptions::parse(&uri)
        .await
        .expect("Failed to parse MongoDB URI");
    client_options.app_name = Some("clinic_booking_api".to_string());
    // Direct connection can sometimes cause issues in complex environments
    // client_options.direct_connection = Some(true);

    let client = Client::with_options(client_options)
        .expect("Unable to create MongoDB client from options");

    // Ping the database to ensure connection is established
    println!("Initializing DB with URI: {} and Database: {}", uri, db_name);
    client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1})
        .await
        .expect("Failed to ping MongoDB");
    println!("DB Ping successful");

    let database = client.database(&db_name);
    let _ = DB.set(database);
}

/// Reset the DB OnceCell. ONLY FOR TESTING.
#[cfg(test)]
pub fn reset_db_for_test() {
    // There is no safe way to reset OnceCell, but for tests we can use a workaround
    // if we really needed to. However, it's better to just ensure it's initialized once.
}

// this function can be shared with other workspaces to access the MongoDB instance
pub fn get_collection<T: Send + Sync>(col_name: &str) -> Collection<T> {
    let db = DB.get().expect("DB not initialized! Call init_db first.");
    db.collection::<T>(col_name)
}
