# TougouBot

A basic bot written in elixir.  
Using the [poison](https://github.com/devinus/poison), [httpoison](https://github.com/edgurgel/httpoison), [alchemy](https://github.com/cronokirby/alchemy), and [floki](https://github.com/philss/floki) libraries.

## Features

### Debug Module:

Contains useful debug commands like `!ping` and `!status`. Also contains the `!roll` command which you can use to generate random numbers, see !help for more info.

### Jisho Module:

Accessable via the `!jisho` command, this module leverage the api provided by http://jisho.org 
to quickly get definitions and readings of Japanese words.  
e.g.: ![example usage](https://i.imgur.com/3yEoNuU.png)

### Tag Module:

Accessable via the `!tag` family of commands (check !help for more syntax info). Allows you to give the bot a piece of text-based information to recall.  
e.g.: ![example usage](https://i.imgur.com/mVAyqfq.png)

### VNDB Module:

Warning: Lkely to be NSFW. Accessable via the `!vndb` family of commands (check !help for more info) Leverages https://vndb.org to search for Visual Novels and provides a link back.  
e.g.: ![example usage](https://i.imgur.com/xoCoUHS.png)

### Wiki Module:

Accessable via the `!wiki` command. Leverages the API provided by https://en.wikipedia.org/wiki/Main_Page to serach for articles on wikipedia.  
e.g.: ![example usage](https://i.imgur.com/lD0BRZO.png)

### Anime Module:

Accessable via the `!anime` and `!manga` commands. Leverages the API provided by https://myanimelist.net to search for anime or manga and provides links to their articles.  
e.g.: ![example usage](https://i.imgur.com/RMTswbK.png)

### Danbooru Module:

Warning: Likely to be NSFW. Accessable via the `!pic` command (check !help for more info). Leverages the [Danbooru API](https://danbooru.donmai.us/wiki_pages/43568) (Site may be NSFW) to provide a random picture based on given serach tags.  
e.g.: ![example usage](https://i.imgur.com/6u4rCae.png)


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
