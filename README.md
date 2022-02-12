$ jab
=====

A subset of [punch](https://github.com/rathrio/punch) reimplemented in Rust to
explore a little more of the language.

Todos
-----

- [x] Model month and compute durations
- [x] Formatter for outputting with totals
- [x] Infer month and year from file name
- [x] Option parsing / basic CLI setup
- [x] Dynamic current date
- [x] Add current day 
- [x] Extract day and block to separate modules
- [x] Coloured output
- [x] Move selection and highlight state to some place else (don't conflate it
  with the days themselves)
- [x] Adding blocks
- [x] Hook up adding blocks to CLI
- [x] Write to file
- [x] Display ongoing blocks properly
- [x] Support all the fancy block shorthands
  - [x] ongoing blocks
  - [x] punch now
  - [x] convenience shorthands, 820 instead of 08:20

- [x] Idiomatic module handling
- [x] Extract fancy block parsing to a more suitable place
- [x] punch --edit
- [x] Cleanup month before writing (e.g. skip empty days)
- [x] --remove
- [x] Read hours folder conf from env variable
- [x] --day, --yesterday
- [x] --comment, --clear-comment
- [x] --brf for opening hours directory
- [x] --full (make this the default behaviour of the terminal formatter)
    - [x] Represent days as hash map

- [x] --month, --next, --previous
- [x] read this chapter: https://doc.rust-lang.org/book/ch09-00-error-handling.html

Out of Scope
------------

- Multiday punching
- Cards
- Config
- All the DEV stuff (logs / git version integration)
- BRF Mailer
- Shell completion
- Error handling on parse failure (started doing it and it's really verbose. No
  need to fully push this through. I got the gist of how to propagate
  recoverable errors etc...)
