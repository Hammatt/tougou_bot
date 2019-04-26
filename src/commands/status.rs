use crate::discord_client::CommandHandler;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct StatusCommand;

impl StatusCommand {
    fn status(&self) -> String {
        format!("Version: {}", VERSION)
    }
}

impl CommandHandler for StatusCommand {
    fn process_command(
        &self,
        _command: &str,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), Box<std::error::Error>> {
        send_message_callback(&self.status());

        Ok(())
    }
}
