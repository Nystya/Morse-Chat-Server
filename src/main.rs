pub mod loaders;
pub mod controllers;
mod models;
mod services;
mod repository;

use std::sync::{Mutex, Arc};

use loaders::catchers;
use repository::{message_repository::{MessageRepository}, user_repository::{UserRepository}};
use rocket::{launch, routes, catchers};
use services::message_service::MessageService;

#[launch]
fn rocket() -> _ {
    let safe_message_repository = Arc::new(Mutex::new(MessageRepository::new()));
    let safe_user_repository = Arc::new(Mutex::new(UserRepository::new()));
    let safe_message_service = Arc::new(Mutex::new(MessageService::new(safe_message_repository.clone(), safe_user_repository.clone())));
    let safe_user_service = safe_user_repository.clone();

    let idx: i32 = 0;
    let safe_idx = Arc::new(Mutex::new(idx));

    rocket::build()
        .mount("/", routes![
            controllers::message_controller::send_message,
            controllers::message_controller::get_messages,
            controllers::user_controller::create_user,
        ])
        .register("/", catchers![
            catchers::not_found,
            catchers::internal_server_error
        ])
        .manage(safe_message_service)
        .manage(safe_user_service)
        .manage(safe_idx)
}