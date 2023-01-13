use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;

use crate::PostgresPool;
use crate::error::ReportableError;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), ReportableError> {
    let pool = match ctx.data.read().await.get::<PostgresPool>() {
        Some(v) => v.get().await.ok(),
        None => None
    }.ok_or(ReportableError::InternalError("Error getting database handle".into()))?;
    
    let pgversion = pool.query_one("SELECT version();", &[]).await?.get::<_, String>(0);

    command.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|m| m.embed(|e| e
            .title("Processing")
            .field("spectre", env!("CARGO_PKG_VERSION"), false)
            .field("postgresql", &format!("{}", pgversion), false)
            .description("Version info for spectre")
        ))
    }).await?;
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("version").description("Get postgresql version")
}
