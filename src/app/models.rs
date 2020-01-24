use super::schema::messages;
use serenity::model::channel;

#[derive(Queryable, Debug, Insertable)]
pub struct Message {
    pub msg_id: i64,
    pub cmd_msg_id: i64
}

impl Message {
    pub fn new(msg_id: &channel::Message, cmd_msg_id: &channel::Message) -> Self {
        Self { msg_id: msg_id.id.0 as i64, cmd_msg_id: cmd_msg_id.id.0 as i64 }
    }
}
