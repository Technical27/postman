use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::Message;

use lazy_static::lazy_static;

use log::error;

use regex::Regex;

use diesel::prelude::*;

use super::app::AppData;
use super::helpers::*;
use super::models;
use super::schema;

lazy_static! {
    static ref USER_REGEX: Regex = { Regex::new(r"^<@!?(\d+)>$").unwrap() };
}

#[command]
#[description("gets your or someone else's stats")]
#[usage("stats [user]")]
#[example("stats")]
#[max_args(1)]
pub fn stats(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id: i64 = match args.single::<String>() {
        Ok(user_name) => {
            let caps = USER_REGEX.captures(&user_name).unwrap();
            if let Some(user_id) = caps.get(1) {
                user_id.as_str().parse::<u64>().unwrap() as i64
            } else {
                *msg.author.id.as_u64() as i64
            }
        }
        Err(_) => *msg.author.id.as_u64() as i64,
    };
    use schema::users::dsl::*;

    let mut client_data = ctx.data.write();
    let appdata = client_data.get_mut::<AppData>().unwrap();
    let conn = appdata.db_pool.get().unwrap();

    let res = users.filter(id.eq(user_id)).first::<models::User>(&conn);

    match res {
        Ok(user) => {
            send_text(ctx, msg, &format!("rank: {}", user.rank))?;
        }
        Err(diesel::NotFound) => {
            send_text(ctx, msg, "looks like you don't have a rank :(")?;
        }
        Err(e) => {
            error!("error getting user from database: {}", e);
        }
    }

    Ok(())
}
