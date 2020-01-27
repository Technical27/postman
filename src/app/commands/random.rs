use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
};

use super::helpers::*;
use super::post::Post;
use super::reddit::*;
use super::CommandError;

fn get_random(sub: &str) -> RedditResult<Post> {
    let mut i = 0;

    let post = loop {
        if i == 5 {
            return Err(RedditAPIError::new("cant find any image"));
        }
        let res = get_reddit_api(&format!("https://reddit.com/r/{}/random.json", sub))?;
        if let Some(post) = parse_post(&res[0]["data"]["children"][0]) {
            break post;
        }
        i += 1;
    };

    Ok(post)
}

#[command]
pub fn random(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let post = get_random(&parse_sub(args)?)?;
    if post.nsfw && !check_nsfw(ctx, msg)? {
        return send_error(ctx, msg, CommandError::boxed("this channel isn't nsfw"));
    }
    send_post(ctx, msg, &post)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_random_posts() {
        let subs = ["dankmemes", "memes", "cursedcomments"];
        let mut passed = false;
        for sub in subs.iter() {
            if let Ok(_) = get_random(sub) {
                passed = true;
                break;
            }
        }
        assert!(passed);
    }
    #[test]
    fn posts_are_random() {
        let post1 = get_random("memes").unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let post2 = get_random("memes").unwrap();
        let equal = post1.title == post2.title && post1.author == post2.author;

        assert!(!equal);
    }
}
