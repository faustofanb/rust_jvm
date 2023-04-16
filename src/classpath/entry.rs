use std::fmt::Display;
use std::path::Path;
use crate::classpath::composite_entry::CompositeEntry;
use crate::classpath::dir_entry::DirEntry;
use crate::classpath::wildcard_entry::WildCardEntry;
use crate::classpath::zip_entry::ZipEntry;

#[cfg(windows)]
pub const PATH_SEPARATOR: char = ';';
#[cfg(target_os = "linux")]
const PATH_SEPARATOR: char = ':';

pub trait Entry: Display{
    fn read_class(&self, class_name: &str)-> Result<Vec<u8>, String>;
}

pub fn new_entry(path: &str)-> Box<dyn Entry>{
    if path.contains(PATH_SEPARATOR) {
        return Box::new(CompositeEntry::new(path));
    }
    if path.ends_with('*'){
        return Box::new(WildCardEntry::new(path));
    }
    if path.ends_with(".jar") || path.ends_with(".JAR") ||
       path.ends_with(".zip") || path.ends_with(".ZIP"){
        return Box::new(ZipEntry::new(path));
    }

    Box::new(DirEntry::new(path))
}

///
/// get absolute directory
///
pub fn absolute(path: &str)-> String{
    let p = Path::new(path);
    match p.canonicalize() {
        Ok(path_buf) => {
            path_buf.to_str().unwrap().to_string()
        }
        Err(err) => panic!("{err}")
    }
}