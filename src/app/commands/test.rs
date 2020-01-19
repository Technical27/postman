use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
};

#[command]
pub fn test(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "test")?;
    Ok(())
}
