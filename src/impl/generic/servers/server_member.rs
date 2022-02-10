use crate::models::{server_member::FieldsMember, Member};

impl Member {
    pub fn remove(&mut self, field: &FieldsMember) {
        match field {
            FieldsMember::Avatar => self.avatar = None,
            FieldsMember::Nickname => self.nickname = None,
            FieldsMember::Roles => self.roles = None,
        }
    }
}
