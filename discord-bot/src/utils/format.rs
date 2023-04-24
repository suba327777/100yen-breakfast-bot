use crate::google::event::CalendarEvent;

pub fn date(events: CalendarEvent) -> String {
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
