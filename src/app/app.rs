use serenity::{
    client::{Client, Context, EventHandler},
    framework::standard::{macros::group, StandardFramework},
    model::prelude::Message,
    model::prelude::Reaction,
    model::prelude::ReactionType,
    prelude::TypeMapKey,
};

use regex::Regex;

use std::{
    collections::HashMap,
    error,
    time::{Duration, Instant},
};

use super::commands::*;

use super::helpers::*;

#[group]
#[commands(top, test, random, new)]
pub struct General;

struct AppHandle;

impl EventHandler for AppHandle {
    fn message(&self, ctx: Context, msg: Message) {
        let re = Regex::new(r"discord.gg/[a-zA-Z0-9]{6}").unwrap();
        if re.is_match(&msg.content) {
            println!(
                "found discord invite: {}\nfrom: {}",
                msg.content, msg.author.name
            );
            msg.delete(ctx.http).unwrap();
        }
    }
    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let mut client_data = ctx.data.write();
        let appdata = client_data.get_mut::<AppData>().unwrap();

        let msg = reaction.message(&ctx.http).unwrap();
        let user = reaction.user(&ctx).unwrap();

        if !msg.author.bot || user.bot {
            return;
        }

        if appdata.client_id == msg.author.id.0 {
            if let ReactionType::Unicode(emoji) = reaction.emoji {
                if emoji == "\u{274C}" {
                    msg.delete(&ctx).unwrap();
                }
            }
        }
    }
}

struct AppData {
    cooldowns: HashMap<String, Instant>,
    cooldown_time: Duration,
    client_id: u64,
}

impl AppData {
    pub fn new(cooldown_time: u64, client_id: u64) -> Self {
        Self {
            cooldowns: HashMap::default(),
            cooldown_time: Duration::from_secs(cooldown_time),
            client_id,
        }
    }
}

impl TypeMapKey for AppData {
    type Value = Self;
}

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub fn check(ctx: &mut Context, msg: &Message, _cmd_name: &str) -> bool {
        let re = Regex::new(r"discord.gg/[a-zA-Z0-9]{6}").unwrap();

        if re.is_match(&msg.content) || msg.author.bot {
            return false;
        }

        let mut client_data = ctx.data.write();
        let appdata = client_data.get_mut::<AppData>().unwrap();

        if appdata.client_id == msg.author.id.0 {
            return false;
        }

        if let Some(ptime) = appdata.cooldowns.get(&msg.author.tag()) {
            if ptime.elapsed() < appdata.cooldown_time {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.content("`please wait a bit before doing any command`")
                    })
                    .unwrap();
                return false;
            }
        }
        appdata.cooldowns.insert(msg.author.tag(), Instant::now());
        true
    }

    pub fn start(&mut self) -> Result<(), Box<dyn error::Error>> {
        let data = load_data();
        let mut client = Client::new(data["token"].to_string(), AppHandle)?;

        let prefix = data["prefix"].to_string();

        let cooldown_time = match data["cooldown_time"].as_u64() {
            Some(time) => time,
            None => 3,
        };

        let client_id = data["client_id"].as_u64().unwrap();

        client.with_framework(
            StandardFramework::new()
                .configure(|c| c.prefix(&prefix))
                .before(Self::check)
                .group(&GENERAL_GROUP),
        );

        {
            let mut client_data = client.data.write();
            client_data.insert::<AppData>(AppData::new(cooldown_time, client_id));
        }

        client.start()?;

        Ok(())
    }
}
