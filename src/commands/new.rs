use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::Message;

use super::helpers::*;

#[command]
#[description("gets the newest post from a sub")]
#[usage("new [sub]")]
#[example("new programmerhumor")]
#[example("new")]
#[max_args(1)]
pub fn new(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    handle_post(
        &format!("https://reddit.com/r/{}/new.json", &parse_sub(args)?),
        ctx,
        msg,
    )
}
