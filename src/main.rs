//! # Introduction
//! [Dyer-cli] is a great tool created to guide you use [dyer] fast and at ease, helps you build a robust crawler, data processor, netwrok program fast and correctly.
//!
//! [Dyer-cli]: https://github.com/HomelyGuy/dyer-cli
//! [dyer]: https://github.com/HomelyGuy/dyer
//!
//! # Installation
//! Dyer-cli is built completely by Rust programming language without extra dependencies, So rust must be installed beforehand, to test it with:
//! ```bash
//! rustup --version
//! ```
//! if you ever see some infomation like that
//! ```bash
//! rustup 1.23.1 (3df2264a9 2020-11-30)
//! ```
//! then you are ready to go, the following code would suffice.
//! ```bash
//! cargo install dyer-cli
//! ```
//! the command will download the source code and complie it to build a executable file inside your `$HOME/.cargo/bin`
//!
//! # Commands
//! Dyer-cli provides some commands that helps you initialize, debug programm, but for now, only `dyer-cli new`
// `dyer-cli run`
//! supported, more commands are to go.
//!
//! ## dyer-cli new
//! This command helps you initialize a project with log level `Info`, other log levels vares from `Error`, `Warn`, `Info`, `Debug`, and `Trace`, and its structure is
//! ```bash
//! |___Cargo.toml
//! |___Readme.md
//! |___data/
//! |___data/tasks/
//! |___src/
//!     |___src/entity.rs
//!     |___src/parser.rs
//!     |___src/spider.rs
//!     |___src/middleware.rs
//!     |___src/main.rs
//!     |___src/pipeline.rs
//! ```
//! Main functionality of each file:                                        
//! * the `entity.rs` contains entities/data structure to be used/collected
//! * the `parser.rs` contains functions that extract entities from response
//! * the `spider.rs` contains initial when opening and final things to do when closing
//! * the `middleware.rs` contains Some middlewares that process data at runtime
//! * the `pipeline.rs` contains entities manipulation including data-storage, displsying and so on
//! * the `main.rs` combines all modules then build them up into a programm
//! * `Cargo.toml` is the basic configuration of the project
//! * `README.md` contains some instructions of the project
//! * `data` folder balance the app load when data in app exceeds, and backup app data at certain gap
// ## dyer-cli run
// This command build your programm and run it with log level `Info`, other log levels vares from `Error`, `Warn`, `Info`, `Debug`, and `Trace`, `--no-log` to disable log out.

mod subcommand;
mod util;

use subcommand::{SubComNew, /*SubComRun,*/ SubCommand};
use util::LogLevel;

#[derive(std::fmt::Debug)]
pub struct Info {
    sub_command: String,
    options: Vec<String>,
    others: Vec<String>,
}
impl From<Vec<String>> for Info {
    fn from(mut args: Vec<String>) -> Self {
        let sub_command = args.remove(0);
        let mut options = Vec::new();
        let mut others = Vec::new();
        args.into_iter().for_each(|item: String| {
            if item.contains("--") {
                let option = item.strip_prefix("--").unwrap().to_owned();
                options.push(option);
            } else if item.contains("-") {
                let option = item.strip_prefix("-").unwrap().to_owned();
                options.push(option);
            } else {
                others.push(item);
            }
        });
        Info {
            sub_command,
            options,
            others,
        }
    }
}
impl Into<SubCommand> for Info {
    fn into(mut self) -> SubCommand {
        let mut comd: SubCommand = SubCommand::Null;
        if self.sub_command == "new" {
            let name = self.others.pop().expect("project name must be specified.");
            let level = if self.options.is_empty() {
                Some(LogLevel::Info)
            } else {
                let index = self.options.pop().unwrap();
                Some(index.parse::<LogLevel>().unwrap_or(LogLevel::Info))
            };
            comd = SubCommand::SubComNew(SubComNew {
                name,
                option: level,
            });
        }
        /*
         *else if self.sub_command == "run" {
         *    let level: LogLevel = self.options.pop().unwrap_or("Info".into()).parse().unwrap();
         *    let item = SubComRun {
         *        option: Some(level),
         *    };
         *    comd = SubCommand::SubComRun(item);
         *}
         */
        comd
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    //println!("raw arguments: {:?}", args);
    args.remove(0); // remove the unnecessary path
                    //let msgs = "Handy tool for dyer\n\nUSAGE:\n\tdyer-cli [subcommand] [options]\n\nSUBCOMMAND:\n\tnew:\t\tinitialize a new empty project\n\trun:\t\tcomplie and run the project\n\nOPTIONS:\n\t--error:\t\tset the log level as ERROR\n\t--warn: \t\tset the log level as WARN\n\t--info: \t\tset the log level as INFO\n\t--debug:\t\tset the debug level as DEBUG\n\t--trace:\t\tset the log level as TRACE".replace("\t", "    ");
    let msgs = "Handy tool for dyer\n\nUSAGE:\n\tdyer-cli [subcommand] [options]\n\teg. dyer-cli new myproject --debug create a project with logger level INFO\n\nSUBCOMMAND:\n\tnew:\t\tinitialize a new empty project\n\nOPTIONS:\n\t--error:\t\tset the log level as ERROR\n\t--warn: \t\tset the log level as WARN\n\t--info: \t\tset the log level as INFO\n\t--debug:\t\tset the debug level as DEBUG\n\t--trace:\t\tset the log level as TRACE".replace("\t", "    ");
    if args.len() > 0 && !["-h", "--help"].contains(&args[0].as_str()) {
        let sub_command: SubCommand = Info::from(args.clone()).into();
        //println!("parsed info: {:?}", sub_command);
        if let SubCommand::Null = sub_command {
            println!(
                "Unknow arguments: \"{}\". Use `dyer-cli -h` to see help",
                args.join(" ")
            );
        } else {
            sub_command.execute();
        }
    } else if args.len() == 0 {
        println!("{}", msgs);
    } else if args.len() > 0 && ["-h", "--help"].contains(&args[0].as_str()) {
        println!("{}", msgs);
    } else {
        println!(
            "Unknow arguments: \"{}\". Use `dyer-cli -h` to see help",
            args.join(" ")
        );
    }
}
