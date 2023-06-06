use std::io::ErrorKind;
use chrono::{NaiveDate, Utc};
use futures::{Stream, StreamExt};
use crate::{Context, Error};
use crate::tcg;
use crate::models::{Card, CurrencyData, Data, ExchangeRate};
use crate::database;

pub async fn autocomplete_pokemon<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(tcg::pokemon())
        .filter(move |pkmn| futures::future::ready(pkmn.starts_with(partial)))
        .map(|name| name.to_string())
}

pub async fn autocomplete_sets<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(tcg::set_names())
        .filter(move |set| futures::future::ready(set.starts_with(partial)))
        .map(|name| name.to_string())
}

pub fn price_data_string_builder(card: &Card) -> String {
    let mut price_data: String = "".to_string();

    let conversion_rate = database::get_exchange_rate("EUR").unwrap();

    if let Some(price) = card.cardmarket.prices.low_price_ex_plus {
        price_data.push_str(&format!("⦁ Lowest price (EX and higher): £{:.2}\n", (price * conversion_rate.gbp)));
    }

    if let Some(price) = card.cardmarket.prices.avg_30 {
        price_data.push_str(&format!("⦁ 30 day average: £{:.2}\n", (price * conversion_rate.gbp)));
    }

    if let Some(price) = card.cardmarket.prices.trend_price {
        price_data.push_str(&format!("⦁ Trend Price: £{:.2}\n", (price * conversion_rate.gbp)));
    }

    if let Some(price) = card.cardmarket.prices.reverse_holo_avg_30 {
        if price > 0.0 {
            price_data.push_str(&format!("⦁ 30 day reverse holo average: £{:.2}\n", (price * conversion_rate.gbp)));
        }
    }
    price_data
}
