use serenity::client::{Client, Context};
use serenity::framework::standard::{macros::group, CommandResult, StandardFramework};
use serenity::model::prelude::Message;
use serenity::prelude::TypeMapKey;

use std::collections::BTreeMap;

use std::{env, error};

use std::time::{Duration, Instant};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;

use log::{error, info, trace};

use super::commands::*;
use super::events::AppHandle;
use super::helpers::send_text;
use super::models;
use super::schema;

static ADMIN_COMMANDS: &[&str] = &["test", "debug"];
static MAIN_COMMANDS: &[&str] = &["new", "top", "random", "rising"];

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
#[commands(top, random, new, rising, debug, stats)]
#[help_available]
struct General;

type DatabasePool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

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
    pub fn check(ctx: &mut Context, msg: &Message, cmd_name: &str) -> bool {
        if msg.author.bot {
            return false;
        }

        if ADMIN_COMMANDS.contains(&cmd_name)
            && env::var("POSTMAN_ADMIN")
                .expect("POSTMAN_ADMIN wasn't specified")
                .parse::<u64>()
                .unwrap()
                != *msg.author.id.as_u64()
        {
            return false;
        }

        let mut client_data = ctx.data.write();
        let appdata = client_data
            .get_mut::<AppData>()
            .expect("failed to get appdata");

        if let Some(ptime) = appdata.cooldowns.get(&msg.author.tag()) {
            if ptime.elapsed() < appdata.cooldown_time {
                send_text(ctx, msg, "```please wait a bit before doing any command```").unwrap();
                return false;
            }
        }
        appdata.cooldowns.insert(msg.author.tag(), Instant::now());

        if !MAIN_COMMANDS.contains(&cmd_name) {
            return true;
        }

        let conn = appdata.db_pool.get().unwrap();

        update_guild(msg, &conn);
        update_user(ctx, msg, &conn);

        true
    }

    pub fn after(_: &mut Context, _: &Message, cmd_name: &str, error: CommandResult) {
        if let Err(err) = error {
            error!("error while running {}: {:?}", cmd_name, err);
        }
    }

    pub fn start() -> Result<(), AppError> {
        let mgr: r2d2::ConnectionManager<PgConnection> = r2d2::ConnectionManager::new(
            env::var("POSTMAN_DATABASE_URL").expect("POSTMAN_DATABASE_URL wasn't specified"),
        );

        let pool = r2d2::Pool::builder()
            .max_size(15)
            .build(mgr)
            .expect("error creating database pool");

        let mut client = Client::new(
            env::var("POSTMAN_DISCORD_TOKEN").expect("POSTMAN_DISCORD_TOKEN wasn't specified"),
            AppHandle,
        )?;

        let prefix = env::var("POSTMAN_PREFIX").expect("POSTMAN_PREFIX wasn't specified");

        let cooldown_time = env::var("POSTMAN_COOLDOWN_TIME")
            .unwrap_or("3".to_string())
            .parse::<u64>()
            .unwrap();

        client.with_framework(
            StandardFramework::new()
                .configure(|c| c.prefix(&prefix))
                .before(Self::check)
                .after(Self::after)
                .group(&GENERAL_GROUP)
                .help(&HELP),
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

fn update_guild(msg: &Message, conn: &PgConnection) {
    use schema::guilds::dsl::*;

    let guild_id = *msg.guild_id.unwrap().as_u64();

    let res = guilds
        .filter(id.eq(guild_id as i64))
        .first::<models::Guild>(conn);

    if let Err(diesel::NotFound) = res {
        trace!("creating guild with id: {}", guild_id as i64);
        diesel::insert_into(guilds)
            .values(models::Guild::new(guild_id))
            .execute(conn)
            .expect("failed creating guild");
    }
}

fn update_user(ctx: &Context, msg: &Message, conn: &PgConnection) {
    use schema::users::dsl::*;

    let res = users
        .filter(id.eq(*msg.author.id.as_u64() as i64))
        .first::<models::User>(conn);

    match res {
        Ok(user) => {
            trace!("updating user {} to rank {}", msg.author.id, user.rank + 1);
            diesel::update(users)
                .set(rank.eq(user.rank + 1))
                .execute(conn)
                .expect("failed updating rank");
        }
        Err(diesel::NotFound) => {
            diesel::insert_into(users)
                .values(models::User::new(*msg.author.id.as_u64()))
                .execute(conn)
                .expect("failed to save a user");

            use schema::guildusers::dsl::*;

            diesel::insert_into(guildusers)
                .values(models::Guilduser::new(
                    *msg.guild_id.unwrap().as_u64(),
                    *msg.author.id.as_u64(),
                ))
                .execute(conn)
                .expect("failed to save guilduser");

            send_text(ctx, msg, "you used me for the first time!").unwrap();
        }
        Err(e) => {
            error!("failed to get user from database: {}", e);
        }
    }
}
