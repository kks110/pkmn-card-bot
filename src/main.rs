mod images;
mod tcg;
mod url;
mod messages;
mod models;
mod commands;
mod helpers;
mod database;
mod schema;

use chrono::{NaiveDate, Utc};
use commands::*;
use poise::{BoxFuture, serenity_prelude as serenity};
use poise::serenity_prelude::Message;
use models::Data;
use crate::models::CurrencyData;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn pre_command(ctx: Context<'_>) {
    // Perform pre-command actions here
    println!("Executing command {}...", ctx.command().qualified_name);

    let mut rate = match database::get_exchange_rate("EUR") {
        Ok(r) => { r },
        Err(e) => { panic!("Could not find rate for EUR") }
    };

    // TODO: Extract this to a seperate method
    if NaiveDate::parse_from_str(&rate.updated_at, "%Y-%m-%d").unwrap() != Utc::now().date_naive() {
        println!("Exchange rate is out of date, updating...");
        let exchange_api_key = std::env::var("EXCHANGE_API_KEY").expect("Missing Exchange rate API KEy");
        let url = format!("http://api.exchangeratesapi.io/v1/latest?access_key={}&format=1&symbols=GBP", exchange_api_key);
        let api_response = reqwest::get(url)
            .await
            .expect("Failed API call")
            .text()
            .await
            .expect("Failed API call");
        let currency_data: CurrencyData = serde_json::from_str(&api_response).expect("Failed to parse data");

        database::update_exchange_rates(&currency_data.base, currency_data.rates.gbp as f32)
    }
}

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

    // TODO: Probably create a seperate start up section
    // TODO: Add DB migrations to run her
    helpers::get_initial_exchange_rate().await;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                find_card::execute(),
                top_five::execute(),
            ],
            pre_command: |ctx| {
                Box::pin(pre_command(ctx))
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
                Ok(Data{})
            })
        });

    framework.run().await.unwrap();
}
