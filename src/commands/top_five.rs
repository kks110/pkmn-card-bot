use crate::Context;
use crate::Error;
use crate::url;
use crate::response::{CardData, ApiError};
use crate::messages;
use crate::helpers::*;

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
    card_name: Option<String>,
) -> Result<(), Error> {
    if card_name.is_none() && set.is_none() {
        let fields: Vec<(String, String, bool)> = vec![
            ("Error:".to_string(), "Please enter either a Pokemon / Trainer or a Set".to_string(), false),
        ];

        messages::send_message(ctx, "An error occurred!", fields, true).await?;

        return Ok(())
    }

    let url = url::find_top_five(&card_name, &set);

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
            ("Card Name".to_string(), card_name),
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
