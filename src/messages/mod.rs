use crate::{Context, database, images, tcg};
use crate::Error;
use poise::serenity_prelude::AttachmentType;
use std::path::Path;
use poise::serenity_prelude::Colour;
use crate::helpers::price_data_string_builder;
use crate::models::Card;

pub fn process_option_fields(fields: Vec<(String, Option<String>)>) -> Vec<(String, String, bool)> {
    let mut processed_fields: Vec<(String, String, bool)> = vec![];
    for (title, body) in fields {
        if let Some(body) = body {
            processed_fields.push(
                (
                    title,
                    body,
                    false
                )
            )
        }
    }
    processed_fields
}

pub async fn send_message<
    D: ToString,
    T,
    U,
    It
>(ctx: Context<'_>, title: D, fields: It, error: bool) -> Result<(), Error>
    where
        It: IntoIterator<Item = (T, U, bool)>,
        T: ToString,
        U: ToString,
{
    let colour = if error { 0xcc0000 } else { 0xcc8800 };

    ctx.send(|b| {
        b.embed(|b| {
            b.colour(colour)
                .title(title)
                .fields(fields)
        })
    }).await?;

    Ok(())
}

pub async fn send_message_with_image<D: ToString, T, U, It, C: Into<Colour>>(
    ctx: Context<'_>,
    title: D,
    fields: It,
    file_name: &str,
    file_path: &str,
    message_colour: C
) -> Result<(), Error>
    where
        It: IntoIterator<Item = (T, U, bool)>,
        T: ToString,
        U: ToString,
{
    ctx.send(|b| {
        b.embed(|b| {
            b.colour(message_colour)
                .title(title)
                .image(format!("attachment://{}", file_name))
                .fields(fields)
        });
        b.attachment(AttachmentType::Path(Path::new(file_path)))
    }).await?;

    Ok(())
}

pub async fn send_card_message(ctx: Context<'_>, card: Card) -> Result<(), Error> {
    let file_name = format!("{}.png", card.id);
    let url = &card.images.large;
    let image_file_path = images::download_image(&file_name, url).await?;

    let price_data: String = price_data_string_builder(&card);

    let fields: Vec<(String, Option<String>)> = vec![
        ("Rarity".to_string(), card.rarity),
        ("Set".to_string(), Some(card.set.name)),
        ("Prices:".to_string(), Some(price_data)),
    ];

    let fields = process_option_fields(fields);

    let message_colour: i32;
    if let Some(types) = card.types {
        message_colour = tcg::colour_map(types.first().unwrap().as_str());
    } else {
        message_colour = tcg::colour_map("Trainer");
    }

    send_message_with_image(
        ctx,
        &card.name,
        fields,
        &file_name,
        &image_file_path,
        message_colour
    ).await?;

    Ok(())
}