use crate::util::LogLevel;

// dyer-cli run --info/--debug/warn
#[derive(std::fmt::Debug)]
pub struct SubComRun {
    pub option: Option<LogLevel>,
}

impl SubComRun {
    pub fn execute(&self) {
        std::process::Command::new("sh")
            .arg("-c")
            .arg("cargo")
            .arg("run")
            .output()
            .expect("failed to call cargo run");
    }
}
