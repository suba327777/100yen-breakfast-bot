use crate::google::auth::fetch_access_token;
use dotenvy::dotenv;
use reqwest::header;
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize)]
pub struct CalendarEvent {
    pub items: Vec<EventItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventItem {
    pub summary: String,
    pub original_start_time: Option<OriginalStartTime>,
    pub start: EventItemPeriod,
    pub end: EventItemPeriod,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginalStartTime {
    pub date_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventItemPeriod {
    pub date_time: Option<String>,
    pub date: Option<String>,
}

pub async fn fetch_schedule() -> CalendarEvent {
    dotenv().ok();
    let acces_token = fetch_access_token().await;
    let calendar_id = env::var("CALEANDAR_ID").expect("Expected a calendarId in the env");

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("OAuth {}", acces_token)).unwrap(),
    );

    let response = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
        .get(&format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events",
            calendar_id
        ))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    serde_json::from_str(&response).unwrap()
}
