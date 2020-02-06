use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::Message;

#[command]
pub fn test(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "test")?;
    Ok(())
}
