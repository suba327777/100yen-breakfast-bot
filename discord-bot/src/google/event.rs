use crate::google::auth::fetch_access_token;
use crate::utils::date::date_now_jst;
use crate::utils::format::format_date;
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

pub enum EventTime {
    Start,
    End,
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

    let jst_now = date_now_jst();

    let response = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
        .get(&format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events",
            calendar_id
        ))
        .query(&[
            ("timeZone", "jst"),
            ("timeMin", &jst_now.to_rfc3339()),
            ("singleEvents", "true"),
            ("orderBy", "startTime"),
        ])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    get_event_message(serde_json::from_str(&response).unwrap())
}

fn get_event_message(events: CalendarEvent) -> String {
    let mut event_message = "予定されている日程はこちらになるよ!\n".to_string();

    if events.items.len() > 0 {
        for event in events.items {
            let start_time = match event.start.date_time {
                Some(d) => format_date(d.to_string(), EventTime::Start),
                None => format_date(
                    event.start.date.as_ref().unwrap().to_string(),
                    EventTime::Start,
                ),
            };
            let end_time = match event.end.date_time {
                Some(d) => format_date(d.to_string(), EventTime::End),
                None => format_date(event.end.date.as_ref().unwrap().to_string(), EventTime::End),
            };

            let event_info = format!("{} ~ {}\n", start_time, end_time);

            event_message.push_str(&event_info);
        }
    } else {
        event_message = "予定は入っていないみたいです😢".to_string();
    }

    event_message
}
