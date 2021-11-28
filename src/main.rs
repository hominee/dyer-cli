//! # Introduction
//! [dyer-cli] is a great tool created to guide you use [dyer] fast and at ease, helps you build a robust crawler, data processor, netwrok program fast and correctly.
//!
//! [dyer-cli]: https://github.com/HomelyGuy/dyer-cli
//! [dyer]: https://github.com/HomelyGuy/dyer
//!
//! # Installation
//! dyer-cli is built completely by Rust programming language without extra dependencies, So rust must be installed beforehand, to test it with:
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
//! the command will download the source code and complie it to build a executable file inside your `$HOME/.cargo/bin`, make sure it's in your `$PATH`
//!
//! # Commands
//! Dyer-cli provides some commands that helps you initialize, debug programm, more commands are to go.
//!
//! ## dyer new
//! This command helps you initialize a project with log level `Info`, other log levels vares from `Error`, `Warn`, `Info`, `Debug`, and `Trace`, and its structure is
//! ```bash
//! |___Cargo.toml
//! |___Readme.md
//! |___data/
//! |___data/tasks/
//! |___src/
//!     |___src/entity.rs
//!     |___src/parser.rs
//!     |___src/actor.rs
//!     |___src/middleware.rs
//!     |___src/pipeline.rs
//! ```
//! Main functionality of each file:                                        
//! * the `affix.rs` serves as an actor to adjust and satisfy additional requirement
//! * the `entity.rs` contains entities/data structure to be used/collected
//! * the `parser.rs` contains functions that extract entities from response
//! * the `actor.rs` contains initial when opening and final things to do when closing
//! * the `middleware.rs` contains Some middlewares that process data at runtime
//! * the `pipeline.rs` contains entities manipulation including data-storage, displsying and so on
//! * `Cargo.toml` is the basic configuration of the project
//! * `README.md` contains some instructions of the project
//! * `data` folder balance the app load when data in app exceeds, and backup app data at certain gap
//!
//! ## dyer check
//!
//! A warper of `cargo check`, if you run it the first time,`dyer-cli` will download the crates and then check the code.
//!
//! ## dyer fix
//!
//! A wraper of `cargo fix`,  if some warning happens such as `unused import` or `dead code` the command does a lot for you. However it won't help if some errors occur, if so, you have to debug the code manually.
//!
//! ## dyer run
//!
//! A wraper of `cargo run`, when the program compiles, run it.
//!
//! ## dyer build
//!
//! A wraper of `cargo build`,   build the program.
//!
//! ## dyer test
//!
//! A wraper of `cargo test`,   test the program.
//!
//! ## dyer clean
//!
//! A wraper of `cargo clean`,   clean the directory.

mod subcommand;
mod util;

use subcommand::{
    SubComBuild, SubComCheck, SubComClean, SubComFix, SubComNew, SubComRun, SubComTest, SubCommand,
};
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
                options.push(item);
            } else if item.contains("-") {
                options.push(item);
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
        } else if ["run".into(), "r".into()].contains(&self.sub_command) {
            let item = SubComRun {
                options: self.options,
            };
            comd = SubCommand::SubComRun(item);
        } else if self.sub_command == "fix" {
            let item = SubComFix {
                options: self.options,
            };
            comd = SubCommand::SubComFix(item);
        } else if ["c".into(), "check".into()].contains(&self.sub_command) {
            let item = SubComCheck {
                options: self.options,
            };
            comd = SubCommand::SubComCheck(item);
        } else if ["b".into(), "build".into()].contains(&self.sub_command) {
            let item = SubComBuild {
                options: self.options,
            };
            comd = SubCommand::SubComBuild(item);
        } else if ["t".into(), "test".into()].contains(&self.sub_command) {
            let item = SubComTest {
                options: self.options,
            };
            comd = SubCommand::SubComTest(item);
        } else if ["clean".into()].contains(&self.sub_command) {
            let item = SubComClean {
                options: self.options,
            };
            comd = SubCommand::SubComClean(item);
        }
        comd
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    //println!("raw arguments: {:?}", args);
    args.remove(0); // remove the unnecessary path
    let msgs = "Handy tool for dyer\n\nUSAGE:\n\tdyer [subcommand] [options]\n\teg. dyer new myproject --debug create a project with logger level INFO\n\nSUBCOMMAND:\n\tnew:\t\tinitialize a new empty project\n\tcheck:\t a wraper of `cargo check`\n\tfix:\t\ta wraper of `cargo fix`\n\trun:\t\ta wraper of `cargo run`, compile and run the project\n\tbuild:\t a wraper of `cargo build`\n\ttest:\t  a wraper of `cargo test`\n\tclean:\t a wraper of `cargo clean`\n\nOPTIONS:\n\tall options of `cargo SUBCOMMAND`\n\t--off:\t\t  set the log level as Off\n\t--error:\t\tset the log level as ERROR\n\t--warn: \t\tset the log level as WARN\n\t--info: \t\tset the log level as INFO\n\t--debug:\t\tset the debug level as DEBUG\n\t--trace:\t\tset the log level as TRACE".replace("\t", "   ");
    if args.len() > 0 && !["-h", "--help"].contains(&args[0].as_str()) {
        let sub_command: SubCommand = Info::from(args.clone()).into();
        //println!("parsed info: {:?}", sub_command);
        if let SubCommand::Null = sub_command {
            println!(
                "Unknow arguments: \"{}\". Use `dyer -h` to see help",
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
            "Unknow arguments: \"{}\". Use `dyer -h` to see help",
            args.join(" ")
        );
    }
}
