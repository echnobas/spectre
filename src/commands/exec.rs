use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOptionValue,
};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;

use anyhow::Result;
use crate::error::ReportableError;
use crate::PostgresPool;

pub async fn run(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), ReportableError> {
    let raw = match command
        .data
        .options
        .get(0)
        .and_then(|r| r.resolved.as_ref())
    {
        Some(CommandDataOptionValue::String(group)) => group,
        _ => return Err(ReportableError::InternalError("Argument was not received")),
    };

    let pool = match ctx.data.read().await.get::<PostgresPool>() {
        Some(v) => v.get().await.ok(),
        None => None,
    }
    .ok_or(ReportableError::InternalError(
        "Error getting database handle".into(),
    ))?;

    let rows = pool.query(raw, &[]).await?;

    command
        .create_interaction_response(&ctx.http, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| {
                    m.embed(|e| e.title("SQL").description(&format!("{:?}", rows)))
                })
        })
        .await?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("exec")
        .description("Execute raw SQL")
        .create_option(|option| {
            option
                .name("raw")
                .description("SQL to execute")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
