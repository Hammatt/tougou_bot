use std::env;

fn main() {
    let token: String = env::var("DISCORD_TOKEN").expect("Must set the environment variable `DISCORD_TOKEN`");

    

    println!("My token is {}", token);
}
