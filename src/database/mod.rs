use diesel::prelude::*;
use std::env;
use chrono::Utc;
use crate::Error;
use crate::models::{ExchangeRate, NewExchangeRate};

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("PKMN_CARD_BOT_DATABASE_URL").expect("PKMN_CARD_BOT_DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_exchange_rate(currency: &str) -> Result<ExchangeRate, Error> {
    use crate::schema::exchange_rates::dsl::*;

    let connection = &mut establish_connection();
    Ok(exchange_rates
        .filter(base.eq(currency.to_uppercase()))
        .first(connection)?
    )
}

pub fn update_exchange_rates(currency: &str, new_gbp: f32) {
    use crate::schema::exchange_rates::dsl::*;

    let conn = &mut establish_connection();

    diesel::update(exchange_rates.filter(base.eq(currency)))
        .set((
                 gbp.eq(new_gbp),
                 updated_at.eq(Utc::now().date_naive().to_string())
        ))
        .execute(conn)
        .expect("Error updating rate");
}

pub fn create_exchange_rate(base: &str, gbp: f32) {
    use crate::schema::exchange_rates;

    let conn = &mut establish_connection();

    let new_rate = NewExchangeRate { base, gbp };

    diesel::insert_into(exchange_rates::table)
        .values(&new_rate)
        .execute(conn)
        .expect("Error saving new rate");
}