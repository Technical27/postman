use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::Message;

use super::helpers::{get_version, send_text};

#[command]
pub fn debug(ctx: &mut Context, msg: &Message) -> CommandResult {
    send_text(ctx, msg, &format!("version: {}", get_version()))?;
    Ok(())
}
