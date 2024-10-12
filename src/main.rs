use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
mod handler;
use handler::{openai_handler, demo_handler};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(Mutex::new(demo_handler::InMemoryDb::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())

            .service(demo_handler::hello)
            .service(demo_handler::echo)

            .service(demo_handler::create_post)
            .service(demo_handler::list_posts)
            .service(demo_handler::read_post)
            .service(demo_handler::update_post)
            .service(demo_handler::delete_post)

            .service(openai_handler::openai)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
