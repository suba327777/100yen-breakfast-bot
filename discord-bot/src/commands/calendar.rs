use serde::{Deserialize, Serialize};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
struct SecretJson {
    // project_id: String,
    private_key: String,
    client_email: String,
    client_id: String,
    auth_uri: String,
    token_uri: String,
    // auth_provider_x509_cert_uri: String,
    // client_x509_cert_uri: String,
}

fn setting() {
    let file_name = "credentials.json";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let serde: SecretJson = serde_json::from_reader(reader).unwrap();
    println!("{:?}", serde.client_id);

    // let google_calendar = client::new(string::from());
}

#[command]
#[description = "fetch schedule week"]
async fn fetch_schedule(ctx: &Context, msg: &Message) -> CommandResult {
    setting();
    msg.channel_id
        .say(&ctx.http, format!("カレンダーだよーん",))
        .await?;

    Ok(())
}
