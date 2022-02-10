use crate::models::{
    server::{FieldsRole, FieldsServer, Role, SystemMessageChannels},
    Server,
};

impl Role {
    pub fn remove(&mut self, field: &FieldsRole) {
        match field {
            FieldsRole::Colour => self.colour = None,
        }
    }
}

impl Server {
    pub fn remove(&mut self, field: &FieldsServer) {
        match field {
            FieldsServer::Description => self.description = None,
            FieldsServer::Categories => self.categories = None,
            FieldsServer::SystemMessages => self.system_messages = None,
            FieldsServer::Icon => self.icon = None,
            FieldsServer::Banner => self.banner = None,
        }
    }
}

impl SystemMessageChannels {
    pub fn into_channel_ids(self) -> Vec<String> {
        let mut ids = vec![];

        if let Some(id) = self.user_joined {
            ids.push(id);
        }

        if let Some(id) = self.user_left {
            ids.push(id);
        }

        if let Some(id) = self.user_kicked {
            ids.push(id);
        }

        if let Some(id) = self.user_banned {
            ids.push(id);
        }

        ids
    }
}
