use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
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
    let client = ctx.data.read().await;
    let client = client
        .get::<PostgresPool>()
        .ok_or(ReportableError::InternalError(
            "Database pool not in context",
        ))?;

    let pgversion = DatabaseClient::new(&client, command.guild_id.unwrap())
        .await?
        .get_version()
        .await?;

    command
        .create_interaction_response(&ctx.http, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| {
                    m.embed(|e| {
                        e.title("Processing")
                            .field("spectre", env!("CARGO_PKG_VERSION"), false)
                            .field("postgresql", &format!("{}", pgversion), false)
                            .description("Version info for spectre")
                    })
                })
        })
        .await?;
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("version")
        .description("Get postgresql version")
}
