use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
};

use super::helpers::*;

#[command]
pub fn new(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    handle_post(
        &format!("https://reddit.com/r/{}/new.json", &parse_sub(args)?),
        ctx,
        msg,
    )
}
