use std::collections::HashSet;

use crate::{
    get_relationship,
    models::{server_member::FieldsMember, user::RelationshipStatus, Channel, Member, User},
    perms, Database, Result,
};

use super::{
    client::EventV1,
    state::{Cache, State},
};

impl Cache {
    pub async fn can_view_channel(&self, db: &Database, channel: &Channel) -> bool {
        match &channel {
            Channel::TextChannel { server, .. } | Channel::VoiceChannel { server, .. } => {
                let member = self
                    .members
                    .iter()
                    .map(|(_, x)| x)
                    .find(|x| &x.id.server == server);

                let server = self
                    .servers
                    .iter()
                    .map(|(_, x)| x)
                    .find(|x| &x.id == server);

                perms(self.users.get(&self.user_id).unwrap())
                    .channel(channel)
                    .member_opt(member)
                    .server_opt(server)
                    .calc_channel(db)
                    .await
                    .get_view()
            }
            _ => true,
        }
    }

    pub async fn filter_accessible_channels(
        &self,
        db: &Database,
        channels: Vec<Channel>,
    ) -> Vec<Channel> {
        let mut viewable_channels = vec![];
        for channel in channels {
            if self.can_view_channel(db, &channel).await {
                viewable_channels.push(channel);
            }
        }

        viewable_channels
    }

    pub fn can_subscribe_to_user(&self, user_id: &str) -> bool {
        if let Some(user) = self.users.get(&self.user_id) {
            match get_relationship(user, user_id) {
                RelationshipStatus::Friend
                | RelationshipStatus::Incoming
                | RelationshipStatus::Outgoing
                | RelationshipStatus::User => true,
                _ => {
                    let user_id = &user_id.to_string();
                    for channel in self.channels.values() {
                        match channel {
                            Channel::DirectMessage { recipients, .. }
                            | Channel::Group { recipients, .. } => {
                                if recipients.contains(user_id) {
                                    return true;
                                }
                            }
                            _ => {}
                        }
                    }

                    false
                }
            }
        } else {
            false
        }
    }
}

impl State {
    pub async fn generate_ready_payload(&mut self, db: &Database) -> Result<EventV1> {
        let user = self.clone_user();

        // Find all relationships to the user.
        let mut user_ids: Vec<String> = user
            .relations
            .as_ref()
            .map(|arr| arr.iter().map(|x| x.id.to_string()).collect())
            .unwrap_or_default();

        // Fetch all memberships with their corresponding servers.
        let members: Vec<Member> = db.fetch_all_memberships(&user.id).await?;
        let server_ids: Vec<String> = members.iter().map(|x| x.id.server.clone()).collect();
        let servers = db.fetch_servers(&server_ids).await?;

        // Collect channel ids from servers.
        let mut channel_ids = vec![];
        for server in &servers {
            channel_ids.append(&mut server.channels.clone());
        }

        // Fetch DMs and server channels.
        let mut channels = db.find_direct_messages(&user.id).await?;
        channels.append(&mut db.fetch_channels(&channel_ids).await?);

        // Append known user IDs from DMs.
        for channel in &channels {
            match channel {
                Channel::DirectMessage { recipients, .. } | Channel::Group { recipients, .. } => {
                    user_ids.append(&mut recipients.clone());
                }
                _ => {}
            }
        }

        // Fetch user data.
        let users = db
            .fetch_users(
                &user_ids
                    .into_iter()
                    .filter(|x| x != &user.id)
                    .collect::<Vec<String>>(),
            )
            .await?;

        // Copy data into local state cache.
        self.cache.users = users.iter().cloned().map(|x| (x.id.clone(), x)).collect();
        self.cache
            .users
            .insert(self.cache.user_id.clone(), user.clone());
        self.cache.servers = servers.iter().cloned().map(|x| (x.id.clone(), x)).collect();
        self.cache.channels = channels
            .iter()
            .cloned()
            .map(|x| (x.id().to_string(), x))
            .collect();
        self.cache.members = members
            .iter()
            .cloned()
            .map(|x| (x.id.server.clone(), x))
            .collect();

        // Filter server channels by permission.
        let channels = self.cache.filter_accessible_channels(db, channels).await;

        // Make all users appear from our perspective.
        let mut users: Vec<User> = users
            .into_iter()
            .map(|x| x.with_relationship(&user))
            .collect();

        users.push(user.foreign());

        // Set subscription state internally.
        self.reset_state();

        for user in &users {
            self.insert_subscription(user.id.clone());
        }

        for server in &servers {
            self.insert_subscription(server.id.clone());
        }

        for channel in &channels {
            self.insert_subscription(channel.id().to_string());
        }

        Ok(EventV1::Ready {
            users,
            servers,
            channels,
            members,
        })
    }

