use crate::google::auth::fetch_access_token;
use dotenvy::dotenv;
use reqwest::header;
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize)]
pub struct CalendarEvent {
    items: Vec<EventItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventItem {
    summary: String,
    original_start_time: Option<OriginalStartTime>,
    start: EventItemPeriod,
    end: EventItemPeriod,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OriginalStartTime {
    date_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventItemPeriod {
    date_time: Option<String>,
    date: Option<String>,
}

pub async fn fetch_schedule() -> String {
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

    let events: CalendarEvent = serde_json::from_str(&response).unwrap();

    let mut event_message = String::new();

    for event in events.items {
        let start_time = match event.start.date_time {
            Some(d) => d.to_string(),
            None => event.start.date.as_ref().unwrap().to_string(),
        };

        let event_info = format!("{}\n", start_time);

        event_message.push_str(&event_info);
    }

    event_message
}
