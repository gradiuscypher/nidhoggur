mod interactions;
mod commands;

use std::env;
use std::fs::File;
use std::io::BufReader;

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

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct BotConfig {
    commands: Vec<String>,
}

struct Handler;

const INTERACTION_PARSER: interactions::interaction_parser::InteractionParser = interactions::interaction_parser::InteractionParser {
    enabled_interactions: vec!["command1".to_string()],
};

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
            INTERACTION_PARSER.execute_interaction(&command, &ctx);
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
    let config_file = File::open("config.json").expect("Unable to read config file.");
    let config_reader = BufReader::new(config_file);
    let bot_config: BotConfig = serde_json::from_reader(config_reader).expect("Unable to parse bot config.");

    println!("bot config: {:#?}", bot_config.commands);
    if bot_config.commands.contains(&"command1".to_string()) {
        println!("Contained");
    }
    if bot_config.commands.contains(&"command10".to_string()) {
        println!("BAD CONTAIN");
    }

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