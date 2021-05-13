pub mod new;
pub mod run;
pub mod check;
pub mod fix;
pub mod build;
pub mod clean;
pub mod test;

pub use check::SubComCheck;
pub use fix::SubComFix;
pub use new::SubComNew;
pub use run::SubComRun;
pub use build::SubComBuild;
pub use clean::SubComClean;
pub use test::SubComTest;

#[derive(std::fmt::Debug)]
pub enum SubCommand {
    SubComNew(SubComNew),
    SubComRun(SubComRun),
    SubComFix(SubComFix),
    SubComCheck(SubComCheck),
    SubComBuild(SubComBuild),
    SubComClean(SubComClean),
    SubComTest(SubComTest),
    Null,
}
impl SubCommand {
    pub fn execute(&self) {
        match self {
            SubCommand::SubComNew(command) => {
                command.execute();
            }
            SubCommand::SubComRun(command) => {
                command.execute();
            }
            SubCommand::SubComFix(command) => {
                command.execute();
            }
            SubCommand::SubComCheck(command) => {
                command.execute();
            }
            SubCommand::SubComBuild(command) => {
                command.execute();
            }
            SubCommand::SubComClean(command) => {
                command.execute();
            }
            SubCommand::SubComTest(command) => {
                command.execute();
            }
            _ => {}
        }
    }
}
