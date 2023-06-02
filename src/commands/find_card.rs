use crate::Context;
use crate::Error;
use crate::url;
use crate::response::{CardData, ApiError};
use crate::messages;
use crate::helpers::*;


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
    card_name: String,
    #[description = "The card number in the set"]
    #[lazy]
    card_number: Option<String>,
) -> Result<(), Error> {
    let url = url::find_card(&card_name, &set, &card_number);

    let api_response = reqwest::get(url)
        .await?
        .text()
        .await?;

    if api_response.contains("\"error\":") {
        let parsed_data: ApiError = serde_json::from_str(&api_response)?;

        let fields: Vec<(String, String, bool)> = vec![
            ("Code".to_string(), parsed_data.error.code.to_string(), false),
            ("Message".to_string(), parsed_data.error.message, false),
        ];

        messages::send_message(ctx, "An error occurred!", fields, true).await?;

        return Ok(())
    }

    let parsed_data: CardData = serde_json::from_str(&api_response)?;

    if parsed_data.data.is_empty() {
        let fields: Vec<(String, Option<String>)> = vec![
            ("Card Name".to_string(), Some(card_name)),
            ("Card Number".to_string(), card_number),
            ("Set".to_string(), set)
        ];

        let fields = messages::process_option_fields(fields);

        messages::send_message(ctx, "Can't find a card with these options", fields, false).await?;
    }

    for card in parsed_data.data {
        messages::send_card_message(ctx, card).await?;
    }

    Ok(())
}
