use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::value::Value;
use rocket::{debug, catch, get, launch, post, routes, catchers, State};
use rocket::http::Status;
use rocket::serde::json::{Json};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
struct User {
    id: usize,
    username: String,
    password: Option<String>,
}

type UserMap = Mutex<HashMap<usize, String>>;

#[get("/")]
async fn index() -> &'static str {
    "This is the base route!"
}

#[get("/")]
async fn hidden_index() -> &'static str {
    "This is the /hidden base route!"
}

#[get("/hello/<name>", rank = 2)]
fn hello(name: String) -> String {
    format!("Hello, user {}!", name)
}

#[get("/hello/<id>")]
fn hello_id(id: u32) -> String {
    format!("Hello, {}!", id)
}

#[post("/user", format = "application/json", data = "<user>")]
fn new_user(user: Json<User>, map: &State<UserMap>) -> (Status, Result<Json<User>, ()>) {
    let mut hashmap = map.lock().expect("Map Locked");
    if hashmap.contains_key(&user.id) {
        debug!("Aici\n");
        (Status::Unauthorized, Ok(user))
    } else {
        hashmap.insert(user.id, String::from(&user.username));
        (Status::Ok, Ok(user))
    }
}

#[get("/user/<id>")]
fn get_user(id: usize, map: &State<UserMap>) -> Option<Json<User>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|name| {
        Json(User {
            id,
            username: String::from(name),
            password: None,
        })
    })
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "Status": "Error",
        "Reason": "Requested user was not found."
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello, hello_id, new_user, get_user])
        .mount("/hidden", routes![hidden_index])
        .register("/", catchers![not_found])
        .manage(Mutex::new(HashMap::<usize, String>::new()))
}