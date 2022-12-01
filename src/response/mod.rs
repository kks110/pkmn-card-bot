use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CardData {
    pub data: Vec<Card>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub rarity: String,
    pub images: Images,
    pub set: Set,
    pub cardmarket: Cardmarket,
    pub types: Vec<String>
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
    pub average_sell_price: Option<f32>
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
