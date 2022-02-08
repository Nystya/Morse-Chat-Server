use std::{sync::Mutex, collections::HashMap};

use crate::models::user::User;

pub struct UserRepository {
    database: Mutex<HashMap<String, User>>
}
    
impl UserRepository {
    pub fn new() -> Self {
        Self { 
            database: Mutex::new(HashMap::<String, User>::new())
        }
    }
    
    pub fn new_user(&mut self, user: User) -> Option<User> {
        let mut db = self.database.lock().expect("Cannot access database");

        let newUser = db.entry(user.id.clone()).or_insert(user);

        Some(User {
            id: newUser.id.clone(),
        })
    }

    pub fn get_users(&mut self) -> Option<Vec<User>> {
        let db = self.database.lock().expect("Cannot access database");

        let mut users = Vec::new();
        for key in db.keys() {
            let user = match db.get(key) {
                Some(user) => User{id: user.id.clone()},
                None => panic!("Cannot find user for existing key"),
            };

            users.push(user);
        }

        Some(users)
    }

    pub fn get_user(&self, uid: String) -> Option<User> {
        let db = self.database.lock().expect("Cannot access database");

        match db.get(&uid) {
            Some(user) => Some(User{id: user.id.clone()}),
            None => None,
        }   
    }
}