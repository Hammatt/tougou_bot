# Tougou Bot

An (in progress) bot for discord. The old version of this had all the features that are still marked as todo so use an old release if you want those still.

## TODO:
 - basic ping command
 - commands to interface with the jihso.org website
 - commands to store and retrieve phrases
 - commands to interface with the vndb.org website
 - commands to interface with some anime database (mal api still down?)
 - commands to interface with wikipedia (was this feature used?)
 - unit/feature tests

## How to build it: 
`cargo build [--release]`

## How to test it:
Linting: `cargo fmt && cargo clippy`  
automated tests still to come.

## How to run it:
Make sure the environment variable `DISCORD_TOKEN` is set to your discord application's bot token.  
Compile using `cargo build --release` and then run the output executable.

## Features:  
### !pic: 
(warning: likely to be NSFW)  
This command fetches a random image from the danbooru site.   
Usage: `!pic [tag(s)]`. Tags are optional and space separated. Note: The Danbooru API caps the number of tags that can constrain a query to 2.
