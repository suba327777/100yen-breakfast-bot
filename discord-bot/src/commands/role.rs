use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("create a role")]
async fn create_role(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    // let roles = guild
    //     .roles
    //     .value
    // println!("{}", response);
    let role = guild
        .create_role(&ctx.http, |r| r.name("100円朝食"))
        .await
        .unwrap();
    msg.channel_id
        .say(&ctx.http, format!("Created role {}", role.name))
        .await?;
    println!("Created role {}", &role.name);
    Ok(())
}
