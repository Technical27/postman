use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::Message;

use super::helpers::*;

#[command]
#[description("gets the top hourly post from a sub")]
#[usage("top [sub]")]
#[example("top teenagers")]
#[example("top")]
#[max_args(1)]
pub fn top(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    handle_post("top.json?t=hour", args, ctx, msg)
}
