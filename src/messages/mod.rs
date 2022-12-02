use crate::Context;
use crate::Error;
use poise::serenity_prelude::AttachmentType;
use std::path::Path;
use poise::serenity_prelude::Colour;

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
