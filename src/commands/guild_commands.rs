use serenity::{
    client::Context,
    model::interactions::application_command::ApplicationCommandOptionType,
    model::prelude::GuildId
};

pub async fn setup_guild_commands(ctx: &Context) {
    let _ = GuildId(268239941195137025).set_application_commands(&ctx.http, |commands| {
        commands
            .create_application_command(|command| {
                command
                    .name("gcommand1")
                    .description("An awesome command 1.")
            })
            .create_application_command(|command| {
                command
                    .name("newcommand")
                    .description("An new command created.")
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