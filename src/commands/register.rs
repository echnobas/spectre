use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;

use crate::database::DatabaseClient;
use crate::error::ReportableError;
use crate::PostgresPool;
use anyhow::Result;

pub async fn run(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), ReportableError> {
    let group = match command
        .data
        .options
        .get(0)
        .and_then(|r| r.resolved.as_ref())
    {
        Some(CommandDataOptionValue::Integer(group)) => group,
        _ => return Err(ReportableError::InternalError("Argument was not received")),
    };
    command
        .create_interaction_response(&ctx.http, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| {
                    m.embed(|e| e.title("Processing").description("Please wait.."))
                })
        })
        .await?;

    let client = ctx.data.read().await;
    let client = client
        .get::<PostgresPool>()
        .ok_or(ReportableError::InternalError(
            "Database pool not in context",
        ))?;
    let client = DatabaseClient::new(client, command.guild_id.unwrap()).await?;

    if client.register_group().await? {
        command
            .create_followup_message(&ctx.http, |resp| {
                resp.embed(|e| {
                    e.title("Failure").description(&format!(
                        "Server ({}) is already present in the database :-1:",
                        command.guild_id.unwrap()
                    ))
                })
            })
            .await?;
    } else {
        command
            .create_followup_message(&ctx.http, |resp| {
                resp.embed(|e| {
                    e.title("Success").description(&format!(
                        "Successfully registered your group ({group}) in the database :+1:",
                    ))
                })
            })
            .await?;
    }
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("register")
        .description("Register a roblox group")
        .create_option(|option| {
            option
                .name("register")
                .description("group id")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}
