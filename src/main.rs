#[macro_use]
extern crate diesel;

use log::error;

use dotenv::dotenv;

use std::env;

mod app;
use app::App;

fn main() {
    env::set_var("POSTMAN_LOG", "info");
    dotenv().ok();
    pretty_env_logger::init_custom_env("POSTMAN_LOG");
    if let Err(err) = App::start() {
        error!("{}", err);
    }
}
