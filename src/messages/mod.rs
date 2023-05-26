use crate::Context;
use crate::Error;
use poise::serenity_prelude::AttachmentType;
use std::path::Path;
use poise::serenity_prelude::Colour;
use crate::response::ApiError;

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
