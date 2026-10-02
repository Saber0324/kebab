use dotenv::dotenv;
use poise::{
    EditTracker,
    serenity_prelude::{self as serenity, builder::CreateAllowedMentions},
};
use std::time::Duration;

use hux_rs::{Data, Error, evaluate, hello};

#[tokio::main]
async fn main() {
    let edit_tracker = Some(std::sync::Arc::from(EditTracker::for_timespan(
        Duration::from_mins(30),
    )));

    let allowed_mentions = CreateAllowedMentions::new()
        .everyone(false)
        .all_roles(false)
        .all_users(false);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), evaluate()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                edit_tracker,
                ..Default::default()
            },
            allowed_mentions: Some(allowed_mentions),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_MESSAGE_REACTIONS
        | serenity::GatewayIntents::DIRECT_MESSAGE_REACTIONS
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client
        .expect("Failed to get client")
        .start()
        .await
        .expect("Failed to start client");
}
