use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
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
        .await
        .unwrap();

    let token_response_body: Value = token_response.json().await.unwrap();

    token_response_body.get("access_token").unwrap().to_string()
}

fn generate_jwt(credential: Credential) -> String {
    let claims = Claims::new(credential.client_email, credential.token_uri);

    let header = Header {
        typ: Some("JWT".to_string()),
        alg: Algorithm::RS256,
        ..Default::default()
    };

    return encode(
        &header,
        &claims,
        &EncodingKey::from_rsa_pem(credential.private_key.as_bytes()).unwrap(),
    )
    .unwrap();
}

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use std::fs;

//     const TEST_FILE_NAME: &str = "test_credential.json";
//     const TEST_CREDENTIAL_JSON: &str = r#"{
//         "client_email": "test@exsample.com",
//         "token_uri": "https://exsample.com/token",
//         "private_key": "test_private_key" }"#;

//     #[tokio::test]
//     async fn test_fetch_access_token() {
//         fs::write(TEST_FILE_NAME, TEST_CREDENTIAL_JSON).unwrap();

//         let access_token: String = fetch_access_token().await;

//         assert!(access_token.len() > 0);

//         fs::remove_file(TEST_FILE_NAME).unwrap();
//     }
// }
