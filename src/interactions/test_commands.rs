use serenity::{
    client::Context,
    model::interactions::{
        Interaction,
        InteractionResponseType,
        application_command:: ApplicationCommandInteraction,
        message_component::ButtonStyle
    }
};

pub async fn test_slash(command: &ApplicationCommandInteraction, ctx: &Context, interaction: Interaction) {
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