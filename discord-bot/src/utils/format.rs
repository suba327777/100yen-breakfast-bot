use crate::google::event::EventTime;
use chrono::DateTime;

pub fn format_date(date_time: String, event_time: EventTime) -> String {
    let parse_date_time = DateTime::parse_from_rfc3339(&date_time).unwrap();

    match event_time {
        EventTime::Start => parse_date_time.format("%m月%d日 %H時%M分").to_string(),
        EventTime::End => parse_date_time.format("%H時%M分").to_string(),
    }
}
