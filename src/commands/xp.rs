use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue, CommandDataOption};
use serenity::model::prelude::command::CommandOptionType;

use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;

use crate::PostgresPool;
use crate::error::ReportableError;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), ReportableError> {
    match command.data.options.get(0) {
        Some(CommandDataOption { name, kind: CommandOptionType::SubCommand, options, .. }) => {
            println!("{name}, {:?}", options);

            let pool = match ctx.data.read().await.get::<PostgresPool>() {
                Some(v) => v.get().await.ok(),
                None => None
            }.ok_or(ReportableError::InternalError("Error getting database handle".into()))?;
        
            match pool.query_opt("SELECT * FROM data_1060269829619191888.members LIMIT 1", &[]).await? {
                Some(row) => {
                    const WIDTH: i64 = 25;
                    let rbx_id = row.get::<_, i64>("rbx_id");
                    let xp = row.get::<_, i64>("xp");
                    command.create_interaction_response(&ctx.http, |resp| {
                        resp.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|m| m.embed(|e| e
                            .title(&format!("User - {}", rbx_id))
                            .description(&format!("{}/24 [{:<width$}]", xp, "#".repeat((100 * xp / (24 * 5) ) as usize), width=WIDTH as usize))
                        ))
                    }).await?;
                },
                _ => {}
            }
        },
        _ => {}
    }
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("xp").description("Get user's xp")
    .create_option(|option| {
        option
            .name("add")
            .description("Add experience")
            .kind(CommandOptionType::SubCommand)
    })
    .create_option(|option| {
        option
            .name("remove")
            .description("Remove experience")
            .kind(CommandOptionType::SubCommand)
            .create_sub_option(|sub_option| {
                sub_option
                    .name("user")
                    .description("Username/UserID")
                    .kind(CommandOptionType::String)  
            })
            .create_sub_option(|sub_option| {
                sub_option
                    .name("xp")
                    .description("XP")
                    .kind(CommandOptionType::Integer)  
            })
    })
}
