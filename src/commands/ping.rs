use crate::discord_client::CommandHandler;

pub struct PingCommand;

impl CommandHandler for PingCommand {
    fn process_command(
        &self,
        _command: &str,
        _tennant_id: u64,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), Box<std::error::Error>> {
        send_message_callback("Pong!");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn test_ping() {
        let ping = PingCommand;

        let result = Arc::new(Mutex::new(Box::new(String::new())));
        let closure_result = result.clone();
        assert!(ping
            .process_command("", 0, &|message| *closure_result.lock().unwrap() =
                Box::new(String::from(message)))
            .is_ok());

        let expected_message = String::from("Pong!");
        let timeout = Instant::now();
        while (timeout.elapsed() < Duration::from_secs(2))
            && (**result.lock().unwrap() != expected_message)
        {
            thread::sleep(Duration::from_millis(200));
        }

        assert_eq!(expected_message, **result.lock().unwrap());
    }
}
