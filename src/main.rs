use tougou_bot::discord_client::*;
use std::env;

fn main() {
    let token: String = env::var("DISCORD_TOKEN").expect("Must set the environment variable `DISCORD_TOKEN`");

    let client = serenity_discord_client::SerenityDiscordClient::new(&token);

    let mut temp = String::new();
    std::io::stdin().read_line(&mut temp).expect("failed to read from console");
}
