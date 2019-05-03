pub mod serenity_discord_client;

use std::sync::{Arc, Mutex};

pub trait DiscordClient {
    fn new(token: &str) -> Self;

    fn register_command<T>(
        &self,
        command: &str,
        command_handler: Arc<Mutex<T>>,
    ) -> Result<(), Box<std::error::Error>>
    where
        T: CommandHandler + Send + 'static;
}

pub trait CommandHandler {
    fn process_command(
        &self,
        command: &str,
        tennant_id: u64,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), Box<std::error::Error>>;
}
