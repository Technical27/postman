use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::Message;

use std::thread;
use std::time::Duration;

use super::helpers::*;
use super::post::*;
use super::reddit::*;

fn get_random(sub: &str, nsfw: bool) -> PostResult {
    for _ in 0..5 {
        let res = get_reddit_api(&format!("https://reddit.com/r/{}/random.json", sub))?;
        let post_data = &res[0]["data"]["children"][0];

        if post_data.is_null() {
            return Err(PostError::NoPostsFound);
        }

        if let Some(post) = parse_post(post_data) {
            if !post.nsfw || nsfw {
                return Ok(post);
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
    Err(PostError::NoImagesFound)
}

#[command]
#[description("gets a random post from a sub")]
#[usage("random [sub]")]
#[example("random cursedcomments")]
#[example("random")]
#[max_args(1)]
pub fn random(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let sub = match parse_sub(args) {
        Ok(sub) => sub,
        Err(e) => return send_text(ctx, msg, &format!("{}", e)),
    };

    match get_random(&sub, check_nsfw(ctx, msg)?) {
        Ok(post) => send_post(ctx, msg, &post),
        Err(err) => send_text(ctx, msg, &format!("{}", err)),
    }
}
