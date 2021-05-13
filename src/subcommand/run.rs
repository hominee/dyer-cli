use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;
use crate::util;

// dyer-cli run --info/--debug/warn
#[derive(std::fmt::Debug)]
pub struct SubComRun {
    pub options: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct MetaData {
    modules: HashMap<String, Module>,
    pkgs: Vec<String>,
    ctype: String,
    base_dir: String,
    package_name: String,
}

impl MetaData {
    pub fn new() -> Self {
        MetaData {
            modules: HashMap::new(),
            pkgs: vec!["std".to_string()],
            ctype: String::new(),
            base_dir: "./".into(),
            package_name: String::new(),
        }
    }

    pub(crate) fn init(&mut self) {
        self.get_pkg();
        let paths = ["middleware", "pipeline", "parser", "entity", "spider"];
        let raw_pat = r"(?sm)^\s*#\[(?P<module>(middleware)|(pipeline)|(entity)|(spider)|(parser))(\(\s*(?P<key>\w+)\s*\))?\].*?(?P<typ>(fn)|(struct)|(enum))\s*(?P<ident>\w+)(.*?Option<(?P<ctyp>.*?)>)?";

        for i in 0..paths.len() {
            let pat = regex::Regex::from_str(&raw_pat).unwrap();
            let path = format!("{}src/{}.rs", self.base_dir, paths[i]);
            let mut file = std::fs::File::open(&path).unwrap();
            let mut handles = HashMap::new();
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            for cap in pat.captures_iter(&buf) {
                let module = cap.name("module").unwrap().as_str();
                let value = cap.name("ident").unwrap().as_str().to_string();
                let key = if ["spider", "parser"].contains(&module) {
                    value.clone()
                } else {
                    cap.name("key").unwrap().as_str().to_string()
                };
                if paths[i] == "pipeline" && &key == "open_pipeline" {
                    let ctype = cap.name("ctyp").unwrap().as_str().to_string();
                    self.ctype = ctype;
                }
                handles.insert(key, value);
            }
            let module = Module { path, handles };
            self.modules.insert(paths[i].to_string(), module);
        }
    }

    pub fn get_pkg(&mut self) {
        let files = std::fs::read_dir(".")
            .unwrap()
            .map(|p|p.unwrap().path().to_str().unwrap().into())
            .collect::<Vec<String>>();
        if !files.iter().fold(false, |acc, file| acc || file.contains(&"Cargo.toml")) {
            panic!("current directory must contain `Cargo.toml` file");
        }
        let path = format!("{}/Cargo.toml", self.base_dir);
        let mut pkgs = Vec::new();
        let file = std::fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let pat = regex::Regex::new(r"\s*([\w|-]+)\s*=\s*").unwrap();
        let pat1 = regex::Regex::new(r"^\s*name\s*=.*?(?P<pkg_name>[\w|-]+)").unwrap();
        let pat2 = regex::Regex::new(r"^\s*\[dependencies\]").unwrap();
        let pat3 = regex::Regex::new(r"^\s*\[.*?\]").unwrap();
        let mut in_content = false;
        for line in reader.lines() {
            let text = line.unwrap();
            if pat2.is_match(&text) {
                in_content = true;
            } else if !pat2.is_match(&text) && pat3.is_match(&text) {
                in_content = false;
            }
            if in_content {
                if let Some(t) = pat.captures(&text) {
                    let pkg = t.get(1).unwrap().as_str().trim().replace("-", "_");
                    pkgs.push(pkg)
                }
            }
            if pat1.is_match(&text) {
                let name = pat1.captures(&text).unwrap().name("pkg_name").unwrap().as_str().replace("-", "_");
                self.package_name = name.into();
            }
        }
        self.pkgs.extend(pkgs);
    }

    fn complete_path(&self) -> String {
        let pieces = self
            .ctype
            .split("::")
            .map(|piece| piece.trim())
            .collect::<Vec<&str>>();
        let subpath = pieces[0].to_string();
        if !self.pkgs.contains(&subpath) {
            panic!("The return type of `open_pipeline` must starts with one of `{}`, not subpath: `{}`", &self.pkgs.join(" "), subpath);
        }
        "".into()
    }

    pub fn get_pkg_list(&self) -> String {
        let list = self.pkgs.iter().filter(|&ele| ele != "std" ).map(|md| format!("extern crate {};", md) ).collect::<Vec<String>>();
        list.join("\n")
    }

