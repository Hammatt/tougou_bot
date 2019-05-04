use std::env;
use std::sync::{Arc, Condvar, Mutex};
use tougou_bot::commands::{
    pic::PicCommand, ping::PingCommand, status::StatusCommand, tag::TagCommand,
};
use tougou_bot::data_access::*;
use tougou_bot::discord_client::*;

fn main() {
    let token: String =
        env::var("DISCORD_TOKEN").expect("Must set the environment variable `DISCORD_TOKEN`");

    let client = serenity_discord_client::SerenityDiscordClient::new(&token);
    client
        .register_command("pic", Arc::new(Mutex::new(PicCommand)))
        .unwrap();
    client
        .register_command("ping", Arc::new(Mutex::new(PingCommand)))
        .unwrap();
    client
        .register_command("status", Arc::new(Mutex::new(StatusCommand)))
        .unwrap();

    let sqlite_tag_repository =
        Box::new(tag_repository::sqlite_tag_repository::SqliteTagRepository::new().unwrap());

    let tag_command = Arc::new(Mutex::new(TagCommand::new(sqlite_tag_repository).unwrap()));
    client
        .register_command("ntag", tag_command.clone())
        .unwrap();
    client.register_command("tag", tag_command.clone()).unwrap();
    client
        .register_command("atags", tag_command.clone())
        .unwrap();

    let keep_alive = Condvar::new();
    let keep_alive_lock = Mutex::new(());
    let _ = keep_alive
        .wait(keep_alive_lock.lock().unwrap())
        .expect("keep alive lock failed");
}
