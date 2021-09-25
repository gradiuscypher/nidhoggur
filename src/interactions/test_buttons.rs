use serenity::{
    client::Context,
    model::interactions::{
        InteractionResponseType,
        message_component::MessageComponentInteraction
    }
};

pub async fn test_buttons(command: &MessageComponentInteraction, ctx: &Context) {
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
}