    pub fn make_main(&self) {
        let entity = self.modules.get("entity").expect("entity cannot be none");
        let entities = entity.handles.get("entities").unwrap();
        let targ = entity.handles.get("targ").unwrap();
        let parg = entity.handles.get("parg").unwrap();
        let spider = self
            .modules
            .get("spider")
            .unwrap()
            .handles
            .values()
            .collect::<Vec<&String>>()[0];
        let get_middleware_list = self.modules.get("middleware").unwrap().get_list();
        let get_pipeline_list = self.modules.get("pipeline").unwrap().get_list();
        let get_pipeline_map = self.modules.get("pipeline").unwrap().get_map();
        let get_middleware_map = self.modules.get("middleware").unwrap().get_map();
        let ctype = &self.ctype;
        let ctype_import = self.complete_path();
        let get_pkg_list = self.get_pkg_list();
        let package_name = &self.package_name;

        let main_str = r"//#![allow(unused_imports)]

<+get_pkg_list+>

use <+package_name+>::*; 
use dyer::*;
use entity::{<+entities+>, <+targ+>, <+parg+>};
use spider::<+spider+>;
use middleware::{<+get_middleware_list+>};
use pipeline::{<+get_pipeline_list+>};
use std::sync::{Arc, Mutex};
<+ctype_import+>

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    let middleware = plug!( MiddleWare<<+entities+>, <+targ+>, <+parg+>> {
        <+get_middleware_map+>
    });
    let pipeline = plug!( PipeLine<<+entities+>, <+ctype+>> {
        <+get_pipeline_map+>
    } );
    let spider = <+spider+>::new();
    let mut app = dyer::App::<<+entities+>, <+targ+>, <+parg+>>::new();
    app.run(&spider, &middleware, pipeline).await.unwrap();
}
        ";
        let main_str = main_str.replace("<+package_name+>", &package_name);
        let main_str = main_str.replace("<+entities+>", &entities);
        let main_str = main_str.replace("<+targ+>", &targ);
        let main_str = main_str.replace("<+parg+>", &parg);
        let main_str = main_str.replace("<+spider+>", &spider);
        let main_str = main_str.replace("<+get_pkg_list+>", &get_pkg_list);
        let main_str = main_str.replace("<+get_middleware_list+>", &get_middleware_list);
        let main_str = main_str.replace("<+get_middleware_map+>", &get_middleware_map);
        let main_str = main_str.replace("<+get_pipeline_list+>", &get_pipeline_list);
        let main_str = main_str.replace("<+get_pipeline_map+>", &get_pipeline_map);
        let main_str = main_str.replace("<+ctype+>", ctype);
        let main_str = main_str.replace("<+ctype_import+>", &ctype_import);
        let main_path = format!("{}.target/main.rs", self.base_dir);
        let mut main_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(main_path)
            .unwrap();
        main_file.write(&main_str.as_bytes()).unwrap();
    }
}

#[derive(Debug)]
struct Module {
    path: String,
    handles: HashMap<String, String>,
}

impl Module {
    pub fn get_list(&self) -> String {
        self.handles
            .values()
            .map(|val| val.as_str())
            .collect::<Vec<&str>>()
            .join(", ")
    }

    pub fn get_map(&self) -> String {
        self.handles
            .iter()
            .map(|(key, val)| format!("{}: {}", key, val))
            .collect::<Vec<String>>()
            .join(",\n        ")
    }
}

impl SubComRun {
    pub fn execute(&self) {
        let paths = std::fs::read_dir("./.target").unwrap().map(|p| p.unwrap().path().to_str().unwrap().into() ).collect::<Vec<String>>();
        //println!("files in \"./\" {:?}", paths);
        if !paths.iter().fold(false, |acc, x| acc || x.contains(&"main.rs".to_owned())) {
            let mut meta = MetaData::new();
            meta.init();
            //println!("{:?}", meta);
            meta.make_main();
        }
        let options = self.options.iter()
            .map(|op| op.as_str())
            .filter(|op| {
                if ["--off", "--error", "--warn", "--info", "--debug", "--trace"].contains(&op) {
                    util::change_log_level(op);
                    return false;
                }
                true
            })
            .collect::<Vec<&str>>();
        let mut args = vec!["run"];
        args.extend( options) ;
        util::run_command("cargo", args);
    }
}
