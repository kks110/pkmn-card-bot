use crate::database;
use crate::models::CurrencyData;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();


pub async fn get_initial_exchange_rate() {
    match database::get_exchange_rate("EUR") {
        Ok(_) => { println!("Exchange rate exists. Continuing...") }
        Err(e) => {
            if e.to_string() == "Record not found".to_string() {
                println!("Exchange rate does not exist, retrieving...");
                // TODO: Extract this as the logic is shared
                let exchange_api_key = std::env::var("EXCHANGE_API_KEY").expect("Missing Exxchange rate API KEy");
                let url = format!("http://api.exchangeratesapi.io/v1/latest?access_key={}&format=1&symbols=GBP", exchange_api_key);
                let api_response = reqwest::get(url)
                    .await
                    .expect("Failed API call")
                    .text()
                    .await
                    .expect("Failed API call");
                let currency_data: CurrencyData = serde_json::from_str(&api_response).expect("Failed to parse data");

                database::create_exchange_rate(&currency_data.base, currency_data.rates.gbp as f32)
            } else {
                panic!("Error occurred: {}", e.to_string())
            }
        }
    };
}

pub fn run_all_migrations() {
    println!("Running DB migrations");
    let mut conn = database::establish_connection();
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => println!("Migrations run successfully"),
        Err(e) => println!("Error running migrations: {}", e)
    }
}
