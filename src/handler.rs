use serenity::{
    async_trait,
    model::prelude::{Interaction, Message, Ready},
    prelude::{Context, EventHandler},
};

use crate::config::BotConfig;

pub struct Handler {
    pub bot_config: BotConfig,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        log::info!("Logged in as {}", _data_about_bot.user.name);
    }

    async fn interaction_create(&self, _ctx: Context, _interaction: Interaction) {}

    async fn message(&self, _ctx: Context, _new_message: Message) {
        match self.bot_config.catify_id {
            Some(_) => self.catify(&_ctx, &_new_message),
            None => (),
        }
    }
}

impl Handler {
    fn catify(&self, _ctx: &Context, _new_message: &Message) {
        let r_id = self.bot_config.catify_id.as_ref().unwrap();

        match _new_message.guild_id {
            Some(g_id) => todo!(),
            None => (),
        };
    }
}
