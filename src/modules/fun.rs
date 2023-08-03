use serenity::{
    async_trait,
    builder::CreateEmbed,
    model::prelude::{
        application_command::ApplicationCommandInteraction, Interaction, InteractionResponseType,
        Ready,
    },
    prelude::Context,
};

use crate::{config::BotConfig, commons};

use super::ModuleTrait;


#[derive(Debug)]
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
            match command.data.name.as_str() {
                "fact" => fact(&ctx, &command).await,
                "images" => animal_images(&ctx, &command).await,
                _ => return 
            }
        }
    }
    async fn ready(&self, config: &BotConfig, ctx: &Context, _ready: &Ready) {
        let g_id = config.guild_id;

        g_id.create_application_command(&ctx.http, |command| {
            command.name("fact").description("get a random fact!")
        })
        .await
        .unwrap();

        g_id.create_application_command(&ctx.http, |command| {
            command
                .name("images")
                .description("Images of animals")
                .create_option(|option| {
                    option
                        .name("type")
                        .description("image type to get")
                        .required(true)
                        .kind(serenity::model::prelude::command::CommandOptionType::String)
                        .add_string_choice("dogs", "shibes")
                        .add_string_choice("cats", "cats")
                        .add_string_choice("birds", "birds")
                })
        })
        .await
        .unwrap();
    }
}

async fn fact(ctx: &Context, command: &ApplicationCommandInteraction) {
    let _ = command
        .create_interaction_response(&ctx, |response| {
            response
                .interaction_response_data(|data| data.content("Fetching a random fact ..."))
                .kind(InteractionResponseType::ChannelMessageWithSource)
        })
        .await
        .unwrap();

    let fact = commons::webapis::get_fact().await;
    command
        .edit_original_interaction_response(&ctx, |response| {
            let mut embed: CreateEmbed = CreateEmbed::default();
            embed.title("Random useless fact");
            embed.color(641757);
            embed.field("Did You Know?", fact.text, false);

            response.add_embed(embed)
        })
        .await
        .unwrap();
}





async fn animal_images(ctx: &Context, command: &ApplicationCommandInteraction) {
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| data.content("Fetching Image"))
        })
        .await
        .unwrap();

    let mut image_type = String::new();
    for opt in command.data.options.iter() {
        if opt.name.as_str() == "type" {
            image_type = opt.clone().value.unwrap().to_string()
        }
    }

    image_type = image_type.replace('\"', "").to_string();

    let mut embed = CreateEmbed::default();

    let res = commons::webapis::get_animal_image(&image_type, 1).await;

    embed.image(&res[0]);

    command
        .edit_original_interaction_response(&ctx.http, |response| response.add_embed(embed))
        .await
        .unwrap();
}
