use serde::{Deserialize, Serialize};

use serenity::model::prelude::{
    application_command::ApplicationCommandInteraction, Interaction, Ready,
};
use serenity::{async_trait, builder::CreateEmbed, prelude::Context};

use crate::{commons::command_error, config::BotConfig};

use super::ModuleTrait;

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

    let word_res = match get_word(&word).await {
        Ok(res) => res,
        Err(_why) => {
            log::error!("{}", _why);
            command_error(&ctx, &command).await;
            return;
        }
    };

    let w = &word_res[0];
    let mut embed = CreateEmbed::default();
    embed.title(&w.word);
    embed.description(&w.phonetic);
    for m in w.meanings.iter() {
        embed.field(
            m.partofspeach
                .clone()
                .unwrap_or_else(|| String::from("Definition")),
            &m.definitions[0].definition,
            true,
        );
    }
    match w.sourceUrl.clone() {
        Some(v_s) => match v_s.first() {
            Some(url) => {
                let _ = embed.field("source", url.clone(), false);
            }
            None => {}
        },
        None => {}
    };

    command
        .edit_original_interaction_response(&ctx, |edit| edit.add_embed(embed))
        .await
        .unwrap();
}

type DictionaryResult = Vec<DictionaryWord>;

#[derive(Debug, Serialize, Deserialize)]
struct DictionaryWord {
    meanings: Vec<Meaning>,
    origin: Option<String>,
    phonetic: String,
    phonetics: Vec<Phonetic>,

    #[allow(non_snake_case)]
    sourceUrl: Option<Vec<String>>,

    word: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Phonetic {
    text: String,
    audio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Definition {
    definition: String,
    example: Option<String>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meaning {
    partofspeach: Option<String>,
    definitions: Vec<Definition>,
}

const BASE_DICTIONARY_API_URL: &str = "https://api.dictionaryapi.dev/api/v2/entries/en/";

async fn get_word(word: &String) -> Result<DictionaryResult, reqwest::Error> {
    let fetch_url = format!("{}{}", BASE_DICTIONARY_API_URL, word);
    let response = match reqwest::get(fetch_url).await {
        Ok(res) => res,
        Err(why) => return Err(why),
    };

    response.json::<DictionaryResult>().await
}

#[cfg(test)]
mod tests {
    use super::get_word;

    #[test]
    fn test_dictionary_api() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let word = String::from("cactus");
        let res = rt.block_on(get_word(&word));
        println!("{:?}", res);
    }
}
