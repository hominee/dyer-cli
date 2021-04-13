pub mod new;
//pub mod run;

pub use new::SubComNew;
//pub use run::SubComRun;

#[derive(std::fmt::Debug)]
pub enum SubCommand {
    SubComNew(SubComNew),
    //SubComRun(SubComRun),
    Null,
}
impl SubCommand {
    pub fn execute(&self) {
        match self {
            SubCommand::SubComNew(command) => {
                command.execute();
            }
            /*
             *SubCommand::SubComRun(command) => {
             *    command.execute();
             *}
             */
            _ => {}
        }
    }
}
