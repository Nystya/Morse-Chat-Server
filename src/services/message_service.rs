use std::sync::{Arc, Mutex};

use crate::{models::{message::{Message}, user::User}, repository::{message_repository::MessageRepository, user_repository::UserRepository}};

pub struct MessageService {
    message_repository: Arc<Mutex<MessageRepository>>,
    user_repository: Arc<Mutex<UserRepository>>
}

impl MessageService {
    pub fn new(
        message_repository: Arc<Mutex<MessageRepository>>,
        user_repository: Arc<Mutex<UserRepository>>
    ) -> Self { 
        Self { 
            message_repository,
            user_repository
        } 
    }

    pub fn send_message(&mut self, sender: User, message: Message) -> Result<(), ()> {
        let mut user_repository = self.user_repository.lock().expect("Cannot get lock for user_repository");
        let mut message_repository = self.message_repository.lock().expect("Cannot get lock for user_repository");

        match user_repository.get_users() {
            Some(users) => {
                for user in users.iter() {
                    if user.id == sender.id {
                        continue;
                    }
                    
                    message_repository.save_message(user.clone(), message.clone());
                }
                Ok(())
            },
            None => Ok(())
        }
    }

    pub fn get_messages(&mut self, uid: String) -> Option<Vec<Message>> {
        let mut message_repository = self.message_repository.lock().expect("Cannot get lock for user_repository");

        let messages = match message_repository.get_messages(User { id: uid }) {
            Some(messages) => Some(messages),
            None => None,
        };

        println!("Valoare {:?}", messages);

        messages
    }
}