

pub mod user_services {
    use std::fmt::{Debug, format};
    use std::sync::mpsc::Receiver;
    use serde::de::value;
    use sqlx::{Pool, Postgres, query, query_as, Row};
    use sqlx::postgres::PgPoolOptions;
    use crate::user_models ;
    use crate::user_models::user_models::{SignIn, SignUp};

    pub async fn create_connection(database_name: String) -> Pool<Postgres> {
        let database_url = format!("postgres://postgres:Phani@9090K@localhost/{}", database_name);

        PgPoolOptions::new()
            .max_connections(5) // Set your desired maximum number of connections
            .connect(&database_url)
            .await.unwrap()
    }


    // we will return true if nothing is not there in data-base expect password
    pub async fn check_sign_up(data: SignUp) -> bool { // reason

        let con = create_connection(String::from("chatapp")).await ;

        let result = sqlx::query("SELECT EXISTS (SELECT 1 FROM user_credentials WHERE username = $1)")
            .bind(&data.username)
            .fetch_optional(&con)
            .await .unwrap();


        match result {
            Some(res) => {

                let result1 = sqlx::query("SELECT EXISTS (SELECT 1 FROM user_credentials WHERE email = $1)")
                    .bind(&data.email)
                    .fetch_optional(&con)
                    .await.unwrap() ;

                match result1 {
                    Some(res) => {

                        let result2 = sqlx::query("SELECT EXISTS (SELECT 1 FROM user_credentials WHERE mobilenumber = $1)")
                            .bind(&data.mobile)
                            .fetch_optional(&con)
                            .await.unwrap() ;
                        con.close().await ;

                        match result2 {
                            Some(res) => {
                                true
                            },
                            None => {
                                false
                            }
                        }
                    },
                    None => {
                        false
                    }
                }

            },
            None => {
                false
            }
        }

    }


    // if both username and password are correct then we will return true
    pub async fn check_sign_in(data: SignIn) -> (bool) {

        println!("The sign-in check was started") ;

        let con = create_connection(String::from("chatapp")).await ;

        let result = sqlx::query("SELECT EXISTS (SELECT 1 FROM user_credentials WHERE username = $1)")
            .bind(&data.username)
            .fetch_optional(&con)
            .await .unwrap();


        match result {

            Some(res) => {

               let result:String = sqlx::query("SELECT PASSWORD FROM USER_CREDENTIALS WHERE \
               USERNAME = $1")
                   .bind(&data.username)
                   .fetch_one(&con)
                   .await.unwrap().get("password") ;

                println!("The Password was {}", result) ;
                con.close().await ;

                if result == String::from(&data.password) {
                    return true
                }else {
                    return false
                }


            },
            None => false

        }

    }


    // we will post sing_up details in the database
    pub async fn sign_up_data(data: SignUp) {

        let con = create_connection(String::from("chatapp")).await ;


        // the below returning clause is used for returning the inserted row
        let result = sqlx::query( "INSERT INTO user_credentials (email, mobilenumber, username, password)
         VALUES ($1, $2, $3, $4)")
            .bind(&data.email)
            .bind(&data.mobile)
            .bind(&data.username)
            .bind(&data.password)
            .execute(&con)
            .await ;

        match result {
            Ok(value) => {
                println!("Sucessfully posted") ;
                // now we are going to create the table for the users which keeps track the users chatted with
                create_users_table(String::from(&data.username)).await;
            }
            Err(err) => {
                println!("Error Ocurred, The Error was {}", err) ;
            }
        }
        con.close().await ;

    }


    // returns list of all usernames chatted with
    pub async fn get_user_chats(username: String) -> Vec<String> {

        let con = create_connection(String::from("chatapp")).await ;

        // we are going to get the user chatted members

        let rows = sqlx::query(&format!("SELECT * FROM {}",&username))
            .fetch_all(&con)
            .await.unwrap() ;

        let mut values = vec![] ;

        for row in rows {
            values.push(row.get("username")) ;
        }
        con.close().await ;

        values
    }

    pub async fn create_users_table(username: String) {

        let con = create_connection(String::from("chatapp")).await ;

        let value = sqlx::query(&format!("create table {} (username varchar(50)  UNIQUE NOT NULL )", &username))
            .execute(&con).await ;

        match value {
            Ok(_) => println!("Sucessfully created the users table"),
            Err(err) => {
                println!("Failed to create the table due to {}", err) ;
            }
        }
        con.close().await ;
    }


    pub async fn add_user_to_chats(username: String, receiver: String) -> bool {

        let con = create_connection(String::from("chatapp")).await ;


        // the below returning clause is used for returning the inserted row
        let result = sqlx::query(&format!("insert into {} (username) Values ($1)", &username))
            .bind(&receiver)
            .execute(&con).await ;
        con.close().await ;
        match result {
            Ok(val) => {
                println!("Successfully added the user ") ;
                true
            },
            Err(err) => {
                println!("Error Occured due to {} ",err) ;
                false
            }
        }

    }

    pub async fn does_username_exists(username: String) -> bool {

        let con = create_connection(String::from("chatapp")).await ;

        let result = sqlx::query("SELECT EXISTS (SELECT 1 FROM user_credentials WHERE username = $1)")
            .bind(&username)
            .fetch_optional(&con)
            .await.unwrap() ;

        con.close().await ;
        match result {
            Some(res)=> true,
            None => false
        }

    }

    pub async fn is_already_in_chatbox(username: String, receiver: String) -> bool {

        let con = create_connection(String::from("chatapp")).await ;

        let result = sqlx::query(&format!("SELECT * FROM {} where username = $1", &username))
            .bind(&receiver)
            .fetch_one(&con)
            .await ;

        match result {
            Ok(res) => {

                let user:String = res.get("username") ;
                println!("The user was {}", user) ;

                true

            }
            Err(err) => {
                false
            }
        }

    }

}