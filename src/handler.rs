use serenity::{
    async_trait,
    model::prelude::{Interaction, Message, Ready},
    prelude::{Context, EventHandler},
};

use crate::config::BotConfig;
use crate::modules::BotModule;

#[derive()]
pub struct Handler {
    pub bot_config: BotConfig,
    pub modules: Vec<BotModule>,
}

#[async_trait]

impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        log::info!("recieved message - {}", message.id.0);
        if message.author.bot {
            return;
        }

        for module in self.modules.iter() {
            module.message(&self.bot_config, &ctx, &message).await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("Logged in as {}", ready.user.name);

        log::info!("Resetting Application Commands");
        for cmd in self
            .bot_config
            .guild_id
            .get_application_commands(&ctx.http)
            .await
            .unwrap()
            .iter()
        {
            self.bot_config
                .guild_id
                .delete_application_command(&ctx.http, cmd.id)
                .await
                .unwrap();
        }

        for module in self.modules.iter() {
            module.ready(&self.bot_config, &ctx, &ready).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        for module in self.modules.iter() {
            module
                .interaction_create(&self.bot_config, &ctx, &interaction)
                .await
        }
    }
}

impl Handler {}
