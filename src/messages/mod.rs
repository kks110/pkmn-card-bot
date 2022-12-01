use crate::Context;
use crate::Error;

pub async fn send_error_message<D: ToString, T, U, It>(ctx: Context<'_>, title: D, fields: It) -> Result<(), Error>
    where
        It: IntoIterator<Item = (T, U, bool)>,
        T: ToString,
        U: ToString,
{
    ctx.send(|b| {
        b.embed(|b| {
            b.colour(0xcc0000)
                .title(title)
                .fields(fields)
        })
    }).await?;

    Ok(())
}