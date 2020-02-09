use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{macros::group, CommandResult, StandardFramework};
use serenity::model::gateway::Ready;
use serenity::model::prelude::{Message, Reaction, ReactionType};
use serenity::prelude::TypeMapKey;

use json::JsonValue;

use lazy_static::lazy_static;

use std::collections::BTreeMap;

use std::{env, error, fs};

use std::time::{Duration, Instant};

use diesel::prelude::*;
use diesel::r2d2;
use diesel::sqlite::SqliteConnection;

use log::{error, info, trace};

use super::commands::*;
use super::models;
use super::schema;

lazy_static! {
    pub static ref CONFIG: JsonValue = {
        trace!("reading config");
        let data = json::parse(
            &String::from_utf8(fs::read("config.json").expect("failed to load config file"))
                .unwrap(),
        )
        .expect("failed to parse config file");
        data
    };
}

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

#[group]
#[commands(top, test, random, new, rising, debug)]
struct General;

struct AppHandle;

impl EventHandler for AppHandle {
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

        if appdata.client_id.expect("failed to get client_id") == msg.author.id.0 {
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

    fn ready(&self, ctx: Context, bot_data: Ready) {
        let mut client_data = ctx.data.write();
        let appdata = client_data.get_mut::<AppData>().unwrap();

        appdata.client_id = Some(*bot_data.user.id.as_u64());

        info!("logged in");
    }
}

type DatabasePool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub struct AppData {
    pub cooldowns: BTreeMap<String, Instant>,
    pub cooldown_time: Duration,
    pub client_id: Option<u64>,
    pub db_pool: DatabasePool,
}

impl AppData {
    pub fn new(cooldown_time: u64, db_pool: DatabasePool) -> Self {
        Self {
            cooldowns: BTreeMap::new(),
            cooldown_time: Duration::from_secs(cooldown_time),
            client_id: None,
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
        if msg.author.bot {
            return false;
        }

        let mut client_data = ctx.data.write();
        let appdata = client_data
            .get_mut::<AppData>()
            .expect("failed to get appdata");

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

    pub fn after(_: &mut Context, _: &Message, cmd_name: &str, error: CommandResult) {
        if let Err(err) = error {
            error!("error while running {}: {:?}", cmd_name, err);
        }
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

        client.with_framework(
            StandardFramework::new()
                .configure(|c| c.prefix(&prefix))
                .before(Self::check)
                .after(Self::after)
                .group(&GENERAL_GROUP),
        );

        {
            let mut client_data = client.data.write();
            client_data.insert::<AppData>(AppData::new(cooldown_time, pool));
        }

        info!("starting client");

        client.start()?;

        Ok(())
    }
}
