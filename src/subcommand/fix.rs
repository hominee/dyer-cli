use crate::util;
use crate::subcommand::run::MetaData;

#[derive(std::fmt::Debug)]
pub struct SubComFix {
    pub options: Vec<String>,
}

impl SubComFix {
    pub fn execute(&self) {
        let paths = std::fs::read_dir("./.target").unwrap().map(|p| p.unwrap().path().to_str().unwrap().into() ).collect::<Vec<String>>();
        //println!("files in \"./\" {:?}", paths);
        if !paths.iter().fold(false, |acc, x| acc || x.contains(&"main.rs".to_owned())) {
            let mut meta = MetaData::new();
            meta.init();
            //println!("{:?}", meta);
            meta.make_main();
            let args = vec!["check"];
            util::run_command("cargo", args);
        }
            let options = self.options.iter().map(|op| op.as_str()).filter(|op| op != &"--allow-no-vcs" ).collect::<Vec<&str>>();
            let mut args = vec!["fix","--allow-no-vcs"];
            args.extend( options) ;
            util::run_command("cargo", args);
    }
}