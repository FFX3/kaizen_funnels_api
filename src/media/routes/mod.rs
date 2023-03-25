use rocket::Route;
use rocket::serde::json::Json;
use rocket::serde::{
    Deserialize,
    Serialize
};
use cloud_storage::Client;
use base64::{Engine, engine::general_purpose};

pub fn get_media_routes() -> Vec<Route> {
    routes![
        convert_base64_media,
        upload,
    ]
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct MediaConversionRequest {
    base64: String,
    file_name: String,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct MediaConversionResponse {
    bucket_url: String
}

#[post("/base64/upload", data = "<media_conversion_request>")]
async fn convert_base64_media(media_conversion_request: Json<MediaConversionRequest>) -> Json<MediaConversionResponse> {
    let client = Client::default();
    let encoded_media: Vec<&str> = media_conversion_request.base64.split(";base64,").collect();
    let split_start: Vec<&str> = encoded_media[0].split(":").collect();
    let mime_type = split_start[1];

    let bytes = general_purpose::STANDARD
        .decode(encoded_media[1]).unwrap();
    let file_path = "kaizen_marketing/".to_owned() + &media_conversion_request.file_name;
    //TODO fix unwrap
    let object = client.object().create("funnel-cms", bytes, &file_path, mime_type).await.unwrap();

    Json(MediaConversionResponse{
        bucket_url: object.media_link,
    })
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
struct UploadRequest {
    files: Vec<String>
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
struct UploadResponse {
    data: Vec<String>
}
#[post("/upload", data = "<upload_request>")]
async fn upload(upload_request: Json<UploadRequest>) -> Json<UploadResponse> {
    println!("{:?}", upload_request.into_inner());
    Json(UploadResponse { data: vec!["bingo".to_owned()] })
}
