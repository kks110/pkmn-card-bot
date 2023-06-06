// @generated automatically by Diesel CLI.

diesel::table! {
    exchange_rates (id) {
        id -> Integer,
        base -> Text,
        gbp -> Float,
        updated_at -> Text,
    }
}
