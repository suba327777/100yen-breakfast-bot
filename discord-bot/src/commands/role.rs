use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("create a role")]
async fn create_role(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let is_roles = guild.roles.values().any(|r| r.name == "100円朝食");

    if is_roles {
        msg.channel_id
            .say(&ctx.http, "ロールはすでに存在してますよ😏")
            .await?;

        Ok(())
    } else {
        let role = guild
            .create_role(&ctx.http, |r| r.name("100円朝食").colour(0x90EE90))
            .await
            .unwrap();

        msg.channel_id
            .say(
                &ctx.http,
                format!("{}というロールを作成したよ！✌️", role.name),
            )
            .await?;

        Ok(())
    }
}

#[command]
#[description("add a role to user")]
async fn add_role(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let role_id = match guild.roles.values().find(|r| r.name == "100円朝食") {
        Some(role) => role.id,
        None => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "ロールが存在していません😅\n!add_roleでロールを作成してください！",
                )
                .await?;

            return Ok(());
        }
    };

    let user_id = msg.author.id;
    let mut member = guild.member(&ctx.http, user_id).await.unwrap();

    if member.roles.contains(&role_id) {
        msg.channel_id
            .say(&ctx.http, "すでにロールは付与されていますよ😏")
            .await?;

        return Ok(());
    }

    if let Err(why) = member.add_role(&ctx.http, &role_id).await {
        msg.channel_id
            .say(&ctx.http, format!("ロールの付与に失敗しました😓 {:?}", why))
            .await?;

        return Ok(());
    }

    msg.channel_id
        .say(&ctx.http, "ロールを付与しました！✌️")
        .await?;

    Ok(())
}