    pub async fn recalculate_server(&mut self, db: &Database, id: &str) {
        if let Some(server) = self.cache.servers.get(id) {
            let mut channel_ids = HashSet::new();
            let mut added_channels = vec![];
            let mut removed_channels = vec![];

            let id = &id.to_string();
            for (channel_id, channel) in &self.cache.channels {
                match channel {
                    Channel::TextChannel { server, .. } | Channel::VoiceChannel { server, .. } => {
                        if server == id {
                            channel_ids.insert(channel_id.clone());

                            if self.cache.can_view_channel(db, channel).await {
                                added_channels.push(channel_id.clone());
                            } else {
                                removed_channels.push(channel_id.clone());
                            }
                        }
                    }
                    _ => {}
                }
            }

            let known_ids = server.channels.iter().cloned().collect::<HashSet<String>>();

            for id in added_channels {
                self.insert_subscription(id);
            }

            for id in removed_channels {
                self.remove_subscription(&id);
            }

            // * NOTE: currently all channels should be cached
            // * provided that a server was loaded from payload
            let unknowns = known_ids
                .difference(&channel_ids)
                .cloned()
                .collect::<Vec<String>>();

            if !unknowns.is_empty() {
                if let Ok(channels) = db.fetch_channels(&unknowns).await {
                    // ! FIXME: unnecessary clone
                    for channel in &channels {
                        self.cache
                            .channels
                            .insert(channel.id().to_string(), channel.clone());
                    }

                    let viewable_channels =
                        self.cache.filter_accessible_channels(db, channels).await;

                    for channel in viewable_channels {
                        self.insert_subscription(channel.id().to_string());
                    }
                }
            }
        }
    }

    pub async fn handle_incoming_event_v1(&mut self, db: &Database, event: &mut EventV1) {
        match event {
            EventV1::ChannelCreate(channel) => {
                let id = channel.id().to_string();
                self.insert_subscription(id.clone());
                self.cache.channels.insert(id, channel.clone());
            }
            EventV1::ChannelUpdate {
                id, data, clear, ..
            } => {
                if let Some(channel) = self.cache.channels.get_mut(id) {
                    for field in clear {
                        channel.remove(field);
                    }

                    channel.apply_options(data.clone());
                }

                if let Some(channel) = self.cache.channels.get(id) {
                    if !self.cache.can_view_channel(db, channel).await {
                        *event = EventV1::ChannelDelete { id: id.clone() };
                    }
                }
            }
            EventV1::ChannelDelete { id } => {
                self.remove_subscription(id);
                self.cache.channels.remove(id);
            }
            EventV1::ChannelGroupJoin { user, .. } => {
                self.insert_subscription(user.clone());
            }
            EventV1::ChannelGroupLeave { user, .. } => {
                if !self.cache.can_subscribe_to_user(user) {
                    self.remove_subscription(user);
                }
            }

            EventV1::ServerUpdate {
                id, data, clear, ..
            } => {
                if let Some(server) = self.cache.servers.get_mut(id) {
                    for field in clear {
                        server.remove(field);
                    }

                    server.apply_options(data.clone());
                }

                if data.default_permissions.is_some() {
                    self.recalculate_server(db, id).await;
                }
            }
            EventV1::ServerMemberJoin { id, user } => {
                // ! FIXME: create server create event
                // which includes server and channel objects
                if user == &self.cache.user_id && self.cache.servers.get(id).is_none() {
                    if let Ok(server) = db.fetch_server(id).await {
                        self.cache.servers.insert(id.to_string(), server);
                        self.recalculate_server(db, id).await;
                    }
                }
            }
            EventV1::ServerMemberLeave { id, user } => {
                if user == &self.cache.user_id {
                    self.remove_subscription(id);

                    if let Some(server) = self.cache.servers.remove(id) {
                        for channel in &server.channels {
                            self.remove_subscription(channel);
                            self.cache.channels.remove(channel);
                        }
                    }
                }
            }
            EventV1::ServerDelete { id } => {
                self.remove_subscription(id);

                if let Some(server) = self.cache.servers.remove(id) {
                    for channel in &server.channels {
                        self.remove_subscription(channel);
                        self.cache.channels.remove(channel);
                    }
                }
            }
            EventV1::ServerMemberUpdate { id, data, clear } => {
                if id.user == self.cache.user_id {
                    if let Some(member) = self.cache.members.get_mut(&id.server) {
                        for field in &clear.clone() {
                            member.remove(field);
                        }

                        member.apply_options(data.clone());
                    }

                    if data.roles.is_some() || clear.contains(&FieldsMember::Roles) {
                        self.recalculate_server(db, &id.server).await;
                    }
                }
            }
            EventV1::ServerRoleUpdate {
                id,
                role_id,
                data,
                clear,
                ..
            } => {
                if let Some(server) = self.cache.servers.get_mut(id) {
                    if let Some(role) = server.roles.get_mut(role_id) {
                        for field in &clear.clone() {
                            role.remove(field);
                        }

                        role.apply_options(data.clone());
                    }
                }

                if data.rank.is_some() || data.permissions.is_some() {
                    if let Some(member) = self.cache.members.get(id) {
                        if let Some(roles) = &member.roles {
                            if roles.contains(role_id) {
                                self.recalculate_server(db, id).await;
                            }
                        }
                    }
                }
            }
            EventV1::ServerRoleDelete { id, role_id } => {
                if let Some(server) = self.cache.servers.get_mut(id) {
                    server.roles.remove(role_id);
                }

                if let Some(member) = self.cache.members.get(id) {
                    if let Some(roles) = &member.roles {
                        if roles.contains(role_id) {
                            self.recalculate_server(db, id).await;
                        }
                    }
                }
            }
            EventV1::UserRelationship { id, user, .. } => {
                self.cache.users.insert(id.clone(), user.clone());

                if self.cache.can_subscribe_to_user(id) {
                    self.insert_subscription(id.clone());
                } else {
                    self.remove_subscription(id);
                }
            }
            _ => {}
        }
    }
}

impl EventV1 {
    pub async fn p(self, channel: String) {
        redis_kiss::p(channel, self).await;
    }
}
