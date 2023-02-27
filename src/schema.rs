// @generated automatically by Diesel CLI.

diesel::table! {
    funnels (id) {
        id -> Int4,
        label -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
