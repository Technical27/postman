use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::Message;

use super::helpers::*;

#[command]
#[description("gets the first rising post from a sub")]
#[usage("rising [sub]")]
#[example("rising noahgettheboat")]
#[example("rising")]
#[max_args(1)]
pub fn rising(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    handle_post(
        &format!("https://reddit.com/r/{}/rising.json", &parse_sub(args)?),
        ctx,
        msg,
    )
}
