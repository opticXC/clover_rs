pub mod commands;
pub mod commons;
pub mod config;
pub mod handler;

use std::io::Write;

use config::BotConfig;
use env_logger;
use handler::Handler;
use log::info;
use serenity::prelude::*;

use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    init_logger();

    let file_path = std::path::Path::new("./config.json");
    let bot_config = match BotConfig::load_from_path(file_path) {
        Ok(b_c) => b_c,
        Err(why) => {
            info!(
                "Creating default config at {:?} -- Remember to Fill it first before running",
                file_path.as_os_str()
            );
            let _ = BotConfig::default().save_to_path(file_path);
            std::process::exit(1);
        }
    };

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(bot_config.get_bot_login_token(), intents)
        .event_handler(Handler { bot_config })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        log::error!("Error starting client - {}", why);
    }

    Ok(())
}

fn init_logger() {
    let log_level = if cfg!(debug_assertion) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    env_logger::Builder::new()
        .filter_level(log_level)
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
