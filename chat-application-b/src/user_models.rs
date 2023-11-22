

pub mod user_models{
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct SignUp {
        pub email: String,
        pub mobile: String,
        pub username: String,
        pub password: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct SignIn{
        pub username: String,
        pub password: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct User{
        pub(crate) username: String
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct AddNewUser{
       pub sender: String,
       pub receiver: String,
    }


    #[derive(Serialize, Deserialize, Clone)]
    pub struct SendChats {
        pub(crate) username: String,
        pub(crate) receiver: String,
        pub message: String,
        pub timestamp: String
    }


    #[derive(Serialize, Deserialize)]
    pub struct Document {
       pub name: String,
       pub chats: Vec<Vec<Message>>,
       pub first_message: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Message {
       pub message: String,
       pub timestamp: String,
    }

}