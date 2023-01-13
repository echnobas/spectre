use std::io::Write;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;
use tokio_postgres::GenericClient;

use crate::PostgresPool;
use crate::error::ReportableError;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), ReportableError> {
    let group = match command.data.options.get(0).and_then(|r| r.resolved.as_ref()) {
        Some(CommandDataOptionValue::Integer(group)) => group,
        _ => return Err("Argument was not received".into())
    };
    command.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|m| m.embed(|e| e
            .title("Processing")
            .description("Please wait..")
        ))
    }).await?;

    let mut cursor = std::io::Cursor::new([0u8; 20]);

    let pool = match ctx.data.read().await.get::<PostgresPool>() {
        Some(v) => v.get().await.ok(),
        None => None
    }.ok_or(ReportableError::InternalError("Error getting database handle".into()))?;

    let guild_id = command.guild_id.unwrap().to_string();
    // Does schema exist?

    if pool.query_opt("SELECT schema_name FROM information_schema.schemata WHERE schema_name = CONCAT('data_', $1::text);", &[ &guild_id ]  ).await?.is_some() {
        command.create_followup_message(&ctx.http, |resp| {
            resp.embed(|e| e
                .title("Failure")
                .description(&format!("Server ({}) is already present in the database :-1:", command.guild_id.unwrap()))
            )
        }).await?;
    } else {
        pool.execute("call register_group($1::text);", &[&guild_id]).await?;
    
        command.create_followup_message(&ctx.http, |resp| {
            resp.embed(|e| e
                .title("Success")
                .description(&format!("Successfully registered your group ({group}) in the database :+1:", ))
            )
        }).await?;
    }
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("register").description("Register a roblox group").create_option(|option| {
        option
            .name("register")
            .description("group id")
            .kind(CommandOptionType::Integer)
            .required(true)
    })
}
