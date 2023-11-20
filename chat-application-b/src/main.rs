
mod user_handlers;
mod user_models;
mod user_services;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

use user_handlers::user_handlers::*;


#[actix::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_header().allow_any_origin().allow_any_method()
                .max_age(3600))
            .service(sign_up)
            .service(sign_in)
            .service(get_users)
            .service(is_user_exists)
            .service(new_chat)


    }).bind(("127.0.0.1", 8080)).unwrap().run().await
}
