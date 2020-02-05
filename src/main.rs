#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;

mod app;
use app::App;
use app::AppResult;

fn main() -> AppResult {
    dotenv().ok();
    pretty_env_logger::init();
    App::start()
}
