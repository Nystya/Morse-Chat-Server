use std::sync::{Arc, Mutex};

use rocket::{post, http::Status, serde::json::Json, State};

use crate::{models::user::User, repository::user_repository::UserRepository};

#[post("/users")]
pub async fn create_user(
    idx_lock: &State<Arc<Mutex<i32>>>,
    user_service_lock: &State<Arc<Mutex<UserRepository>>>
) -> (Status, Result<Json<User>, ()>) {
    let mut user_service = user_service_lock.lock().expect("Cannot access MessageService");
    let mut idx = idx_lock.lock().unwrap();

    *idx += 1;

    let user = user_service.new_user(User {
        id: idx.to_string()
    });

    match user {
        Some(user) => (Status::Created, Ok(Json(user))),
        None => (Status::InternalServerError, Err(())),
    }
}