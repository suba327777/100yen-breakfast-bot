use crate::google::event::{fetch_schedule, CalendarEvent};
use crate::utils::format::date;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "fetch schedule week"]
async fn sch(ctx: &Context, msg: &Message) -> CommandResult {
    let event: CalendarEvent = fetch_schedule().await;

    let event_message = date(event);

    msg.channel_id
        .say(
            &ctx.http,
            format!("予定されている日程はこちらになるよ！\n{}", event_message),
        )
        .await?;

    Ok(())
}
