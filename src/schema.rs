// @generated automatically by Diesel CLI.

diesel::table! {
    verify_tx (id) {
        id -> Integer,
        to_address -> Nullable<Text>,
        from_address -> Nullable<Text>,
        tx_hash -> Nullable<Text>,
        verify_status -> Nullable<Integer>,
    }
}
