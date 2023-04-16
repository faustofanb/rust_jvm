use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{PathBuf};
use crate::classpath::entry::{absolute, Entry};

pub struct DirEntry{
    absolute_dir: String
}

impl DirEntry {
    pub fn new(path: &str)-> DirEntry{
        DirEntry{ absolute_dir: absolute(path) }
    }
}


impl Entry for DirEntry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {
        let path = PathBuf::from(&self.absolute_dir).join(class_name);
        let mut read_buf = vec![];
        match File::open(path.as_path()) {
            Ok(file) => {
                BufReader::new(file).read_to_end(&mut read_buf).expect("Read file fail.");
                Ok(read_buf)
            }
            Err(e) => Err(format!("file [{class_name}] not found: {e}"))
        }
    }
}

impl Display for DirEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "absolute directory: {}", self.absolute_dir)
    }
}



#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_read_class(){
        let entry = DirEntry {
            absolute_dir: String::from("C:/Users/10962/Desktop/Deligence/Rust/rust_jvm/src/classpath")
        };
        let data = entry.read_class("dir_entry.rs").unwrap();
        println!("{entry}");
        println!("{:?}", data);
    }
}























