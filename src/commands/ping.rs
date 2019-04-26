use crate::discord_client::CommandHandler;

pub struct PingCommand;

impl CommandHandler for PingCommand {
    fn process_command(
        &self,
        _command: &str,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), &'static str> {
        send_message_callback("Pong!");

        Ok(())
    }
}
