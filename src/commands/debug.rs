use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::Message;

use regex::Regex;

use lazy_static::lazy_static;

use super::helpers::{get_version, parse_post, send_post, send_text};
use super::reddit::get_reddit_api;

lazy_static! {
    static ref URL_REGEX: Regex =
        { Regex::new(r"https?://reddit.com/[a-zA-Z/_0-9]+\.json").unwrap() };
}

#[command]
#[help_available(false)]
pub fn debug(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    match args.single_quoted::<String>() {
        Ok(arg) => {
            if !URL_REGEX.is_match(&arg) {
                return send_text(ctx, msg, "`invalid url`");
            }

            let data = get_reddit_api(arg.as_str())?;
            let post = parse_post(&data[0]["data"]["children"][0])?;
            send_post(ctx, msg, &post)
        }
        Err(_) => send_text(ctx, msg, &format!("`version: {}`", get_version())),
    }
}
