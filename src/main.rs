use log::info;
use std::env;
use std::sync::{Arc, Condvar, Mutex};
use tougou_bot::commands::{
    jisho::JishoCommand, pic::PicCommand, ping::PingCommand, status::StatusCommand,
    tag::TagCommand, vndb::VNDBCommand,
};
use tougou_bot::data_access::*;
use tougou_bot::discord_client::*;

fn main() {
    env_logger::init();
    info!("starting init...");

    let token: String =
        env::var("DISCORD_TOKEN").expect("Must set the environment variable `DISCORD_TOKEN`");

    let client = serenity_discord_client::SerenityDiscordClient::new(&token);

    let danbooru_pic_repository =
        Box::new(pic_repository::danbooru_pic_repository::DanbooruPicRepository::default());

    let pic_command = Arc::new(Mutex::new(PicCommand::new(danbooru_pic_repository)));
    client.register_command("pic", pic_command).unwrap();

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

    let jisho_org_repository =
        Box::new(jisho_repository::jisho_org_repository::JishoOrgRepository::default());
    let jisho_command = Arc::new(Mutex::new(JishoCommand::new(jisho_org_repository)));
    client.register_command("jisho", jisho_command).unwrap();

    let vndb_org_repository =
        Box::new(vndb_repository::vndb_org_repository::VNDBOrgRepository::default());
    let vndb_command = Arc::new(Mutex::new(VNDBCommand::new(vndb_org_repository)));
    client.register_command("vndb", vndb_command).unwrap();

    log::info!("init finished. starting keep alive lock.");

    let keep_alive = Condvar::new();
    let keep_alive_lock = Mutex::new(());
    let _ = keep_alive
        .wait(keep_alive_lock.lock().unwrap())
        .expect("keep alive lock failed");

    log::info!("exit requested. exiting.")
}
