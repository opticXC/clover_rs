pub mod bot_utils;
pub mod catify;
pub mod dictionary;
pub mod fun;

use std::sync::Arc;

use serenity::{
    async_trait,
    model::prelude::{Interaction, Message, Ready},
    prelude::Context,
};

use crate::config::BotConfig;

pub type BotModule = Arc<dyn ModuleTrait>;

#[async_trait]
pub trait ModuleTrait: Send + Sync {
    async fn ready(&self, _config: &BotConfig, _ctx: &Context, _ready: &Ready) {}
    async fn interaction_create(
        &self,
        _config: &BotConfig,
        _ctx: &Context,
        _interaction: &Interaction,
    ) {
    }
    async fn message(&self, _config: &BotConfig, _ctx: &Context, _message: &Message) {}
}
