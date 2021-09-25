use crate::util::{get_file_intro, get_file_path, LogLevel};
use std::io::Write;

// dyer new <+name+>
#[derive(std::fmt::Debug)]
pub struct SubComNew {
    pub name: String,
    pub option: Option<LogLevel>,
}

impl SubComNew {
    /*
     *|___Cargo.toml
     *|___Readme.md
     *|___data/
     *|___data/tasks/
     *|___src/
     *    |___src/entity.rs
     *    |___src/parser.rs
     *    |___src/spider.rs
     *    |___src/middleware.rs
     *    |___src/pipeline.rs
     */
    pub fn execute(&self) {
        let name = &self.name;
        let level = match self.option.as_ref().unwrap() {
            LogLevel::Error => "Error",
            LogLevel::Warn => "Warn",
            LogLevel::Info => "Info",
            LogLevel::Debug => "Debug",
            LogLevel::Trace => "Trace",
        };
        std::fs::create_dir_all(format!("{}/data/tasks/", name)).unwrap();
        std::fs::create_dir_all(format!("{}/src", name)).unwrap();
        std::fs::create_dir_all(format!("{}/src/bin/", name)).unwrap();
        let indexs = [
            "cargo",
            "readme",
            "entity",
            "parser",
            "spider",
            "middleware",
            "pipeline",
            "config",
        ];
        indexs.iter().for_each(|index| {
            let path = get_file_path(index, name.clone());
            let buf = get_file_intro(index)
                .replace("<+name+>", name)
                .replace("<+log_level+>", level);
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(path)
                .unwrap();
            file.write(buf.as_bytes()).unwrap();
        });
        println!("project {} is created successfully!", name);
    }
}
