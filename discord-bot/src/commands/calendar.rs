use chrono::{Duration, Utc};
use google_calendar3::api::Event;
use google_calendar3::CalendarHub;
// use hyper;
// use hyper_rustls;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fs::File;
use std::io::BufReader;
use yup_oauth2::{read_service_account_key, ServiceAccountAuthenticator};

const FILE_NAME: &str = "credentials.json";
const SCOPE: &str = "https://www.googleapis.com/auth/calendar";

#[derive(Debug, Serialize, Deserialize)]
struct Credential {
    r#type: String,
    project_id: String,
    private_key: String,
    client_email: String,
    client_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_x509_cert_url: String,
}

#[derive(Debug, Serialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: i64,
    iat: i64,
}

async fn fetch_token() -> String {
    let file = File::open(FILE_NAME).unwrap();

    let credential: Credential = serde_json::from_reader(BufReader::new(file)).unwrap();

    //generate timestamps of JWT issue time and expiration date
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::minutes(60)).timestamp();

    let mut header = Header::default();
    header.typ = Some("JWT".to_string());
    header.alg = Algorithm::RS256;

    let claims = Claims {
        iss: credential.client_email,
        scope: SCOPE.to_string(),
        aud: credential.token_uri,
        exp,
        iat,
    };

    let jwt = encode(
        &header,
        &claims,
        &EncodingKey::from_rsa_pem(credential.private_key.as_bytes()).unwrap(),
    )
    .unwrap();

    let token_body = json!({
        "grant_type":"urn:ietf:params:oauth:grant-type:jwt-bearer",
        "assertion":jwt,
    });

    let token_response = Client::new()
        .post(&claims.aud)
        .json(&token_body)
        .send()
        .unwrap();

    let token_response_body: Value = token_response.json().unwrap();
    let access_token = token_response_body
        .get("access_token")
        .unwrap()
        .as_str()
        .unwrap();

    return access_token.to_string();
}

#[command]
#[description = "fetch schedule week"]
async fn fetch_schedule(ctx: &Context, msg: &Message) -> CommandResult {
    let token = fetch_token().await;
    msg.channel_id
        .say(&ctx.http, format!("カレンダーだよーん",))
        .await?;

    Ok(())
}
// let hub = CalendarHub::new(
//     hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots),
//     key,
// );

// let google_calendar = client::new(string::from(),String::from()
// Client::new();
