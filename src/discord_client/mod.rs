pub mod serenity_discord_client;

pub trait DiscordClient {
    fn new(token: &str) -> Self;

    fn register_command<T>(&self, command: &str, command_handler: T) -> Result<(), &'static str>
    where
        T: CommandHandler + Send + 'static;
}

pub trait CommandHandler {
    fn process_command(
        &self,
        command: &str,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), &'static str>;
}
