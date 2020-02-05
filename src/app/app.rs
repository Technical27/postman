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
    env, error,
    time::{Duration, Instant},
};

use diesel::prelude::*;
use diesel::r2d2;
use diesel::sqlite::SqliteConnection;

use super::commands::*;

use super::helpers::CONFIG;

use super::models;
use super::schema;

#[derive(Debug, Clone)]
pub struct AppError(String);

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for AppError {}

impl From<serenity::Error> for AppError {
    fn from(err: serenity::Error) -> Self {
        Self(format!("error with client: {}", err))
    }
}

pub type AppResult = Result<(), AppError>;

#[group]
#[commands(top, test, random, new, rising, debug)]
struct General;

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
        use schema::messages::dsl as table;

        let mut client_data = ctx.data.write();
        let appdata = client_data.get_mut::<AppData>().unwrap();
        let conn = appdata.db_pool.get().unwrap();

        let msg = reaction.message(&ctx.http).unwrap();
        let user = reaction.user(&ctx).unwrap();

        if !msg.author.bot || user.bot {
            return;
        }

        if appdata.client_id == msg.author.id.0 {
            if let ReactionType::Unicode(emoji) = reaction.emoji {
                if emoji == "\u{274C}" {
                    let results = table::messages
                        .filter(table::msg_id.eq(*msg.id.as_u64() as i64))
                        .load::<models::Message>(&conn)
                        .expect("error getting messages from database");

                    if results.len() < 1 {
                        return;
                    }

                    let cmd_msg_id = results[0].cmd_msg_id as u64;

                    let cmd_msg = ctx
                        .http
                        .get_message(*msg.channel_id.as_u64(), cmd_msg_id)
                        .expect("failed to get message");

                    if cmd_msg.author != user {
                        return;
                    }

                    msg.delete(&ctx).expect("failed to delete message");
                    cmd_msg.delete(&ctx).expect("failed to delete message");
                }
            }
        }
    }
}

pub struct AppData {
    pub cooldowns: HashMap<String, Instant>,
    pub cooldown_time: Duration,
    pub client_id: u64,
    pub db_pool: r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>,
}

impl AppData {
    pub fn new(
        cooldown_time: u64,
        client_id: u64,
        db_pool: r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>,
    ) -> Self {
        Self {
            cooldowns: HashMap::default(),
            cooldown_time: Duration::from_secs(cooldown_time),
            client_id,
            db_pool,
        }
    }
}

impl TypeMapKey for AppData {
    type Value = Self;
}

pub struct App;

impl App {
    pub fn check(ctx: &mut Context, msg: &Message, _cmd_name: &str) -> bool {
        let re = Regex::new(r"discord.gg/[a-zA-Z0-9]{6}").expect("failed creating regex");

        if re.is_match(&msg.content) || msg.author.bot {
            return false;
        }

        let mut client_data = ctx.data.write();
        let appdata = client_data
            .get_mut::<AppData>()
            .expect("failed to get appdata");

        if appdata.client_id == *msg.author.id.as_u64() {
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

    pub fn start() -> Result<(), AppError> {
        let mgr: r2d2::ConnectionManager<SqliteConnection> = r2d2::ConnectionManager::new(
            env::var("DATABASE_URL").expect("no database location was specified"),
        );

        let pool = r2d2::Pool::builder()
            .max_size(15)
            .build(mgr)
            .expect("error creating database pool");

        let mut client = Client::new(
            env::var("DISCORD_TOKEN").expect("no discord token was specified"),
            AppHandle,
        )?;

        let prefix = CONFIG["prefix"].to_string();

        let cooldown_time = match CONFIG["cooldown_time"].as_u64() {
            Some(time) => time,
            None => 3,
        };

        let client_id = CONFIG["client_id"]
            .as_u64()
            .expect("no client id was specified");

        client.with_framework(
            StandardFramework::new()
                .configure(|c| c.prefix(&prefix))
                .before(Self::check)
                .group(&GENERAL_GROUP),
        );

        {
            let mut client_data = client.data.write();
            client_data.insert::<AppData>(AppData::new(cooldown_time, client_id, pool));
        }

        client.start()?;

        Ok(())
    }
}
