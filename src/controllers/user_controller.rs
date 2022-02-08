use std::sync::{Arc, Mutex};

use rocket::{post, http::Status, serde::json::Json, State};
use uuid::Uuid;

use crate::{models::user::User, repository::user_repository::UserRepository};

#[post("/users")]
pub async fn create_user(
    user_service_lock: &State<Arc<Mutex<UserRepository>>>
) -> (Status, Result<Json<User>, ()>) {
    let mut user_service = user_service_lock.lock().expect("Cannot access MessageService");

    let uuid = Uuid::new_v4();

    let user = user_service.new_user(User {
        id: uuid.to_string()
    });

    match user {
        Some(user) => (Status::Created, Ok(Json(user))),
        None => (Status::InternalServerError, Err(())),
    }
}