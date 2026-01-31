// initialize Database connection here.

use config::Config;
use mongodb::{Client, Collection, Database};
use once_cell::sync::OnceCell;

static DB: OnceCell<Database> = OnceCell::new();

// this Database init function should be called once in main
pub async fn init_db() {
    let db_config = Config::builder()
        .add_source(config::File::with_name("settings_dev.toml"))
        .build()
        .expect("Unable to read Database endpoint from file.");

    let uri = db_config
        .get_string("mongodb.uri")
        .expect("Unable to retrieve MongoURI URI from database.");

    let db_name = db_config
        .get_string("mongodb.database")
        .expect("Unable to retrieve MongoDB database name from database.");

    let client = Client::with_uri_str(&uri)
        .await
        .expect("Unable to connect to MongoDB client");

    let database = client.database(&db_name);
    DB.set(database).expect("DB already initialized.");
}

// this function can be shared with other workspaces to access the MongoDB instance
pub fn get_collection<T: Send + Sync>(col_name: &str) -> Collection<T> {
    let db = DB.get().expect("DB not initialized! Call init_db first.");
    db.collection::<T>(col_name)
}
