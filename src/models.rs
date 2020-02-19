use super::schema::*;

use serenity::model::channel;

#[derive(Queryable, Debug, Insertable)]
pub struct Message {
    pub msg_id: i64,
    pub cmd_msg_id: i64,
}

impl Message {
    pub fn new(msg_id: &channel::Message, cmd_msg_id: &channel::Message) -> Self {
        Self {
            msg_id: *msg_id.id.as_u64() as i64,
            cmd_msg_id: *cmd_msg_id.id.as_u64() as i64,
        }
    }
}

#[derive(Queryable, Insertable, Debug)]
pub struct Guild {
    pub id: i64,
}

impl Guild {
    pub fn new(id: u64) -> Self {
        Self { id: id as i64 }
    }
}

// model to represent a user on discord
#[derive(Queryable, Insertable, Debug)]
pub struct User {
    pub id: i64,
    pub rank: i64,
}

impl User {
    pub fn new(id: u64) -> Self {
        Self {
            id: id as i64,
            rank: 0,
        }
    }
}

// model to link Guild and User together
#[derive(Queryable, Insertable, Debug, Associations)]
#[belongs_to(Guild)]
#[belongs_to(User)]
pub struct Guilduser {
    pub user_id: i64,
    pub guild_id: i64,
}

impl Guilduser {
    pub fn new(guild_id: u64, user_id: u64) -> Self {
        Self {
            guild_id: guild_id as i64,
            user_id: user_id as i64,
        }
    }
}
