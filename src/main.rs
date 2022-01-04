#[macro_use]
extern crate diesel;

use database::connection_pool::pg_pool;
use dotenv::dotenv;
use log::info;
use server::api_filters;
use std::env;
use warp::Filter;

mod database;
mod error;
mod server;

itconfig::config! {
    PORT: u16 => 8000,
    database {
        URL < (
            "postgres://",
            POSTGRES_USERNAME => "postgres",
            ":",
            POSTGRES_PASSWORD: String,
            "@",
            POSTGRES_HOST => "localhost:5432",
            "/",
            POSTGRES_DB => "test",
        ),

        pool {
            MAX_SIZE: u32 => 32,
            MIN_IDLE: u32 => 8,
            TIMEOUT_SECONDS: u64 => 15,
        },
    },
    MAX_REQUEST_SIZE_KB: u64 => 16
}

#[tokio::main]
async fn main() {
    // init enviroment variables
    dotenv().expect("dotenv setup to be successful");
    config::init();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    // init logger
    pretty_env_logger::init();

    // init db connection pool
    let pg_pool = pg_pool(
        &config::database::URL(),
        config::database::pool::MAX_SIZE(),
        config::database::pool::MIN_IDLE(),
        config::database::pool::TIMEOUT_SECONDS(),
    );

    // setup routes
    let routes = api_filters(pg_pool).recover(error::handle_rejection);

    info!("Starting server on port {}", config::PORT());

    // start server
    warp::serve(routes)
        .run(([127, 0, 0, 1], config::PORT()))
        .await;
}
