// /// Initialize the Database connection pool and make available to all workspaces
//
// use config::Config;
// use serde::Deserialize;
// use mongodb::{Client, Database};
//
// #[derive(Debug, Deserialize)]
// struct DatabaseSettings {
//     mongodb: DatabaseCredentials,
// }
//
// #[derive(Debug, Deserialize)]
// struct DatabaseCredentials {
//     uri: String,
//     database: String,
// }
//
// // initialize environment variables here
// fn init_env_variables() -> DatabaseSettings {
//     let settings = Config::builder()
//         .add_source(config::File::with_name("settings_dev.toml"))
//         .add_source(config::Environment::with_prefix("APP"))
//         .build()
//         .expect("Failed to build configuration");
//
//     let db_settings: DatabaseSettings = settings
//         .try_deserialize()
//         .expect("Failed to deserialize configuration");
//
//     db_settings
//
// }
//
// pub async fn init_db() -> Database {
//     let settings = init_env_variables();
//     let client = Client::with_uri_str(settings.mongodb.uri)
//             .await
//             .expect("Failed to connect to mongodb");
//     let database = client.database(&settings.mongodb.database);
//     database
// }
