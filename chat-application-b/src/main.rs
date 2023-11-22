
mod user_handlers;
mod user_models;
mod user_services;
mod chat_handlers;
mod chat_services;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};

use user_handlers::user_handlers::*;

use chat_handlers::chat_handlers::* ;


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
            .route("/chat", web::get().to(websocket_handler))


    }).bind(("127.0.0.1", 8080)).unwrap().run().await
}
