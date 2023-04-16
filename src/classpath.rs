use std::env;
use std::fmt::{Display, Formatter};
use std::path::Path;
use crate::classpath::entry::{Entry, new_entry};
use crate::classpath::wildcard_entry::WildCardEntry;

pub mod entry;
mod composite_entry;
mod wildcard_entry;
mod zip_entry;
mod dir_entry;

pub struct ClassPath{
    boot_classpath: Box<dyn Entry>,
    ext_classpath:  Box<dyn Entry>,
    user_classpath: Box<dyn Entry>,
}

impl ClassPath {
    pub fn parse(jre_option: &str, cp_option: &str)-> Self{
        let boot = ClassPath::parse_boot(jre_option);
        let ext = ClassPath::parse_ext(jre_option);
        let user = ClassPath::parse_user(cp_option);
        ClassPath{
            boot_classpath: boot,
            ext_classpath: ext,
            user_classpath: user,
        }
    }
}

impl Entry for ClassPath {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {
        let class_name = class_name.to_string() + ".class";

        if let Ok(data) = self.boot_classpath.read_class(class_name.as_str()) {
            return Ok(data);
        }
        if let Ok(data) = self.ext_classpath.read_class(class_name.as_str()) {
            return Ok(data);
        }
        if let Ok(data) = self.user_classpath.read_class(class_name.as_str()) {
            return Ok(data);
        }

        Err(format!("find {class_name} fail"))
    }
}

impl ClassPath {
    fn parse_boot(jre_option: &str) -> Box<dyn Entry> {
        let jre_dir = ClassPath::read_jre_dir(jre_option);
        Box::new(
            WildCardEntry::new(
                Path::new(&jre_dir)
                    .join("lib")
                    .join("*")
                    .to_str().unwrap()
            )
        )
    }
    fn parse_ext(jre_option: &str)-> Box<dyn Entry>{
        let jre_dir = ClassPath::read_jre_dir(jre_option);
        Box::new(
            WildCardEntry::new(
                Path::new(&jre_dir)
                    .join("lib")
                    .join("ext")
                    .join("*")
                    .to_str().unwrap()
            )
        )
    }
    fn parse_user(user_option: &str)-> Box<dyn Entry>{
        let classpath = if user_option.is_empty() {
            "."
        }else {
            user_option
        };
        new_entry(classpath)
    }
    fn read_jre_dir(jre_option: &str)-> String{
        // use -Xjre by user input
        if !jre_option.is_empty() && Path::new(jre_option).exists() {
            return jre_option.to_string();
        }
        // use current directory jre
        if Path::new(".jre").exists() {
            return ".jre".to_string();
        }
        // use JAVA_HOME
        match env::var("JAVA_HOME") {
            Ok(java_home) if !java_home.is_empty() => {
                return Path::new(&java_home).join("jre").to_str().unwrap().to_string();
            }
            _ => {}
        }
        // not find jre
        panic!("can't find jre folder!")
    }
}

impl Display for ClassPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[boot: {}, ext: {}, user: {}]",
               self.boot_classpath,
               self.ext_classpath,
               self.user_classpath)
    }
}