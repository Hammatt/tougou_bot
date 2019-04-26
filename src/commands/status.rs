use crate::discord_client::CommandHandler;

pub struct StatusCommand;

impl StatusCommand {
    fn status(&self) -> &str {
        ""
    }
}

impl CommandHandler for StatusCommand {
    fn process_command(
        &self,
        _command: &str,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), &'static str> {
        send_message_callback(self.status());

        Ok(())
    }
}
