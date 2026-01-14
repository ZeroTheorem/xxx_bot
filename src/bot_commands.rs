use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Commands {
    #[command(description = "display this text.")]
    Menu,
}
