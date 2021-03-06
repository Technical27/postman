use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::Message;

use regex::Regex;

use lazy_static::lazy_static;

use systemstat::{saturating_sub_bytes, Platform, System};

use super::helpers::{get_version, send_text};

lazy_static! {
    static ref URL_REGEX: Regex =
        { Regex::new(r"https?://reddit.com/[a-zA-Z/_0-9]+\.json").unwrap() };
}

#[command]
#[help_available(false)]
pub fn debug(ctx: &mut Context, msg: &Message) -> CommandResult {
    let sys = System::new();

    let load_avg = sys.load_average().unwrap();
    let mem = sys.memory().unwrap();
    let temp = sys.cpu_temp().unwrap();

    send_text(
        ctx,
        msg,
        &format!(
            "```version: {}\navg cpu load: {:?}%\nmem used: {:?}\ncpu temp: {:?}C```",
            get_version(),
            load_avg.fifteen,
            saturating_sub_bytes(mem.total, mem.free),
            temp,
        ),
    )
}
