// initialize Database connection here.

use config::Config;
use mongodb::{Client, Collection, Database};
use once_cell::sync::OnceCell;
use std::env;

use std::sync::RwLock;

static DB: OnceCell<Database> = OnceCell::new();
static TEST_DB_LOCK: OnceCell<RwLock<Option<Database>>> = OnceCell::new();

fn get_test_db_lock() -> &'static RwLock<Option<Database>> {
    TEST_DB_LOCK.get_or_init(|| RwLock::new(None))
}

// this Database init function should be called once in main
pub async fn init_db() {
    let test_lock = get_test_db_lock();
    let db = {
        let read_guard = test_lock.read().unwrap();
        read_guard.clone()
    };
    if let Some(db) = db {
        // Check if connection is still alive
        if db
            .run_command(mongodb::bson::doc! {"ping": 1})
            .await
            .is_ok()
        {
            return;
        }
    }

    if DB.get().is_some()
        && DB
            .get()
            .unwrap()
            .run_command(mongodb::bson::doc! {"ping": 1})
            .await
            .is_ok()
    {
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

    let client =
        Client::with_options(client_options).expect("Unable to create MongoDB client from options");

    // Ping the database to ensure connection is established
    println!(
        "Initializing DB with URI: {} and Database: {}",
        uri, db_name
    );
    client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1})
        .await
        .expect("Failed to ping MongoDB");
    println!("DB Ping successful");

    let database = client.database(&db_name);

    // In test mode, we store the DB in the lock.
    {
        let mut write_guard = test_lock.write().unwrap();
        *write_guard = Some(database.clone());
    }

    // We also set the global DB if not already set.
    if DB.get().is_none() {
        let _ = DB.set(database.clone());
    }
}

/// Reset the DB. ONLY FOR TESTING.
pub fn reset_db_for_test() {
    #[cfg(test)]
    {
        let lock = get_test_db_lock();
        let mut write_guard = lock.write().unwrap();
        *write_guard = None;
    }
}

// this function can be shared with other workspaces to access the MongoDB instance
pub fn get_collection<T: Send + Sync>(col_name: &str) -> Collection<T> {
    let lock = get_test_db_lock();
    let guard = lock.read().unwrap();
    if let Some(db) = &*guard {
        return db.collection::<T>(col_name);
    }

    let db = DB.get().expect("DB not initialized! Call init_db first.");
    db.collection::<T>(col_name)
}
