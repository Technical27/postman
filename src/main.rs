#[macro_use]
extern crate diesel;

use log::error;

use dotenv::dotenv;

use std::env;

mod app;
mod commands;
mod events;
mod helpers;
mod models;
mod post;
mod reddit;
mod schema;

use app::App;

fn main() {
    dotenv().ok();

    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "postman=trace");
    }

    pretty_env_logger::init();
    if let Err(err) = App::start() {
        error!("{}", err);
    }
}
