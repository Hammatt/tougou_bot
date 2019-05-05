# Tougou Bot

An (in progress) bot for discord. The old version of this had all the features that are still marked as todo so use an old release if you want those still.

## TODO:
 - commands to interface with the jihso.org website
 - commands to interface with the vndb.org website
 - commands to interface with some anime database (mal api still down?)
 - commands to interface with wikipedia (was this feature used?)
 - unit/feature tests (need to stub out external APIs somehow)
 - bubble more error messages back to the user (at the moment some are swallowed and logged so the end user doesn't know something has happened)

## How to build it: 
[Make sure rustc & cargo are installed](https://www.rust-lang.org/learn/get-started)  
`cargo build [--release]`

## How to test it:
Linting: `cargo fmt && cargo clippy`  
Unit: `cargo test`  
More detailed automated tests still to come.

## How to run it:
Make sure the environment variable `DISCORD_TOKEN` is set to your discord application's bot token.  
Compile using `cargo build --release` and then run the output executable.

The environment variable `RUST_LOG` can be set to log levels of `trace`, `debug`, `info`, `warn`, or `error`. I reccomend using at least `info`.

## Commands:  
### !ping:  
Tougou replies saying "Pong!"

### !pic: 
(warning: likely to be NSFW)  
This command fetches a random image from the danbooru site.   
Usage: `!pic [tag(s)]`. Tags are optional and space separated. Note: The Danbooru API caps the number of tags that can constrain a query to 2.

### !tag:  
This command allows you to set tougou to memorise and recall phrases.  
Example:  
```
you: !ntag tag_name some tag body
tougou: Created new tag tag_name with body some tag body
you: !tag tag_name
tougou: some tag body
```