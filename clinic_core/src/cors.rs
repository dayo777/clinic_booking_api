// CORS config for Front-end access
use actix_cors::Cors;
// use config::Config;
// use std::env;

pub(crate) fn frontend_allowed_cors_origins() -> Cors {
    // TODO: work on the CORS later
    // let config = Config::builder()
    //     .add_source(config::File::with_name("settings_dev.toml").required(false))
    //     .build()
    //     .expect("Unable to retrieve CORS value for Front-end");
    //
    // let allowed_origin = env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| {
    //     config
    //         .get_string("server.ALLOWED_ORIGIN")
    //         .expect("Unable to retrieve allowed-origins from *toml file.")
    // });
    //
    // let cors = Cors::default()
    //     .allowed_origin(allowed_origin.as_str())
    //     // Add other trusted frontend origins here:
    //     // .allowed_origin("https://your-frontend.example.com")
    //     .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
    //     .allowed_headers(vec!["Content-Type", "Authorization", "x-api-version"])
    //     .expose_headers(vec!["Content-Length"])
    //     .max_age(3600);

    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}
