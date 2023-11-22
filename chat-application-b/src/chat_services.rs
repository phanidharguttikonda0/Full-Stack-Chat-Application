

pub mod chat_services {
    use mongodb::bson::doc;
    use mongodb::{Client, Collection};
    use mongodb::options::{ClientOptions, FindOneOptions, InsertOneOptions, UpdateOptions};
    use crate::user_models::user_models::{AddNewUser, Document, SendChats};

    async fn create_connection() -> mongodb::Database {

        // Define the MongoDB connection string and database name
        let connection_string = "mongodb://localhost:27017";
        let database_name = "chat-app";

        // Parse the connection string into ClientOptions
        let client_options = ClientOptions::parse(connection_string)
            .await
            .expect("Failed to parse connection string");

        // Create a MongoDB client
        let client = Client::with_options(client_options)
            .expect("Failed to create MongoDB client");

        // Access the database
        let db = client.database("chat-app") ;

        db
    }


    pub async fn get_chats(data: AddNewUser) -> Document { // the AddNewUser takes sender and Receiver

        // we need to get the chats
        let db = create_connection().await ;
        let collection = db.collection(&data.sender) ;

        let filter = doc! { "name": &data.sender };
        let options = FindOneOptions::builder()
            // Your options here, for example:
            .projection(doc! { })
            .build();
        let result = collection.find_one(Some(filter), (options)).await.unwrap() ;

        match result {
            Some(res) => {
                res
            },
            None => {
                println!("The document doesn't found") ;
                Document {
                    chats: Vec::new(),
                    first_message: false,
                    name: String::from("")
                } // if the name of the receiver length is 0 then there is no document
            }
        }

    }


    // here we will update the chat as well as return the new chat
    pub async fn send_chat(data: SendChats) {

        // we are here updating the task

        let db = create_connection().await ;

        // first i need to get the present chat data

        let document = get_chats(AddNewUser{
            sender: String::from(&data.username),
            receiver: String::from(&data.receiver)
        }).await ;

        // now we will use the help of this document to update

        let collection: Collection<Document> = db.collection(&data.username);

        let filter = doc! {
            "name" : &data.receiver,
        } ;


        // here we are going to check whether we need to add in the previous vec or to create new vector

        /*

            if first_message {
                // first sended by the reciever
                if .len()%2 == 0 { // last messaged by the sender
            }else {
                // first sended by the sender
            }

        */

        let mut update_value = false ;
        if document.chats.len() == 0 {
             // means the first message was not registered yet
            // we need to set that the first message was false, means send by him self
            let filter_1 = doc! { "name" : &data.username } ;
            let filter_2 = doc! { "name" : &data.receiver } ;
            let update_1 = doc! { "$set" : {"first_message" : true}} ;
            let update_2 = doc! { "$set" : {"first_message" : false}} ;
            let receiver_collection: Collection<Document> = db.collection(&data.receiver) ;
            // we need to set the true for the first_message for the receiver side object
            let result_1 = receiver_collection.update_one(filter_1, update_1, None).await ;
            let resul_2 = collection.update_one(filter_2, update_2, None).await ;

            println!("Successfully Completed setting the first_message field") ;

        }

        let mut update ;


        if document.first_message { // true -> send by the receiver
            if document.chats.len()%2 != 0 {
                update = doc! {
                    "$push" : {
                        "chats" : {
                            "$each" : [
                                {
                                    "message" : &data.message ,
                                    "timestamp" : &data.timestamp
                                }
                            ]
                        }
                    }
                } ;

            }else { // just pushing the document
                update = doc! {
                    "$push": {
                        "chats.$[outer].$[inner]": {
                            "$each": [{ "message": &data.message, "timestamp": &data.timestamp }],
                            "$position": -1
                        }
                    }
                };
            }
        }else { // false -> send by the sender itself
            if document.chats.len() == 0 || document.chats.len()%2 == 0 {
                update = doc! {
                    "$push" : {
                        "chats" : {
                            "$each" : [
                                {
                                    "message" : &data.message ,
                                    "timestamp" : &data.timestamp
                                }
                            ]
                        }
                    }
                } ;
            }else{
                update = doc! {
                    "$push": {
                        "chats.$[outer].$[inner]": {
                            "$each": [{ "message": &data.message, "timestamp": &data.timestamp }],
                            "$position": -1
                        }
                    }
                };
            }
        }





        let result = collection.update_one((filter), (update), None).await.unwrap();

    }


    pub async fn create_document(data: AddNewUser) {

        // creating a document means creating the document both on the receiver end and the sender end

        let db = create_connection().await ;

        // we will create an empty document

        let collection = db.collection(&data.sender);

        let document = doc! { "name": &data.receiver, "chats": [], "first_message": false  };
        let options = InsertOneOptions::builder().build();

        let result = collection.insert_one(document, options).await.unwrap();

        println!("Completed Inserting the new document") ;

    }


    // mongo db structure of each Collection
    /*

    Collection
    [ // documents
    {
    username:      ,
    chats; [[]..],
    first_messaged: true/false
    } // document-1

    {

    } // document-2

    ]




    */

}