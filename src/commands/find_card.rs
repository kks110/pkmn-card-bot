use crate::Context;
use crate::Error;
use crate::url;
use crate::response::{CardData, ApiError};
use crate::messages;
use crate::images;
use crate::tcg;
use crate::helpers::*;

use std::{
    fs::File,
    path::Path,
};

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
    #[description = "Which Pokemon / Trainer"]
    #[autocomplete = "autocomplete_pokemon"]
    pokemon: String,
    #[description = "The card number in the set"]
    #[lazy]
    card_number: Option<String>,
) -> Result<(), Error> {
    let image_root = std::env::var("IMAGE_PATH").expect("missing image path");

    let url = url::find_card(&pokemon, &set, &card_number);

    let api_response = reqwest::get(url)
        .await?
        .text()
        .await?;

    // TODO: Make a fields builder
    if api_response.contains("\"error\":") {
        let parsed_data: ApiError = serde_json::from_str(&api_response)?;
        messages::send_error_message(ctx, parsed_data).await?;

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
        let url = &card.images.large;

        let file_path = images::download_image(&file_name, &url).await?;

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

        let mut price_data: String = price_data_string_builder(&card);

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
            &file_path,
            message_colour
        ).await?;
    }

    Ok(())
}
