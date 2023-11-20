

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

    #[derive(Serialize, Deserialize)]
    pub struct AddNewUser{
       pub sender: String,
       pub receiver: String,
    }

}