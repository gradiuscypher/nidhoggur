use super::test_commands:: {
    test_slash,
    new_command
};


use serenity::{
    client::Context,
    model::interactions::application_command::ApplicationCommandInteraction,
};

pub async fn execute_command(command: &ApplicationCommandInteraction, ctx: &Context, enabled_commands: &Vec<String>) {
    if enabled_commands.contains(&command.data.name.as_str().to_string()) {
        match command.data.name.as_str() {
            "gcommand2" => { test_slash(&command, &ctx).await }
            "newcommand" => { new_command(&command, &ctx).await }
            _ => { println!("not implemented: {}", command.data.name.as_str().to_string()); }
        }
    } else {
        println!("command not enabled: {}", command.data.name.as_str().to_string());
    }
}