use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::Message;

#[command]
#[help_available(false)]
pub fn test(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "test")?;
    Ok(())
}
