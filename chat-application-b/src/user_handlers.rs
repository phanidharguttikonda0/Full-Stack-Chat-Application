

pub mod user_handlers {

    use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
    use actix_web::web::Json;
    use serde::de::Unexpected::Str;
    use crate::user_models::*;
    use crate::user_models::user_models::{AddNewUser, SignIn, SignUp, User};
    use crate::user_services::user_services::{add_user_to_chats, check_sign_in, check_sign_up, does_username_exists, get_user_chats, is_already_in_chatbox, sign_up_data};


    #[post("/sign-in")]
    pub async fn sign_in(req: HttpRequest, data: Json<SignIn>) -> impl Responder {

            HttpResponse::Ok().body(check_sign_in(data.clone()).await.to_string())

    }

    #[post("/sign-up")]
    pub async fn sign_up(req: HttpRequest, data: Json<SignUp>) -> impl Responder{

        let (val1) = check_sign_up(data.clone()).await ;
        if val1 {
            sign_up_data(data.clone()).await ;
            HttpResponse::Ok().body("true")
        }else {
            println!("The Reason Why Sign-Up Failed was ") ;
            HttpResponse::Ok().body("false")
        }
    }

    #[get("/users")]
    pub async fn get_users(query: web::Query<User>) -> impl Responder{

        let username = String::from(&query.username) ;
        println!("The username was {}", username) ;

        HttpResponse::Ok().json(get_user_chats(String::from(&username)).await)

    }

    #[post("/search/added")]
    pub async fn new_chat(req: HttpRequest, data: Json<AddNewUser>) -> impl Responder{

        // here we will check whether the user is already in the chat list or not

        let val = is_already_in_chatbox(String::from(&data.sender), String::from(&data.receiver)).await ;

        if val {
            // Already chatted so we will say false
            return HttpResponse::Ok().body("false") // already ur chat-box member
        }

        // a new user will be added
        add_user_to_chats(String::from(&data.sender),String::from( &data.receiver)).await;
        add_user_to_chats(String::from(&data.receiver), String::from(&data.sender)).await;
        HttpResponse::Ok().body("true")
    }


    #[post("/search")]
    pub async fn is_user_exists(req: HttpRequest, data: Json<User>) -> impl Responder{

        // if user exists returns true
        if does_username_exists(String::from(&data.username)).await {
            HttpResponse::Ok().body("true")
        }else {
            HttpResponse::Ok().body("false")
        }
        // else false

    }// if this function returns true then the front-end will goes to next-page say's add to ur chat-box



}