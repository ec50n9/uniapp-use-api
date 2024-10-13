mod handler;
mod responses;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use handler::{demo_handler, openai_handler};
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(Mutex::new(demo_handler::InMemoryDb::new()));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new().wrap(cors).app_data(db.clone()).service(
            web::scope("/api/v1")
                .service(
                    web::scope("demo")
                        .service(demo_handler::hello)
                        .service(demo_handler::echo)
                        .service(demo_handler::create_post)
                        .service(demo_handler::list_posts)
                        .service(demo_handler::read_post)
                        .service(demo_handler::update_post)
                        .service(demo_handler::delete_post),
                )
                .service(web::scope("ai").service(openai_handler::openai)),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
