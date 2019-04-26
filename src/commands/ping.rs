use crate::discord_client::CommandHandler;

pub struct PingCommand;

impl CommandHandler for PingCommand {
    fn process_command(
        &self,
        _command: &str,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), Box<std::error::Error>> {
        send_message_callback("Pong!");

        Ok(())
    }
}
