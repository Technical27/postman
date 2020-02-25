use serenity::client::{Context, EventHandler};
use serenity::model::gateway::{Activity, Ready};
use serenity::model::prelude::{Reaction, ReactionType};

use log::{error, info};

use diesel::prelude::*;

use super::app::AppData;
use super::models;
use super::schema;

pub struct AppHandle;

impl EventHandler for AppHandle {
    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let msg = reaction.message(&ctx.http).unwrap();
        let user = reaction.user(&ctx).unwrap();

        if !msg.author.bot || user.bot {
            return;
        }

        use schema::messages::dsl as table;

        let mut client_data = ctx.data.write();
        let appdata = client_data.get_mut::<AppData>().unwrap();
        let conn = appdata.db_pool.get().unwrap();

        if appdata.client_id.expect("failed to get client_id") == msg.author.id.0 {
            if let ReactionType::Unicode(emoji) = reaction.emoji {
                if emoji == "\u{274C}" {
                    let res = table::messages
                        .filter(table::msg_id.eq(*msg.id.as_u64() as i64))
                        .first::<models::Message>(&conn);

                    match res {
                        Ok(old_msg) => {
                            let cmd_msg_id = old_msg.cmd_msg_id as u64;
                            let cmd_msg = ctx
                                .http
                                .get_message(*msg.channel_id.as_u64(), cmd_msg_id)
                                .expect("failed to get message");

                            if cmd_msg.author != user {
                                return;
                            }

                            msg.delete(&ctx).expect("failed to delete message");
                            cmd_msg.delete(&ctx).expect("failed to delete message");
                        }
                        Err(diesel::NotFound) => {
                            return;
                        }
                        Err(e) => {
                            error!("failed getting message from database: {:?}", e);
                        }
                    }
                }
            }
        }
    }

    fn ready(&self, ctx: Context, bot_data: Ready) {
        let mut client_data = ctx.data.write();
        let appdata = client_data.get_mut::<AppData>().unwrap();

        appdata.client_id = Some(*bot_data.user.id.as_u64());

        ctx.set_activity(Activity::playing("-help"));

        info!("logged in");
    }
}
