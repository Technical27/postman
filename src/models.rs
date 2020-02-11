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

#[derive(Queryable, Insertable, Debug)]
pub struct User {
    pub id: i64,
    pub rank: i64,
}

#[derive(Queryable, Insertable, Debug, Associations)]
#[belongs_to(Guild)]
#[belongs_to(User)]
pub struct Guilduser {
    pub user_id: i64,
    pub guild_id: i64,
}
