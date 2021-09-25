mod interactions;
mod commands;

use std::env;

use serenity:: {
    async_trait, 
    model:: {
        gateway::Ready, 
        interactions:: {
            Interaction
        }
    }, 
    prelude::*
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "gcommand2" => { interactions::test_commands::test_slash(&command, &ctx).await }
                "newcommand" => { interactions::test_commands::new_command(&command, &ctx).await }
                _ => { println!("not implemented: {}", command.data.name.as_str().to_string()); }
            }
        } else if let Interaction::MessageComponent(command) = interaction {
            match command.data.custom_id.as_str() {
                "test_button" => { interactions::test_buttons::test_buttons(&command, &ctx).await }
                _ => { println!("not implemented: {}", command.data.custom_id.as_str().to_string()); }
            }
        } else {
            println!("We're outside in spooky");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
        commands::guild_commands::setup_guild_commands(&ctx).await
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