use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};

use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;

use crate::error::ReportableError;
use crate::PostgresPool;
use anyhow::Result;

pub async fn run(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), ReportableError> {
    match command.data.options.get(0) {
        Some(CommandDataOption {
            name,
            kind: CommandOptionType::SubCommand,
            options,
            ..
        }) => {
            println!("{name}, {:?}", options);

            let pool = match ctx.data.read().await.get::<PostgresPool>() {
                Some(v) => v.get().await.ok(),
                None => None,
            }
            .ok_or(ReportableError::InternalError(
                "Error getting database handle".into(),
            ))?;

            match name.as_str() {
                operation @ ("add" | "remove") => {
                    let user = match options.get(0).and_then(|r| r.resolved.as_ref()) {
                        Some(CommandDataOptionValue::String(username)) => {
                            crate::rblx::user::User::from_username(username).await?
                        }
                        _ => {
                            return Err(ReportableError::InternalError(
                                "Username/UserID was not received",
                            ))
                        }
                    };

                    let xp = match options.get(1).and_then(|r| r.resolved.as_ref()) {
                        Some(CommandDataOptionValue::Integer(group)) => {
                            if operation == "add" {
                                *group
                            } else {
                                -*group
                            }
                        }
                        _ => {
                            return Err(ReportableError::InternalError("Argument was not received"))
                        }
                    };

                    pool.execute(
                        "call ADD_XP($1::text, $2::bigint, $3::bigint)",
                        &[
                            &command.guild_id.unwrap().to_string(),
                            &user.get_user_id(),
                            &xp,
                        ],
                    )
                    .await?;

                    command
                        .create_interaction_response(&ctx.http, |resp| {
                            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|m| {
                                    m.embed(|e| {
                                        e.title(&format!("Success")).description(&format!(
                                            "{} {} xp to {}",
                                            if operation == "add" {
                                                "Added"
                                            } else {
                                                "Removed"
                                            },
                                            xp.abs(),
                                            user.get_username()
                                        ))
                                    })
                                })
                        })
                        .await?;
                }
                "show" => {
                    let user = match options.get(0).and_then(|r| r.resolved.as_ref()) {
                        Some(CommandDataOptionValue::String(username)) => {
                            crate::rblx::user::User::from_username(username).await?
                        }
                        _ => {
                            return Err(ReportableError::InternalError(
                                "Username/UserID was not received",
                            ))
                        }
                    };
                    let thumbnail = user.get_thumbnail().await?;

                    match pool.query_opt("SELECT * FROM GET_USER($1::text, $2::bigint) as t(rbx_id bigint, d_id bigint, xp bigint);", 
                        &[&command.guild_id.unwrap().to_string(), &user.get_user_id()]).await? {
                        Some(row) => {
                            println!("Found");
                            command.create_interaction_response(&ctx.http, |resp| {
                                resp.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|m| m.embed(|e| e
                                    .title(&format!("Success"))
                                    .description(&format!("User: {}\nXP: {}", user.get_username(), row.get::<_, i64>("xp")))
                                    .thumbnail(&thumbnail)
                                ))
                            }).await?;
                        },
                        None => {
                            println!("Not Found");
                            command.create_interaction_response(&ctx.http, |resp| {
                                resp.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|m| m.embed(|e| e
                                    .title(&format!("Success"))
                                    .description(&format!("User: {}\nXP: 0", user.get_username()))
                                    .thumbnail(&thumbnail)
                                ))
                            }).await?;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("xp")
        .description("Get user's xp")
        .create_option(|option| {
            option
                .name("add")
                .description("Add experience")
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
        .create_option(|option| {
            option
                .name("show")
                .description("Show experience")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|sub_option| {
                    sub_option
                        .name("user")
                        .description("Username/UserID")
                        .kind(CommandOptionType::String)
                })
        })
}
