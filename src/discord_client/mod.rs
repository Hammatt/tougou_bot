mod serenity_discord_client;

trait DiscordClient {
    fn new(token: &str) -> Self;

    fn register_command(command: &str) -> Result<(), &'static str>;
    fn register_prefix(prefix: &str) -> Result<(), &'static str>;
}