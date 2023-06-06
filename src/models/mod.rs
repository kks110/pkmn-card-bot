use chrono::{NaiveDate, Utc, Duration};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::*;

// Structs for Exchange Rate database data
#[derive(Queryable, Selectable)]
#[diesel(table_name = exchange_rates)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ExchangeRate {
    pub id: i32,
    pub base: String,
    pub gbp: f32,
    pub updated_at: String
}

#[derive(Insertable)]
#[diesel(table_name = exchange_rates)]
pub struct NewExchangeRate<'a> {
    pub base: &'a str,
    pub gbp: f32,
}
// -------------------------------------

// Structs for shared app data
#[derive(Debug)]
pub struct Data {}
// -------------------------------------


// Structs for the Currenct Exchange API
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Rates {
    pub gbp: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrencyData {
    pub success: bool,
    pub timestamp: i64,
    pub base: String,
    pub date: NaiveDate,
    pub rates: Rates,
}
// -------------------------------------


// Structs for the Pokemon API
#[derive(Serialize, Deserialize, Debug)]
pub struct CardData {
    pub data: Vec<Card>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub rarity: Option<String>,
    pub images: Images,
    pub set: Set,
    pub cardmarket: Cardmarket,
    pub types: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Images {
    pub small: String,
    pub large: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
    pub id: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cardmarket {
    pub prices: Prices
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Prices {
    pub avg_30: Option<f32>,
    pub reverse_holo_avg_30: Option<f32>,
    pub low_price_ex_plus: Option<f32>,
    pub trend_price: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub error: ApiErrorData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiErrorData {
    pub message: String,
    pub code: i32
}
// -------------------------------------