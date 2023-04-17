use crate::google::event::{fetch_schedule, CalendarEvent};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "fetch schedule week"]
async fn sch(ctx: &Context, msg: &Message) -> CommandResult {
    let event_message: String = fetch_schedule().await;

    msg.channel_id
        .say(
            &ctx.http,
            format!("日程はこちらになるよ！\n{}", event_message),
        )
        .await?;

    Ok(())
}
