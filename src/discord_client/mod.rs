pub mod serenity_discord_client;

pub trait DiscordClient {
    fn new(token: &str) -> Self;

    fn register_command<T>(command: &str, command_handler: T) -> Result<(), &'static str>
    where
        T: CommandHandler;
    fn register_prefix(prefix: &str) -> Result<(), &'static str>;
}

pub trait CommandHandler {
    fn process_command(
        &self,
        command: &str,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), &'static str>;
}
