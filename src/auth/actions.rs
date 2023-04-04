use pwhash::bcrypt;
use diesel::prelude::*;
use crate::schema::gatekeeper::users;
use crate::database::establish_connection;
use crate::auth::models::{NewUser, User};

use super::requests::NewUserRequest;

fn hash(input: &str) -> String {
    bcrypt::hash(input).unwrap()
}

fn verify(input: &str, hashed: &str) -> bool {
    bcrypt::verify(input, hashed)
}


pub fn create_user(new_user_request: NewUserRequest) -> User {
    let conn = &mut establish_connection();
    let user = NewUser {
        email: new_user_request.email,
        password_hash: hash(&new_user_request.password)
    };
    diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)
        .expect("Error creating user")
}

pub fn create_authorization_token() {

}

pub fn generate_access_token() {

}

pub fn parse_access_token() {

}
