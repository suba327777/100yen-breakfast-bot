use crate::google::event::fetch_schedule;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "fetch schedule from google calendar"]
async fn sch(ctx: &Context, msg: &Message) -> CommandResult {
    let event_message: String = fetch_schedule().await;

    msg.channel_id
        .say(&ctx.http, event_message.to_string())
        .await?;

    Ok(())
}
