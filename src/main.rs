mod commands;
mod error;
mod rblx;

#[macro_use]
extern crate serde;

use std::env;
use std::error::Error;
use std::sync::Arc;

use error::ReportableError;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::Embed;
use serenity::{async_trait, model::prelude::interaction::InteractionResponseType};
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};

// Why the fuck this is necessary is beyond me
pub struct PostgresPool;
impl TypeMapKey for PostgresPool {
    type Value = Arc<Pool>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let command_result = match command.data.name.as_str() {
                "xp" => commands::xp::run(&ctx, &command).await,
                "register" => commands::register::run(&ctx, &command).await,
                "version" => commands::version::run(&ctx, &command).await,
                "exec" => commands::exec::run(&ctx, &command).await,
                _ => return,
            };

            match command_result {
                Ok(_) => {}
                Err(why) => {
                    let mut embed = CreateEmbed::default();
                    match why {
                        ReportableError::UserError(msg) => {
                            let embed = embed
                                .title("Uh oh..")
                                .description(&format!("```\n{}```", msg))
                                .to_owned();

                            // Try create interaction response, fails when response already made
                            match command.create_interaction_response(&ctx.http, |resp| resp
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|m| m
                                    .set_embed(embed.to_owned())
                                )).await {
                                Ok(_) => {},
                                Err(_) => {
                                    // Send error as message instead
                                    _ = command
                                    .channel_id
                                    .send_message(&ctx.http, |m| m
                                        .set_embed(embed.to_owned())
                                    )
                                    .await;
                                }
                            }
                        }
                        _ => {
                            eprintln!("FATAL: {:?}", why);
                            _ = command
                                .channel_id
                                .send_message(&ctx.http, |m| m
                                    .set_embed(embed.to_owned())
                                )
                                .await;
                        }
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands = GuildId::set_application_commands(
            &GuildId(1060269829619191888),
            &ctx.http,
            |commands| {
                commands
                    .create_application_command(|command| commands::xp::register(command))
                    .create_application_command(|command| commands::register::register(command))
                    .create_application_command(|command| commands::version::register(command))
                    .create_application_command(|command| commands::exec::register(command))
            },
        )
        .await
        .unwrap();
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;

        let mut cfg = Config::new();
        cfg.host = Some("db".to_owned());
        cfg.port = Some(5432);
        cfg.user = Some("postgres".to_owned());
        cfg.password = Some("postgres".to_owned());
        cfg.dbname = Some("master".to_owned());

        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        let pool = cfg
            .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
            .unwrap();

        // Migration
        pool.get()
            .await
            .unwrap()
            .batch_execute(include_str!("../migrations.sql"))
            .await
            .unwrap();

        data.insert::<PostgresPool>(Arc::new(pool));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
