mod images;
mod tcg;
mod url;
mod messages;
mod response;

use images::PNG;
use tcg::{sets, set_names, pokemon, colour_map};
use response::{CardData, ApiError};


use poise::serenity_prelude as serenity;
use futures::{Stream, StreamExt};
use std::collections::HashMap;
use reqwest;
use reqwest::Client;
use serde::{Serialize, Deserialize};

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use std::time::Duration;
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::Path;
use poise::serenity_prelude::AttachmentType;


#[poise::command(slash_command)]
async fn pokemon_set(
    ctx: Context<'_>,
    #[description = "Which Set"]
    #[autocomplete = "autocomplete_sets"]
    #[lazy]
    set: Option<String>,
    #[description = "Which Pokemon"]
    #[autocomplete = "autocomplete_pokemon"]
    pokemon: String,
    #[description = "The card number in the set"]
    #[lazy]
    card_number: Option<u32>,
) -> Result<(), Error> {
    // TODO: add better error responses
    // eg. set invalid, cant find card

    let url = url::builder(pokemon, set, card_number);

    let api_response = reqwest::get(url)
        .await?
        .text()
        .await?;

    if api_response.contains("\"error\":") {
        let parsed_data: ApiError = serde_json::from_str(&api_response)?;
        let mut fields: Vec<(String, String, bool)> = vec![];

        fields.push(
            (format!("Code"),
             format!("{}", parsed_data.error.code),
             false
            )
        );
        fields.push(
            (format!("Message"),
             format!("{}", parsed_data.error.message),
             false
            )
        );
        messages::send_error_message(ctx, "An error occurred!", fields).await?;

        return Ok(())
    }

    let parsed_data: CardData = serde_json::from_str(&api_response)?;

    for card in parsed_data.data {
        let file_name = format!("{}.png", card.id);
        let path = format!("D:/Users/Kev/Pictures/{}", file_name);
        let url = &card.images.small;
        let client = Client::new();
        let res = client
            .get(url)
            .send()
            .await
            .or(Err(format!("Failed to GET from '{}'", &url)))?;

        let mut file;
        let mut stream = res.bytes_stream();

        println!("Seeking in file.");
        if Path::new(&path).exists() {
            println!("File exists. Resuming.");
            file = std::fs::OpenOptions::new()
                .read(true)
                .append(true)
                .open(&path)
                .unwrap();
        } else {
            println!("Fresh file..");
            file = File::create(&path).or(Err(format!("Failed to create file '{}'", &path)))?;
        }

        println!("Commencing transfer");
        while let Some(item) = stream.next().await {
            let chunk = item.or(Err(format!("Error while downloading file")))?;
            file.write(&chunk)
                .or(Err(format!("Error while writing to file")))?;
        }

        // TODO: retry download if if file is invalid
        let png = PNG::open(&path);
        match png {
            Ok(..) => println!("Downloaded successfully"),
            Err(e) => println!("An error occurred: {}", e)
        }

        let mut fields: Vec<(String, String, bool)> = vec![];
        fields.push(
            (format!("Rarity"),
             format!("{}", card.rarity),
             false
            )
        );

        if let Some(price) = card.cardmarket.prices.average_sell_price {
            fields.push(
                (format!("Average Sell price"),
                 format!("{}€", price),
                 false
                )
            );
        }

        fields.push(
            (format!("Set"),
             format!("{}", card.set.name),
             false
            )
        );

        let colour = colour_map(card.types.first().unwrap().as_str());

        ctx.send(|b| {
            b.embed(|b| {
                b.colour(colour)
                    .title(&card.name)
                    .image(format!("attachment://{}", file_name))
                    .fields(fields)
            });
            b.attachment(AttachmentType::Path(Path::new(&path)))
        }).await?;
    }

    Ok(())
}

async fn autocomplete_pokemon<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(pokemon())
        .filter(move |pkmn| futures::future::ready(pkmn.starts_with(partial)))
        .map(|name| name.to_string())
}

async fn autocomplete_sets<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(set_names())
        .filter(move |set| futures::future::ready(set.starts_with(partial)))
        .map(|name| name.to_string())
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

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                pokemon_set(),
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
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::serenity_prelude::GuildId(844882826930421800)
                    .set_application_commands(ctx, |b| {
                        *b = poise::builtins::create_application_commands(
                            &framework.options().commands,
                        );
                        b
                    })
                    .await
                    .unwrap();
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}