use rocket::Route;
use rocket::serde::json::Json;
use rocket::serde::{
    Deserialize,
    Serialize
};
use cloud_storage::Client;

pub fn get_media_routes() -> Vec<Route> {
    routes![
        convert_base64_media,
    ]
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct MediaConversionRequest {
    base64: String
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct MediaConversionResponse {
    bucket_url: String
}

#[post("/", data = "<media_conversion_request>")]
async fn convert_base64_media(media_conversion_request: Json<MediaConversionRequest>) -> Json<MediaConversionResponse> {
    let client = Client::default();
    let bucket = client.bucket().read("funnel-cms").await.unwrap(); //fix unwrap on external api

    //TODO: https://www.reddit.com/r/rust/comments/bzacoc/question_converting_base64_representation_of_png/
    //https://docs.rs/cloud-storage/latest/cloud_storage/
    Json(MediaConversionResponse{
        bucket_url: media_conversion_request.base64.to_owned(),
    })
}
