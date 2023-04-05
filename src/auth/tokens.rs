use base64::{engine::general_purpose, Engine as _};
use rand::{distributions::Alphanumeric, Rng};
use crate::{auth::models::User, database::establish_connection, schema::gatekeeper::authorization_tokens};
use diesel::prelude::*;
use rocket::serde::json::serde_json;
use rocket::serde::{ Serialize, Deserialize };
use std::collections::HashMap;
use std::env;
use hmac::{Hmac, Mac};
use sha2::Sha256;

lazy_static! {
    static ref APP_SECRET: String = env::var("APP_SECRET").expect("APP_SECRET must be set");
}

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = authorization_tokens)]
pub struct AuthToken {
    pub id: i32,
    pub user_id: i32,
    pub permissions: serde_json::Value,
    pub key: String,
}

impl AuthToken {
    pub fn create_from_user(user: User, permissions: serde_json::Value) -> AuthToken {
        let conn = &mut establish_connection();
        #[derive(Queryable, Insertable, Selectable)]
        #[diesel(table_name = authorization_tokens)]
        struct NewAuthToken {
            user_id: i32,
            permissions: serde_json::Value,
            key: String,
        }
        diesel::insert_into(authorization_tokens::table)
            .values(NewAuthToken {
                user_id: user.id,
                permissions,
                key: generate_random_key(), 
            })
            .get_result(conn)
            .expect("Error creating user")
    }
}

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    user_id: i32,
    permissions: HashMap<String, serde_json::Value>
}

impl AccessToken {
    pub fn from_encoded(encoded_token: String) -> Result<AccessToken, &'static str> {
        // parse token
        Ok(AccessToken {
            user_id: 1,
            permissions: HashMap::new()
        })
    }

    pub fn to_encoded(&self) -> String {
        let payload = &serde_json::to_string(self).unwrap();
        let header = "{\"alg\": \"HS256\"}".to_owned();
        type HMACSHA256 = Hmac<Sha256>;
        let mut mac = HMACSHA256::new_from_slice(&(APP_SECRET.to_string().into_bytes())).unwrap();
        let claim = general_purpose::STANDARD.encode(header) 
            + "." 
            + &general_purpose::STANDARD.encode(payload);
        mac.update(&(&claim).to_owned().into_bytes());
        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());
        claim + "." + &signature
    }

    pub fn from_auth_token(auth_token: AuthToken) -> Self {
        let permissions = serde_json_value_to_hashmap(&auth_token.permissions).unwrap();
        Self  {
            user_id: auth_token.user_id,
            permissions,
        }
    }
}

fn generate_random_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(27)
        .map(char::from)
        .collect()
}

fn json_to_hashmap(json: &str) -> Result<HashMap<String, serde_json::Value>, &'static str> {
    let mut map: HashMap<String, serde_json::Value> = serde_json::from_str(json).unwrap();
    Ok(map)
}

fn serde_json_value_to_hashmap(serde_json_value: &serde_json::Value) -> Result<HashMap<String, serde_json::Value>, &'static str> {
    let mut map: HashMap<String, serde_json::Value> = serde_json::from_value(serde_json_value.to_owned()).unwrap();
    Ok(map)
}
