# Tougou Bot

An (in progress) bot for discord. The old version of this had all the features that are still marked as todo so use an old release if you want those still.

## TODO:
 - commands to interface with the vndb.org website
 - commands to interface with some anime database (mal api still down?)
 - commands to interface with wikipedia (was this feature used?)
 - unit/feature tests (need to stub out external APIs somehow)
 - build packages for releases (.deb, .rpm?)

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
tougou: 新しいタッグ「tag_name」➡「some tag body」を作った。
you: !tag tag_name
tougou: some tag body
```

### !jisho:
This command allows you to check for the definition and/or reading of a japanese word. This command uses the jisho.org api.  
Example:  
```
you: !jisho wipe
tougou: 言葉：拭く
        読み方：ふく
        言葉の意味：to wipe; to dry
        続き：https://jisho.org/search/wipe
```