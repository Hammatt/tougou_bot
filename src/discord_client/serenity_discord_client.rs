use crate::discord_client::{CommandHandler, DiscordClient};
use serenity::{
    client,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct SerenityDiscordClient {
    serenity_client: Arc<Mutex<Client>>,
    command_callbacks: Arc<Mutex<HashMap<String, Box<CommandHandler + Send>>>>,
}

struct SerenityDiscordHandler {
    command_callbacks: Arc<Mutex<HashMap<String, Box<CommandHandler + Send>>>>,
    command_prefix: &'static str,
}

impl DiscordClient for SerenityDiscordClient {
    fn new(token: &str) -> Self {
        println!("valid token: {}", client::validate_token(token).is_ok());

        let command_callbacks = Arc::new(Mutex::new(HashMap::new()));
        let serenity_handler = SerenityDiscordHandler {
            command_callbacks: command_callbacks.clone(),
            command_prefix: "!", //TODO: make this configurable in case of clashes with other bots
        };

        let serenity_client =
            Arc::new(Mutex::new(Client::new(token, serenity_handler).expect("Error creating serenity client")));
        println!("created client");

        let thread_serenity_client = serenity_client.clone();
        thread::spawn( move || {
            if let Err(why) = thread_serenity_client.lock().unwrap().start() {
                println!("An error occurred while running the client: {:?}", why);
            }
        });
        println!("started connection");

        SerenityDiscordClient {
            serenity_client: serenity_client.clone(),
            command_callbacks: command_callbacks.clone(),
        }
    }

    fn register_command<T>(&self, command: &str, command_handler: T) -> Result<(), Box<std::error::Error>>
    where
        T: CommandHandler + Send + 'static,
    {
        if self
            .command_callbacks
            .lock()
            .unwrap()
            .insert(command.to_string(), Box::new(command_handler))
            .is_some()
        {
            panic!("command was entered twice for {}", command);
        }
        Ok(())
    }
}

impl EventHandler for SerenityDiscordHandler {
    fn message(&self, ctx: Context, msg: Message) {
        //TODO: clean this up and split it out into more functions?
        if msg.content.starts_with(self.command_prefix) {
            if let Some(command) = msg.content.split_whitespace().nth(0) {
                if let Some(command) = command.chars().next().map(|c| &command[c.len_utf8()..]) {
                    if let Some(command_handler) =
                        self.command_callbacks.lock().unwrap().get(command)
                    {
                        if let Err(err) = command_handler.process_command(command, &|output| {
                            if let Err(err) = msg.channel_id.say(&ctx.http, output) {
                                println!("Error sending message: {:?}", err);
                            }
                        }) {
                            println!("Error processing command: {:?}", err);
                        }
                    };
                };
            };
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
