// @generated automatically by Diesel CLI.

diesel::table! {
    authorization_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        permissions -> Jsonb,
        key -> Varchar,
    }
}

diesel::table! {
    organizations (id) {
        id -> Int4,
        name -> Varchar,
        database_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password_hash -> Varchar,
        organization_id -> Int4,
    }
}

diesel::joinable!(authorization_tokens -> users (user_id));
diesel::joinable!(users -> organizations (organization_id));

diesel::allow_tables_to_appear_in_same_query!(
    authorization_tokens,
    organizations,
    users,
);
