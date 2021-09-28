use serenity::{
    client::Context,
    model::interactions::{
        InteractionResponseType,
        application_command:: ApplicationCommandInteraction,
        message_component::ButtonStyle
    }
};

pub async fn test_slash(command: &ApplicationCommandInteraction, ctx: &Context) {
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

pub async fn new_command(command: &ApplicationCommandInteraction, ctx: &Context) {
    if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message.content("I'm a new command!".to_string());
                message.components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.style(ButtonStyle::Primary);
                            b.label("New Button");
                            b.custom_id("new_button");
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