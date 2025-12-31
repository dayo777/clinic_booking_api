use actix_web::{App, HttpServer, Responder, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
             .route("/", web::get().to(hello))

        })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

// TODO: remove line below
async fn hello() -> impl Responder {
    "Hello, world!"
}
