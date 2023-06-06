use chrono::{NaiveDate, Utc, Duration};
use serde::{Deserialize, Serialize};

// Structs for shared app data
#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub data_version: f32,
    pub euro_conversion_rate: f64,
    pub date_of_rate: NaiveDate
}

impl Data {
    pub fn new() -> Data {
        let yesterday = Utc::now().date_naive() - Duration::days(1);

        Data {
            data_version: crate::CURRENT_DATA_VERSION,
            date_of_rate: yesterday,
            euro_conversion_rate: 0.0
        }
    }

    pub fn set_date_of_rate(&mut self, date: NaiveDate) {
        self.date_of_rate = date;
    }

    pub fn set_euro_conversion_rate(&mut self, rate: f64) {
        self.euro_conversion_rate = rate;
    }
}
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