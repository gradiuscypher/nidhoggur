mod interactions;

use std::{env, process::Command};

use serenity::{async_trait, futures::io::Read, model::{gateway::Ready, id::GuildId, interactions::{Interaction, InteractionResponseType, message_component::{ButtonStyle}, application_command::{ApplicationCommand, ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionChoice, ApplicationCommandOptionType}}}, prelude::*};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "gcommand1" => {
                    if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content("I've run the gcommand!".to_string());
                                message.components(|c| {
                                    c.create_action_row(|ar| {
                                        ar.create_button(|b| {
                                            b.style(ButtonStyle::Primary);
                                            b.label("Test Button");
                                            b.custom_id("test_button");
                                            b
                                        })
                                    })
                                })
                            })
                    })
                    .await
                    {
                        println!("Error: {}", why);
                    }
                }
                _ => {
                    println!("not implemented: {}", command.data.name.as_str().to_string());
                }
            }
        } else if let Interaction::MessageComponent(command) = interaction {
            println!("MessageComponent\n command.id: {} user.id: {} data.custom_id: {}", command.id, command.user.id, command.data.custom_id);
            if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content("Button has been pressed.".to_string())
                    })
            })
            .await
            {
                println!("Error: {}", why);
            }
        } else {
            println!("We're outside in spooky");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);

        let _ = GuildId(268239941195137025).set_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command
                        .name("gcommand1")
                        .description("An awesome command 1.")
                })
                .create_application_command(|command| {
                    command
                        .name("gcommand2")
                        .description("An awesome command 2.")
                        .create_option(|option| {
                            option
                                .name("stringopt")
                                .description("The string to send")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                                .add_string_choice("Welcome friend!", "friend")
                                .add_string_choice("Do you want coffee", "coffee")
                                .add_string_choice("Welcome to the club", "club")
                        })
                        .create_option(|option| {
                            option
                                .name("opt2")
                                .description("The string to send")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                                .add_string_choice("This is string1", "str1")
                                .add_string_choice("This is string2", "str2")
                        })
                })
        })
        .await;
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