use futures::{Stream, StreamExt};
use crate::Context;
use crate::tcg;
use crate::models::Card;
use std::{
    fs::File,
    path::Path,
};
use std::io::{Read, Write};
use crate::Error;
use crate::models::{Data, CurrencyData};
use chrono::Utc;

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

pub fn price_data_string_builder(card: &Card, conversion_rate: f64) -> String {
    let mut price_data: String = "".to_string();
    if let Some(price) = card.cardmarket.prices.low_price_ex_plus {
        price_data.push_str(&format!("⦁ Lowest price (EX and higher): £{:.2}\n", (price as f64 * conversion_rate)));
    }

    if let Some(price) = card.cardmarket.prices.avg_30 {
        price_data.push_str(&format!("⦁ 30 day average: £{:.2}\n", (price as f64 * conversion_rate)));
    }

    if let Some(price) = card.cardmarket.prices.trend_price {
        price_data.push_str(&format!("⦁ Trend Price: £{:.2}\n", (price as f64 * conversion_rate)));
    }

    if let Some(price) = card.cardmarket.prices.reverse_holo_avg_30 {
        if price > 0.0 {
            price_data.push_str(&format!("⦁ 30 day reverse holo average: £{:.2}\n", (price as f64 * conversion_rate)));
        }
    }
    price_data
}

pub async fn load_app_data() -> Result<Data, Error> {
    let data_path = std::env::var("DATA_PATH").expect("Missing data path");
    let data_file_path = format!("{}{}", data_path, "data.json");

    if !Path::new(&data_file_path).exists() {
        println!("Data file doesn't exist. Creating...");
        let data = Data::new();
        save_app_data(&data, &data_file_path)
    }

    let mut data = load_app_data_file(&data_file_path);

    if data.data_version != crate::CURRENT_DATA_VERSION {
        println!("Data file incompatible. Is version {}, latest version is {}. Recreating...", data.data_version, crate::CURRENT_DATA_VERSION);
        let new_data = Data::new();
        save_app_data(&new_data, &data_file_path);
        data = load_app_data_file(&data_file_path);
    }

    if Utc::now().date_naive() != data.date_of_rate {
        let exchange_api_key = std::env::var("EXCHANGE_API_KEY").expect("Missing Exxchange rate API KEy");
        let url = format!("http://api.exchangeratesapi.io/v1/latest?access_key={}&format=1&symbols=GBP", exchange_api_key);
        let api_response = reqwest::get(url)
            .await?
            .text()
            .await?;
        let currency_data: CurrencyData = serde_json::from_str(&api_response)?;

        data.set_date_of_rate(currency_data.date);
        data.set_euro_conversion_rate(currency_data.rates.gbp);
        save_app_data(&data, &data_file_path);
    }

    Ok(data)
}

fn save_app_data(data: &Data, file_path: &str) {
    let mut file = File::create(file_path).expect("Failed to open the file");
    let json_data = serde_json::to_string(&data).expect("Failed to serialize JSON");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to the file");
}

fn load_app_data_file(file_path: &str) -> Data {
    let mut file = File::open(file_path).expect("Failed to open the file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Failed to read the file");
    serde_json::from_str(&json_data).expect("Failed to deserialize JSON")
}