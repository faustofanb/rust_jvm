use std::{fmt, fs};
use std::fmt::{Display, Formatter};

use crate::classpath::entry::{Entry, PATH_SEPARATOR};
use crate::classpath::zip_entry::ZipEntry;

pub struct WildCardEntry{
    entries: Vec<Box<dyn Entry>>
}

impl WildCardEntry {
    pub fn new(path: &str)-> Self{
        let mut entries: Vec<Box<dyn Entry>> = vec![];

        let base_dir = &path[..path.len() - 1];
        let file = fs::read_dir(base_dir).unwrap();

        for entry in file {
            let p = entry.unwrap().path();
            if p.is_dir()  {
                continue;
            }

            let str = p.to_str().unwrap();
            if str.ends_with(".jar") || str.ends_with(".JAR") {
                entries.push(Box::new(ZipEntry::new(str)))
            }else {
                continue;
            }
        }

        WildCardEntry{
            entries
        }
    }
}


impl Entry for WildCardEntry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {
        for entry in &self.entries {
            match entry.read_class(class_name) {
                Ok(data) => {return Ok(data)}
                //当前的压缩包未找到对应的class文件， 读取下一个压缩包
                Err(_) => continue
            }
        }
        Err(format!("{class_name} not found"))
    }
}

impl Display for WildCardEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut vec = vec![];
        self.entries.iter().for_each(|entry|{
            vec.push(format!("{entry}"));
        });
        write!(f, "{}", vec.join(&PATH_SEPARATOR.to_string()))
    }
}