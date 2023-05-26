use futures::{Stream, StreamExt};
use crate::Context;
use crate::tcg;
use crate::response::Card;

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
    if let Some(price) = card.cardmarket.prices.low_price_ex_plus {
        price_data.push_str(&format!("⦁ Lowest price (EX and higher): {}€\n", price));
    }

    if let Some(price) = card.cardmarket.prices.avg_30 {
        price_data.push_str(&format!("⦁ 30 day average: {}€\n", price));
    }

    if let Some(price) = card.cardmarket.prices.trend_price {
        price_data.push_str(&format!("⦁ Trend Price: {}€\n", price));
    }

    if let Some(price) = card.cardmarket.prices.reverse_holo_avg_30 {
        if price > 0.0 {
            price_data.push_str(&format!("⦁ 30 day reverse holo average: {}€\n", price));
        }
    }
    price_data
}