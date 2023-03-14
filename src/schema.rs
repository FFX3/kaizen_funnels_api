// @generated automatically by Diesel CLI.

diesel::table! {
    contents (id) {
        id -> Int4,
        content -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        grapesjs -> Nullable<Text>,
    }
}

diesel::table! {
    funnels (id) {
        id -> Int4,
        label -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        slug -> Nullable<Varchar>,
    }
}

diesel::table! {
    steps (id) {
        id -> Int4,
        title -> Varchar,
        variation_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        content_id -> Int4,
        order -> Int4,
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

diesel::joinable!(steps -> contents (content_id));
diesel::joinable!(steps -> variations (variation_id));
diesel::joinable!(variations -> funnels (funnel_id));

diesel::allow_tables_to_appear_in_same_query!(
    contents,
    funnels,
    steps,
    variations,
);
