use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::File;
use std::io::BufReader;

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

impl Claims {
    fn new(client_email: String, token_uri: String) -> Self {
        //generate timestamps of JWT issue time and expiration date
        let now = Utc::now();
        let iat = now.timestamp();
        let exp = (now + Duration::minutes(60)).timestamp();

        Claims {
            iss: client_email,
            scope: SCOPE.to_string(),
            aud: token_uri,
            iat,
            exp,
        }
    }
}

pub async fn fetch_access_token() -> String {
    let file = File::open(FILE_NAME).unwrap();
    let credential: Credential = serde_json::from_reader(BufReader::new(file)).unwrap();
    let token_uri = credential.token_uri.clone();
    let jwt = generate_jwt(credential);

    let token_body = json!({
        "grant_type":"urn:ietf:params:oauth:grant-type:jwt-bearer",
        "assertion":jwt,
    });

    let token_response = Client::new()
        .post(&token_uri)
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

fn generate_jwt(credential: Credential) -> String {
    let claims = Claims::new(credential.client_email, credential.token_uri);

    let mut header = Header::default();
    header.typ = Some("JWT".to_string());
    header.alg = Algorithm::RS256;

    return encode(
        &header,
        &claims,
        &EncodingKey::from_rsa_pem(credential.private_key.as_bytes()).unwrap(),
    )
    .unwrap();
}
