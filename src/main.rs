use std::env;
use tougou_bot::commands::{ pic::PicCommand, ping::PingCommand, status::StatusCommand };
use tougou_bot::discord_client::*;

fn main() {
    let token: String =
        env::var("DISCORD_TOKEN").expect("Must set the environment variable `DISCORD_TOKEN`");

    let client = serenity_discord_client::SerenityDiscordClient::new(&token);
    client.register_command("pic", PicCommand).unwrap();
    client.register_command("ping", PingCommand).unwrap();
    client.register_command("status", StatusCommand).unwrap();

    let mut temp = String::new();
    std::io::stdin()
        .read_line(&mut temp)
        .expect("failed to read from console");
}
