// @generated automatically by Diesel CLI.

diesel::table! {
    funnels (id) {
        id -> Int4,
        label -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    variations (id) {
        id -> Int4,
        label -> Varchar,
        funnel_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        mark_a -> Nullable<Timestamp>,
        mark_b -> Nullable<Timestamp>,
    }
}

diesel::joinable!(variations -> funnels (funnel_id));

diesel::allow_tables_to_appear_in_same_query!(
    funnels,
    variations,
);
