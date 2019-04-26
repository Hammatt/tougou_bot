use crate::discord_client::{CommandHandler, DiscordClient};
use serenity::{
    client,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct SerenityDiscordClient {
    serenity_client: Client,
}

struct SerenityDiscordHandler {
    command_callbacks: Arc<Mutex<HashMap<String, Box<CommandHandler + Send>>>>,
}

impl DiscordClient for SerenityDiscordClient {
    fn new(token: &str) -> Self {
        println!("valid token: {}", client::validate_token(token).is_ok());
        let handler = SerenityDiscordHandler {
            command_callbacks: Arc::new(Mutex::new(HashMap::new())),
        };
        let mut serenity_client =
            Client::new(token, handler).expect("Error creating serenity client");
        println!("created client");
        if let Err(why) = serenity_client.start() {
            println!("An error occurred while running the client: {:?}", why);
        }
        println!("started connection");
        SerenityDiscordClient { serenity_client }
    }

    fn register_command<T>(command: &str, command_handler: T) -> Result<(), &'static str>
    where
        T: CommandHandler,
    {
        Ok(())
    }

    fn register_prefix(prefix: &str) -> Result<(), &'static str> {
        Ok(())
    }
}

impl EventHandler for SerenityDiscordHandler {
    fn message(&self, ctx: Context, msg: Message) {
        if let Some(command) = msg.content.split_whitespace().nth(0) {
            if let Some(command_handler) = self.command_callbacks.lock().unwrap().get(command) {
                if let Err(err) = command_handler.process_command(command, &|output| {
                    if let Err(err) = msg.channel_id.say(&ctx.http, output) {
                        println!("Error sending message: {:?}", err);
                    }
                }) {
                    println!("Error processing command: {:?}", err);
                }
            };
        };
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
