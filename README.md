# Introduction
[Dyer-cli] is a great tool created to guide you use [dyer] fast and at ease, helps you build a robust crawler, data processor, netwrok program fast and correctly.

[Dyer-cli]: https://github.com/HomelyGuy/dyer-cli
[dyer]: https://github.com/HomelyGuy/dyer

# Installation
Dyer-cli is built completely by Rust programming language without extra dependencies, So rust must be installed beforehand, to test it with:
```bash
rustup --version
```
if you ever see some infomation like that
```bash
rustup 1.23.1 (3df2264a9 2020-11-30)
```	
then you are ready to go, the following code would suffice.
```bash
cargo install dyer-cli
```
the command will download the source code and complie it to build a executable file inside your `$HOME/.cargo/bin`

# Commands
Dyer-cli provides some commands that helps you initialize, debug programm, but for now, only `dyer-cli new` supported, more commands are to go.

## dyer-cli new
This command helps you initialize a project with log level `Info`, other log levels vares from `Error`, `Warn`, `Info`, `Debug`, and `Trace`, and its structure is
```bash
|___Cargo.toml
|___Readme.md
|___data/
|___data/tasks/
|___src/
    |___src/entity.rs
    |___src/parser.rs
    |___src/spider.rs
    |___src/middleware.rs
    |___src/pipeline.rs
```
## dyer-cli check
A warper of `cargo check`, if you run it the first time,`dyer-cli` will download the crates and then check the code. 

## dyer-cli fix
A wraper of `cargo fix`,  if some warning happens such as `unused import` or `dead code` the command does a lot for you. However it won't help if some errors occur, if so, you have to debug the code manually.

## dyer-cli run
A wraper of `cargo run`, when the program compiles, run it.

## dyer-cli build
A wraper of `cargo build`,   build the program.

## dyer-cli test
A wraper of `cargo test`,   test the program.

## dyer-cli clean
A wraper of `cargo clean`,   clean the directory.
