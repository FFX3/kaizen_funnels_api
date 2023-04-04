use diesel::prelude::*;
use crate::schema::gatekeeper::users;

#[derive(Queryable, Identifiable, Selectable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub organization_id: i32
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
}

