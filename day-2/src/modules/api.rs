use serde::{Deserialize, Serialize};
use dotenv::dotenv;
#[derive(Serialize, Deserialize, Debug)]
struct Request {
    base_url: String,
    param_key: String,
    param_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    result_url: String,
}

#[tokio::main]
pub async fn short_url(link: &str) -> String {
    dotenv().ok();
    let client = reqwest::Client::new();
    let base_url = std::env::var("BASE_URL").expect("Need some credentials for using this apps!");
    let req = Request {
        base_url: String::from(base_url),
        param_key: String::from("url"),
        param_value: String::from(link),
    };

    let request_with = [(req.param_key, req.param_value)];
    let api = client
        .post(req.base_url)
        .form(&request_with)
        .send()
        .await
        .unwrap();

    match api.status() {
        reqwest::StatusCode::OK => {
            match api.json::<ApiResponse>().await {
                Ok(parsed) => return String::from(parsed.result_url),
                Err(_) => return String::from("Gagal mendapatkan url baru, silahkan coba lagi."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            return String::from(
                "Server sibuk...Silahkan generate url dalam beberapa menit kedepan",
            );
        }
        other => {
            panic!("maaf terjadi kesalahan - {} ", other);
        }
    };
}
