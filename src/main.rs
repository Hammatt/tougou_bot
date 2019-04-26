use std::env;
use std::sync::{Condvar, Mutex};
use tougou_bot::commands::{pic::PicCommand, ping::PingCommand, status::StatusCommand};
use tougou_bot::discord_client::*;

fn main() {
    let token: String =
        env::var("DISCORD_TOKEN").expect("Must set the environment variable `DISCORD_TOKEN`");

    let client = serenity_discord_client::SerenityDiscordClient::new(&token);
    client.register_command("pic", PicCommand).unwrap();
    client.register_command("ping", PingCommand).unwrap();
    client.register_command("status", StatusCommand).unwrap();

    let keep_alive = Condvar::new();
    let keep_alive_lock = Mutex::new(());
    let _ = keep_alive
        .wait(keep_alive_lock.lock().unwrap())
        .expect("keep alive lock failed");
}
