mod images;
mod tcg;
mod url;
mod messages;
mod models;
mod commands;
mod helpers;

use commands::*;
use poise::serenity_prelude as serenity;
use models::Data;

const CURRENT_DATA_VERSION: f32 = 0.1;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let data = match helpers::load_app_data().await {
        Ok(a) => { a }
        Err(e) => { panic!("Error occurred starting up: {}", e); }
    };
    println!("Data Loaded: {:?}", data);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                find_card::execute(),
                top_five::execute(),
            ],
            pre_command: |ctx| {
                Box::pin(async move {
                    println!("Executing command {}...", ctx.command().qualified_name);
                })
            },
            /// This code is run after a command if it was successful (returned Ok)
            post_command: |ctx| {
                Box::pin(async move {
                    println!("Executed command {}!", ctx.command().qualified_name);
                })
            },
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::serenity_prelude::GuildId(
                    std::env::var("GUILD_ID").expect("missing GUILD_ID").parse().expect("Guild ID should be a number")
                )
                    .set_application_commands(ctx, |b| {
                        *b = poise::builtins::create_application_commands(
                            &framework.options().commands,
                        );
                        b
                    })
                    .await
                    .unwrap();
                Ok(data)
            })
        });

    framework.run().await.unwrap();
}
