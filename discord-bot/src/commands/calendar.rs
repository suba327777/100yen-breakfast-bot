use crate::google::auth::fetch_access_token;
use dotenvy::dotenv;
use reqwest::blocking::Client;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

#[command]
#[description = "fetch schedule week"]
async fn fetch_schedule(ctx: &Context, msg: &Message) -> CommandResult {
    dotenv().ok();
    let acces_token = fetch_access_token().await;
    let calendar_id = env::var("CALEANDAR_ID").expect("Expected a calendarId in the env");

    let response = Client::new()
        .get(&format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events",
            calendar_id
        ))
        .bearer_auth(acces_token)
        .send()
        .unwrap();

    println!("{:?}", response);
    println!("{:?}", response.text());
    msg.channel_id
        .say(&ctx.http, format!("カレンダーだよーん",))
        .await?;

    Ok(())
}
