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
pub struct EventItem {
    summary: String,
    OriginalStartTime: Option<OriginalStartTime>,
    start: EventItemPeriod,
    end: EventItemPeriod,
}

#[derive(Debug, Serialize, Deserialize)]
struct OriginalStartTime {
    dateTime: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EventItemPeriod {
    dateTime: Option<String>,
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
        let start_time = match event.start.dateTime {
            Some(d) => d.to_string(),
            None => event.start.date.as_ref().unwrap().to_string(),
        };

        let event_info = format!("{}\n", start_time);

        event_message.push_str(&event_info);
    }

    return event_message;
}
