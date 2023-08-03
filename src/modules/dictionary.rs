

use log::info;
use serenity::model::prelude::{
    application_command::ApplicationCommandInteraction, Interaction, Ready,
};
use serenity::{async_trait, builder::CreateEmbed, prelude::Context};
use crate::config::BotConfig;
use super::ModuleTrait;

use webster;
pub struct Mod;

#[async_trait]
impl ModuleTrait for Mod {
    async fn ready(&self, config: &BotConfig, ctx: &Context, _ready: &Ready) {
        let g_id = &config.guild_id;
        g_id.create_application_command(&ctx.http, |command| {
            command
                .name("dictionary")
                .description("Fetch Dictionary entry for a word")
                .create_option(|option| {
                    option
                        .name("word")
                        .description("word to fetch")
                        .required(true)
                        .kind(serenity::model::prelude::command::CommandOptionType::String)
                })
        })
        .await
        .unwrap();

        info!("Decompressing Webster Dictionary");
        webster::preload();
    }

    async fn interaction_create(
        &self,
        _config: &BotConfig,
        ctx: &Context,
        interaction: &Interaction,
    ) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "dictionary" => dictionary_run(&ctx, command).await,
                _ => return,
            }
        }
    }
}

async fn dictionary_run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut word = String::new();
    for opt in command.data.options.iter() {
        if opt.name.as_str() == "word" {
            word = opt.value.clone().unwrap().to_string().replace("\"", "");
        }
    }
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| data.content("Fetching Word"))
        })
        .await
        .unwrap();
    let def = match webster::dictionary(&word){
        Some(d) => d,
        None => {
            command.edit_original_interaction_response(&ctx, |edit|{
                edit.content("Word Not Found")
            }).await.unwrap();
            return;
        }
    };
    
    let mut embed = CreateEmbed::default();
    embed.title(&word);
    embed.description(def);
    embed.color(641757);

    command
        .edit_original_interaction_response(&ctx, |edit| edit.add_embed(embed))
        .await
        .unwrap();
}


#[cfg(test)]
mod tests{
    #[test]
    fn test_dictionary(){
        let word = "clover";
        let def = webster::dictionary(word).unwrap();
        println!("Found:\n {def}");
    }
}