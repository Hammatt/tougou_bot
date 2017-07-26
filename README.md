# TougouBot

A basic bot written in elixir.  
Using the [poison](https://github.com/devinus/poison), [httpoison](https://github.com/edgurgel/httpoison), [alchemy](https://github.com/cronokirby/alchemy), 

## Running

Before you run you need to make sure that you have a file in the root director of the project called "token" which contains your private discord application bot token.  
make sure you have the dependancies with 
```bash
mix deps.get
```
then run the bot
```bash
## To start in production
./kidou.sh

## To start in debug/interactive mode
iex -S mix
```