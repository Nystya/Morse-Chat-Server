use std::sync::{Mutex, Arc};

use rocket::{serde::json::Json, http::Status, post, State, get, debug};

use crate::{models::{message::Message, user::User}, services::message_service::{MessageService}, repository::user_repository::UserRepository};

#[post("/messages", format = "application/json", data = "<message>")]
pub fn send_message(
    message: Json<Message>,
    message_service_lock: &State<Arc<Mutex<MessageService>>>,
    user_service_lock: &State<Arc<Mutex<UserRepository>>>
) -> (Status, Result<Json<Message>, ()>) {
    let parsed_message = Json::into_inner(message);

    println!("Muye la Rust");
    println!("{:?}", parsed_message);

    let mut message_service = message_service_lock.lock().expect("Cannot access MessageService");
    let user_service = user_service_lock.lock().expect("Cannot access MessageService");

    println!("{:?}", parsed_message);

    
    let current_user: User;
    if let Some(user) = user_service.get_user(parsed_message.uid.clone()) {
        current_user = user.clone();
    } else {
        return (Status::NotFound, Err(()))
    }

    drop(user_service);

    message_service.send_message(
        current_user,
        parsed_message.clone()
    );

    (Status::Created, Ok(Json(parsed_message.clone())))
}

#[get("/messages/<uid>")]
pub async fn get_messages(
    uid: String,
    message_service_lock: &State<Arc<Mutex<MessageService>>>,
    user_service_lock: &State<Arc<Mutex<UserRepository>>>
) -> (Status, Result<Json<Vec<Message>>, ()>) {

    println!("Muye la Rust");
    let mut message_service = message_service_lock.lock().expect("Cannot access MessageService");
    let user_service = user_service_lock.lock().expect("Cannot access MessageService");
    
    match user_service.get_user(uid.clone()) {
        Some(user) => {
            match message_service.get_messages(user.id.clone()) {
                Some(messages) => (Status::Ok, Ok(Json(messages))),
                None => (Status::Ok, Ok(Json(vec![]))),
            }
        },
        None => (Status::NotFound, Err(())),
    }
}
