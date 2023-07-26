use serenity::{
    async_trait,
    model::prelude::{Mention, Message},
    prelude::Context,
};

use crate::config::BotConfig;

use super::ModuleTrait;

pub struct Mod;

#[async_trait]
impl ModuleTrait for Mod {
    async fn message(&self, config: &BotConfig, ctx: &Context, message: &Message) {
        let r_id = config.catify_id.as_ref().unwrap();
        log::debug!("running catify on user_id {}", message.author.name);

        let g_id = match message.guild_id {
            Some(g_id) => g_id,
            None => return,
        };
        let target = match g_id.member(&ctx.http, message.author.id).await {
            Ok(m) => m,
            Err(_) => return,
        };

        if target.roles.contains(r_id) {
            if !message.content.to_lowercase().contains("nya") {
                message.delete(&ctx.http).await.unwrap();
                let self_msg = message
                    .channel_id
                    .say(
                        &ctx.http,
                        format!("{},You have been catified nya~\nYou hvae to use 'nya' in your messages nya~", Mention::User(message.author.id.clone())),
                    )
                    .await
                    .unwrap();
                tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;
                self_msg.delete(&ctx.http).await.unwrap();
            }
        }
    }
}
