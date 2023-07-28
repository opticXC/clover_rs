use serenity::{
    model::prelude::application_command::ApplicationCommandInteraction, prelude::Context,
};

pub async fn command_error(ctx: &Context, command: &ApplicationCommandInteraction) {
    let _ = match command.get_interaction_response(&ctx.http).await {
        Ok(mut res) => res
            .edit(&ctx.http, |edit| edit.content("Error Executing Command"))
            .await
            .unwrap(),
        Err(_) => command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content("Error Executing Command").ephemeral(true)
                })
            })
            .await
            .unwrap(),
    };
}
