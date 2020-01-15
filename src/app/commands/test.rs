use serenity::{
    framework::standard::{
        CommandResult,
        macros::command,
    },
    client::Context,
    model::prelude::Message
};

#[command]
pub fn test(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "test")?;
    Ok(())
}
