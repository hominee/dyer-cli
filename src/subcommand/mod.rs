pub mod new;
pub mod run;
pub mod check;
pub mod fix;

pub use check::SubComCheck;
pub use fix::SubComFix;
pub use new::SubComNew;
pub use run::SubComRun;

#[derive(std::fmt::Debug)]
pub enum SubCommand {
    SubComNew(SubComNew),
    SubComRun(SubComRun),
    SubComFix(SubComFix),
    SubComCheck(SubComCheck),
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
            _ => {}
        }
    }
}
