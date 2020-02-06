use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::Message;

use super::helpers::*;

use super::post::Post;

use super::reddit::*;

fn get_random(sub: &str) -> RedditResult<Post> {
    let mut i = 0;

    let post = loop {
        if i == 5 {
            return Err(RedditAPIError::new("cant find any image"));
        }
        let res = get_reddit_api(&format!("https://reddit.com/r/{}/random.json", sub))?;
        if let Ok(post) = parse_post(&res[0]["data"]["children"][0]) {
            break post;
        }
        i += 1;
    };

    Ok(post)
}

#[command]
pub fn random(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    match get_random(&parse_sub(args)?) {
        Ok(post) => {
            if post.nsfw && !check_nsfw(ctx, msg)? {
                return send_text(ctx, msg, "this channel isn't nsfw");
            }
            send_post(ctx, msg, &post)
        }
        Err(err) => send_text(ctx, msg, &format!("`{}`", err)),
    }
}
