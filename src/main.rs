mod interactions;
mod commands;

use interactions::interaction_parser::execute_interaction;
use interactions::command_parser::execute_command;

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
    components: Vec<String>,
}

impl TypeMapKey for BotConfig {
    type Value = BotConfig;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let data = ctx.data.read().await;
        let conf = data.get::<BotConfig>().unwrap();

        if let Interaction::ApplicationCommand(command) = interaction {
            execute_command(&command, &ctx, &conf.commands).await;

        } else if let Interaction::MessageComponent(command) = interaction {
            execute_interaction(&command, &ctx, &conf.components).await;

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
    // grab the config.json and parse it
    let config_file = File::open("config.json").expect("Unable to read config file.");
    let config_reader = BufReader::new(config_file);
    let bot_config: BotConfig = serde_json::from_reader(config_reader).expect("Unable to parse bot config.");

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

    // insert all needed data into context data
    {
        let mut data = client.data.write().await;
        data.insert::<BotConfig>(bot_config);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}