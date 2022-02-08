use std::{sync::Mutex, collections::{HashMap, VecDeque}};

use crate::models::{message::Message, user::User};

pub struct MessageRepository{
    database: Mutex<HashMap<String, VecDeque<Message>>>
}

impl MessageRepository {
    pub fn new() -> Self { 
        Self { 
            database: Mutex::new(HashMap::<String, VecDeque<Message>>::new())
        }
    }

    pub fn save_message(&mut self, user: User,  message: Message) -> Option<()> {
        let mut db = self.database.lock().expect("Cannot access database");

        db.entry(user.id).or_insert(VecDeque::new()).push_back(message);

        Some(())
    }

    pub fn get_messages(&mut self, user: User) -> Option<Vec<Message>> {
        let mut db = self.database.lock().expect("Cannot access database");
        
        db.get_mut(&user.id).map(|mq| {
            if let Some(message) = mq.front() {
                println!("{:?}", message);
            }

            let messages = mq.drain(..).collect::<Vec<Message>>();

            println!("{:?}", messages);
            messages
        })
    }
}
