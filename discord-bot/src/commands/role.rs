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
            .say(&ctx.http, "ロールはすでに存在してますよ〜")
            .await?;
        println!("Role already exists");
        Ok(())
    } else {
        let role = guild
            .create_role(&ctx.http, |r| r.name("100円朝食").colour(8210593))
            .await
            .unwrap();

        msg.channel_id
            .say(
                &ctx.http,
                format!("{}というロールを作成したよ！", role.name),
            )
            .await?;
        println!("Created role {}", &role.name);
        Ok(())
    }
}
