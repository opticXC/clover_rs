use serenity::{
    async_trait,
    model::prelude::{application_command::ApplicationCommandInteraction, Interaction, Ready},
    prelude::Context,
};

use crate::config::BotConfig;

use super::ModuleTrait;

pub struct Mod;

#[async_trait]
impl ModuleTrait for Mod {
    async fn interaction_create(
        &self,
        _config: &BotConfig,
        ctx: &Context,
        interaction: &Interaction,
    ) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let _ = match command.data.name.as_str() {
                "ping" => ping(&ctx, &command).await,
                _ => return,
            };
        }
    }

    async fn ready(&self, config: &BotConfig, ctx: &Context, _ready: &Ready) {
        log::info!("Setting up Module bot_utils");

        let g_id = config.guild_id;

        g_id.create_application_command(&ctx.http, |command| {
            command.name("ping").description("pong")
        })
        .await
        .unwrap();
    }
}

async fn ping(ctx: &Context, command: &ApplicationCommandInteraction) {
    let s_time = std::time::SystemTime::now();
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .interaction_response_data(|data| {
                    data.content("calculating ping ...").ephemeral(true)
                })
                .kind(serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource)
        })
        .await
        .unwrap();

    command
        .edit_original_interaction_response(&ctx.http, |response| {
            response.content(format!(
                "pong! {} ms",
                s_time.elapsed().unwrap().as_millis()
            ))
        })
        .await
        .unwrap();
}
