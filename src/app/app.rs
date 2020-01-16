use serenity::{
    client::{
        EventHandler,
        Context,
        Client
    },
    model::prelude::Message,
    framework::standard::{
        StandardFramework,
        macros::group,
    },
    prelude::TypeMapKey
};

use regex::Regex;

use std::{
    error,
    collections::HashMap,
    time::{
        Instant,
        Duration
    }
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
            println!("found discord invite: {}\nfrom: {}", msg.content, msg.author.name);
            msg.delete(ctx.http).unwrap();
        }
    }
}

struct AppData {
    cooldowns: HashMap<String, Instant>,
    cooldown_time: Duration
}

impl AppData {
    pub fn new(cooldown_time: u64) -> Self {
        Self { cooldowns: HashMap::default(), cooldown_time: Duration::from_secs(cooldown_time) }
    }
}

impl TypeMapKey for AppData {
    type Value = Self;
}

pub struct App;

impl App {
    pub fn new() -> Self {Self}

    pub fn check(ctx: &mut Context, msg: &Message, _cmd_name: &str) -> bool {
        let re = Regex::new(r"discord.gg/[a-zA-Z0-9]{6}").unwrap();
        if re.is_match(&msg.content) { return false; }
        let mut client_data = ctx.data.write();
        let appdata = client_data.get_mut::<AppData>().unwrap();
        if let Some(ptime) = appdata.cooldowns.get(&msg.author.tag()) {
            if ptime.elapsed() < appdata.cooldown_time {
                msg.channel_id.send_message(&ctx.http, |m| m.content("`please wait a bit before doing any command`")).unwrap();
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

        let cooldown_time =
        match data["cooldown_time"].as_u64() {
            Some(time) => time,
            None => 3
        };

        client.with_framework(
            StandardFramework::new()
            .configure(|c| c.prefix(&prefix))
            .before(Self::check)
            .group(&GENERAL_GROUP)
        );

        {
            let mut client_data = client.data.write();
            client_data.insert::<AppData>(AppData::new(cooldown_time));
        }

        client.start()?;

        Ok(())
    }
}
