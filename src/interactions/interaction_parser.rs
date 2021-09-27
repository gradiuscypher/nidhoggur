use serenity::model::interactions::Interaction;
use super::test_buttons::test_buttons;

use serenity::{
    client::Context,
    model::interactions::{
        InteractionResponseType,
        message_component::MessageComponentInteraction
    }
};

pub struct InteractionParser {
    enabled_interactions: Vec<String>,
}

impl InteractionParser {
    fn is_enabled(&self, interaction_name: String) -> bool {
        self.enabled_interactions.contains(&interaction_name)
    }

    pub async fn execute_interaction(&self, command: &MessageComponentInteraction, ctx: &Context) {
        match command.data.custom_id.as_str() {
            "test_button" => { test_buttons(&command, &ctx).await }
            _ => { println!("not implemented: {}", command.data.custom_id.as_str().to_string()); }
        }
    }
}