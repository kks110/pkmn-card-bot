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
rename = "top_five"
)]
pub async fn execute(
    ctx: Context<'_>,
    #[description = "Which Set"]
    #[autocomplete = "autocomplete_sets"]
    #[lazy]
    set: Option<String>,
    #[description = "Which Pokemon / Trainer"]
    #[autocomplete = "autocomplete_pokemon"]
    #[lazy]
    pokemon: Option<String>,
) -> Result<(), Error> {
    let image_root = std::env::var("IMAGE_PATH").expect("missing image path");
    println!("before the none check");
    if pokemon.is_none() && set.is_none() {
        let mut fields: Vec<(String, String, bool)> = vec![];

        fields.push(
            (format!("Error:"),
             format!("Please enter either a Pokemon or a Set"),
             false
            )
        );
        messages::send_message(ctx, "An error occurred!", fields, true).await?;

        return Ok(())
    }

    println!("generating URL");
    let url = url::find_top_five(&pokemon, &set);

    println!("URL: {}", url);
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

    println!("parsing data");
    println!("{:?}", api_response);
    let parsed_data: CardData = serde_json::from_str(&api_response)?;
    println!("parsed");

    if parsed_data.data.is_empty() {
        println!("ITS EMPTY!");
        let mut fields: Vec<(String, String, bool)> = vec![];


        if let Some(pokemon) = pokemon {
            fields.push(
                (format!("Pokemon"),
                 format!("{}", pokemon),
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
        println!("looping cards");
        let file_name = format!("{}.png", card.id);
        let path = format!("{}{}", image_root, file_name);
        let url = &card.images.large;

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

        let mut price_data: String = "".to_string();
        if let Some(price) = card.cardmarket.prices.low_price_ex_plus {
            price_data.push_str(&format!("⦁ Lowest price (EX and higher): {}€\n", price));
        }

        if let Some(price) = card.cardmarket.prices.avg_30 {
            price_data.push_str(&format!("⦁ 30 day average: {}€\n", price));
        }

        if let Some(price) = card.cardmarket.prices.reverse_holo_avg_30 {
            if price > 0.0 {
                price_data.push_str(&format!("⦁ 30 day reverse holo average: {}€\n", price));
            }
        }

        fields.push(
            (format!("Prices:"),
             price_data,
             false
            )
        );

        let message_colour: i32;
        if let Some(types) = card.types {
            message_colour = tcg::colour_map(types.first().unwrap().as_str());
        } else {
            message_colour = tcg::colour_map("Trainer");
        }

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