use super::test_buttons::test_buttons;

use serenity::{
    client::Context,
    model::interactions::message_component::MessageComponentInteraction,
};

pub async fn execute_interaction(command: &MessageComponentInteraction, ctx: &Context, enabled_components: &Vec<String>) {
    if enabled_components.contains(&command.data.custom_id.as_str().to_string()) {
        match command.data.custom_id.as_str() {
            "test_button" => { test_buttons(&command, &ctx).await }
            "new_button" => { test_buttons(&command, &ctx).await }
            _ => { println!("not implemented: {}", command.data.custom_id.as_str().to_string()); }
        }
    } else {
        println!("component not enabled: {}", command.data.custom_id.as_str().to_string());
    }
}