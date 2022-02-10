use nanoid::nanoid;

use crate::models::{bot::FieldsBot, Bot};

impl Bot {
    pub fn remove(&mut self, field: &FieldsBot) {
        match field {
            FieldsBot::Token => self.token = nanoid!(64),
            FieldsBot::InteractionsURL => {
                self.interactions_url.take();
            }
        }
    }
}
