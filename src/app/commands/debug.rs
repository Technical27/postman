use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
};

use super::helpers::send_text;

fn get_version() -> String {
    format!(
        "{}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    )
}

#[command]
pub fn debug(ctx: &mut Context, msg: &Message) -> CommandResult {
    send_text(ctx, msg, &format!("version: {}", get_version()))?;
    Ok(())
}
