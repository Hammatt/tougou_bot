# TougouBot

A basic bot written in elixir.  
Using the [poison](https://github.com/devinus/poison), [httpoison](https://github.com/edgurgel/httpoison), [alchemy](https://github.com/cronokirby/alchemy), and [floki](https://github.com/philss/floki) libraries.

## Running

Before you run you need to make sure that you have a file in the root director of the project called "token" which contains your private discord application bot token.  
You also will require a file called "mal" with the credentials for a [myanimelist.net](myanimelist.net) 
account if you want to use the anime and manga commands.

Make sure you have the dependancies with 
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