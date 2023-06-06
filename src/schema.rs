// @generated automatically by Diesel CLI.

diesel::table! {
    exchange_rates (id) {
        id -> Nullable<Integer>,
        base -> Text,
        gbp -> Float,
        updated_at -> Timestamp,
    }
}
