use std::{env, process::Command};

use serenity::{async_trait, futures::io::Read, model::{
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommand,
                ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType
            },
            Interaction,
            InteractionResponseType
        },
    }, prelude::*};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Hey I'm alive!".to_string(),
                "id" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option")
                        .resolved
                        .as_ref()
                        .expect("Expected user object");
                    
                    if let ApplicationCommandInteractionDataOptionValue::User(user, _member) = 
                        options
                    {
                        format!("{}'s ID is {}", user.tag(), user.id)
                    } else {
                        "Please provide a valid user".to_string()
                    }
                },
                _ => "not implemented".to_string(),
            };
            
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);

        let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command.name("id").description("Get a user ID").create_option(|option| {
                        option
                            .name("id")
                            .description("The user ID to lookup")
                            .kind(ApplicationCommandOptionType::User)
                            .required(true)
                    })
                })
                .create_application_command(|command| {
                    command
                        .name("welcome")
                        .description("welcome a user")
                        .create_option(|option| {
                            option
                                .name("user")
                                .description("The user to welcome")
                                .kind(ApplicationCommandOptionType::User)
                                .required(true)

                        })
                        .create_option(|option| {
                            option
                                .name("message")
                                .description("The message to send")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                                .add_string_choice("Welcome friend!", "friend")
                                .add_string_choice("Do you want coffee", "coffee")
                                .add_string_choice("Welcome to the club", "club")
                        })
                })
        })
        .await;

        println!("I now have the following commands: {:#?}", commands);

        let guild_command = GuildId(268239941195137025)
            .create_application_command(&ctx.http, |command| {
                command.name("awesome").description("An awesome command.")
            })
            .await;
            println!("I created the guild command: {:#?}", guild_command);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token environment variable");

    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an APPLICATON_ID environment variable")
        .parse()
        .expect("Application ID is not valid");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}