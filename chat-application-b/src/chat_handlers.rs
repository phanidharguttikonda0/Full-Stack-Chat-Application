


pub mod chat_handlers{
    use std::alloc::System;
    use std::error::Error;
    use actix::{Actor, Addr, Context, StreamHandler};
    use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
    use actix_web_actors::ws;
    use actix_web_actors::ws::{Message, ProtocolError};
    use futures_util::FutureExt;
    use crate::chat_services::chat_services::{get_chats, send_chat};
    use crate::user_models::user_models::{AddNewUser, Document, SendChats, User};

    #[derive(Debug)]
    struct MyWebSocket;

    impl Actor for MyWebSocket {
        type Context = ws::WebsocketContext<Self>;

        fn started(&mut self, ctx: &mut Self::Context) {
            println!("Connection Established") ;

            ctx.text("Connection Established from the server side ") ;
        }
    }

    impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
        fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {

            match msg {
                Ok(ws::Message::Text(text)) => {

                    let message: SendChats = serde_json::from_str(&text).unwrap();

                    if message.message.len() == 0 {
                        // means we need to get the messages

                        let result = actix_rt::System::new().block_on(
                            get_chats(AddNewUser{
                                sender: String::from(&message.username),
                                receiver: String::from(&message.receiver)
                            })
                        ) ;
                        ctx.text(serde_json::to_string(&result).unwrap() ) ; // i think we need parse this in to a string
                    }else {
                        // means we need to send message and pass back the new chat list
                        actix_rt::System::new().block_on(send_chat(message.clone())) ;
                        let result = actix_rt::System::new().block_on(
                            get_chats(AddNewUser{
                                sender: String::from(&message.username),
                                receiver: String::from(&message.receiver)
                            })
                        ) ;

                        ctx.text(serde_json::to_string(&result).unwrap()) ;
                    }

                }
                _ => {}
            }

        }
    }

    pub async  fn websocket_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Box<dyn Error>> {
        let resp = ws::start(MyWebSocket {}, &req, stream).unwrap();
        Ok(resp)
    }


}