use rocket::{catch, serde::json::serde_json::{json, value::Value}};

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "Status": "Not Found",
        "Code": 404,
        "Reason": "Requested user was not found."
    })
}

#[catch(500)]
pub fn internal_server_error() -> Value {
    json!({
        "Status": "Internal Server Error",
        "Code": 500,
        "Reason": "Internal Server Error."
    })
}