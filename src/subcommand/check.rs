
use crate::util;
use crate::subcommand::run::MetaData;

#[derive(std::fmt::Debug)]
pub struct SubComCheck {
    pub options: Vec<String>,
}

impl SubComCheck {
    pub fn execute(&self) {
        let paths = std::fs::read_dir("./src/bin").unwrap().map(|p| p.unwrap().path().to_str().unwrap().into() ).collect::<Vec<String>>();
        //println!("files in \"./\" {:?}", paths);
        let pkg_name = util::get_package_name();
        if !paths.iter().fold(false, |acc, x| acc || x.contains(&pkg_name)) {
            let mut meta = MetaData::new();
            meta.init();
            //println!("{:?}", meta);
            meta.make_main();
        }
            let options = self.options.iter().map(|op| op.as_str()).collect::<Vec<&str>>();
            let mut args = vec!["check"];
            args.extend( options) ;
            util::run_command("cargo", args);
    }
}
