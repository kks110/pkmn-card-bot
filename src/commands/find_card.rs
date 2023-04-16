use crate::Context;
use crate::Error;
use crate::url;
use crate::response::{CardData, ApiError};
use crate::messages;
use crate::images;
use crate::tcg;

use std::{
    fs::File,
    path::Path,
};
use futures::{Stream, StreamExt};

#[poise::command(
    slash_command,
    rename = "find_cards"
)]
pub async fn execute(
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
    card_number: Option<String>,
) -> Result<(), Error> {
    let url = url::builder(&pokemon, &set, &card_number);

    let api_response = reqwest::get(url)
        .await?
        .text()
        .await?;

    // TODO: Make a fields builder
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
        messages::send_message(ctx, "An error occurred!", fields, true).await?;

        return Ok(())
    }

    let parsed_data: CardData = serde_json::from_str(&api_response)?;

    if parsed_data.data.is_empty() {
        let mut fields: Vec<(String, String, bool)> = vec![];

        fields.push(
            (format!("Pokemon"),
             format!("{}", &pokemon),
             false
            )
        );

        if let Some(card_number) = card_number {
            fields.push(
                (format!("Card Number"),
                 format!("{}", card_number),
                 false
                )
            );
        }

        if let Some(set) = set {
            fields.push(
                (format!("Set"),
                 format!("{}", &set),
                 false
                )
            );
        }

        messages::send_message(ctx, "Can't find a card with these options", fields, false).await?;
    }

    for card in parsed_data.data {
        let file_name = format!("{}.png", card.id);
        let path = format!("D:/Users/Kev/Pictures/{}", file_name);
        let url = &card.images.small;

        let mut file;

        if !Path::new(&path).exists() {
            println!("File doesnt exist. Creating");
            file = File::create(&path).or(Err(format!("Failed to create file '{}'", &path)))?;
            images::download_image(&url, &file).await?;
        }
        while images::PNG::open(&path).is_err() {
            println!("File has an error, trying again");
            std::fs::remove_file(&path)?;
            file = File::create(&path).or(Err(format!("Failed to create file '{}'", &path)))?;
            images::download_image(&url, &file).await?;
        }

        let mut fields: Vec<(String, String, bool)> = vec![];
        fields.push(
            (format!("Rarity"),
             format!("{}", card.rarity),
             false
            )
        );
        fields.push(
            (format!("Set"),
             format!("{}", card.set.name),
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

        let message_colour = tcg::colour_map(card.types.first().unwrap().as_str());

        messages::send_message_with_image(
            ctx,
            &card.name,
            fields,
            &file_name,
            &path,
            message_colour
        ).await?;
    }

    Ok(())
}

async fn autocomplete_pokemon<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(tcg::pokemon())
        .filter(move |pkmn| futures::future::ready(pkmn.starts_with(partial)))
        .map(|name| name.to_string())
}

async fn autocomplete_sets<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(tcg::set_names())
        .filter(move |set| futures::future::ready(set.starts_with(partial)))
        .map(|name| name.to_string())
}