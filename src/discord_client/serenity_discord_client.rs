use crate::discord_client::DiscordClient;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    client,
};

pub struct SerenityDiscordClient {
    serenity_client: Client,
}

struct SerenityDiscordHandler;

impl DiscordClient for SerenityDiscordClient {
    fn new(token: &str) -> Self {
        println!("valid token: {}", client::validate_token(token).is_ok());
        let mut serenity_client = Client::new(token, SerenityDiscordHandler).expect("Error creating serenity client");
        println!("created client");
        if let Err(why) = serenity_client.start() {
            println!("An error occurred while running the client: {:?}", why);
        }
        println!("started connection");
        SerenityDiscordClient { serenity_client }
    }

    fn register_command(command: &str) -> Result<(), &'static str> {
        Ok(())
    }

    fn register_prefix(prefix: &str) -> Result<(), &'static str> {
        Ok(())
    }
}

impl EventHandler for SerenityDiscordHandler {
    fn message(&self, _: Context, msg: Message) {
        println!("received message {}", msg.content);
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say("Pong!") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}