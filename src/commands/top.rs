use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::Message;

use super::helpers::*;

#[command]
pub fn top(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    handle_post(
        &format!("https://reddit.com/r/{}/top.json?t=hour", &parse_sub(args)?),
        ctx,
        msg,
    )
}
