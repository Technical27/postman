use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::Message;

use std::thread;
use std::time::Duration;

use super::helpers::*;
use super::post::Post;
use super::reddit::*;

fn get_random(sub: &str, nsfw: bool, tries: u8) -> RedditResult<Post> {
    thread::sleep(Duration::from_millis(100));
    let res = get_reddit_api(&format!("https://reddit.com/r/{}/random.json", sub))?;
    if tries > 4 {
        return Err(RedditAPIError::new("can't find any post"));
    }
    if let Ok(post) = parse_post(&res[0]["data"]["children"][0]) {
        if post.nsfw && !nsfw {
            return get_random(sub, nsfw, tries + 1);
        }
        return Ok(post);
    }
    return get_random(sub, nsfw, tries + 1);
}

#[command]
pub fn random(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    match get_random(&parse_sub(args)?, check_nsfw(ctx, msg)?, 1) {
        Ok(post) => send_post(ctx, msg, &post),
        Err(err) => send_text(ctx, msg, &format!("`{}`", err)),
    }
}
