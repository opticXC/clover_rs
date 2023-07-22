pub mod commands;
pub mod commons;
pub mod config;
pub mod handler;

use std::io::Write;

use config::BotConfig;
use env_logger;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    init_logger();

    let file_path = std::path::Path::new("./config.json");
    let bot_config = BotConfig::load_from_path(file_path).unwrap();
}

fn init_logger() {
    let log_level = if cfg!(debug_assertion) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    env_logger::Builder::new()
        .filter(None, log_level)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {} ",
                chrono::Local::now().format("%Y-%m-%d|%T"),
                record.level(),
                record.args()
            )
        })
        .init();

    log::info!("Logger started with log level: {}", log_level.as_str());
}
