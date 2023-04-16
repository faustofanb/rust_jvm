use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use zip::ZipArchive;

use crate::classpath::entry::Entry;

pub struct ZipEntry{
    pub absolute_path: String,
}

impl ZipEntry {
    pub fn new(path: &str)-> Self{
        ZipEntry{
            absolute_path: path.to_string()
        }
    }
}

impl Entry for ZipEntry {
    ///
    /// 只能读取压缩包解压后，一级目录中的文件
    ///
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {

        let file =  File::open(PathBuf::from(&self.absolute_path).as_path())
            .expect("Failed to open jar file");

        let mut zip_archive = ZipArchive::new(BufReader::new(file))
            .expect("Failed to unzip jar file");

        let target = match zip_archive.by_name(class_name) {
            Ok(file) => file,
            Err(_) => {return Err("".to_string())}
        };

        let mut buf = vec![];
        BufReader::new(target)
            .read_to_end(&mut buf)
            .expect("read file error");

        Ok(buf)
    }
}

impl Display for ZipEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zip entry's absolute path: {}", self.absolute_path)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_read_zip(){
        let zip = ZipEntry::new("C:\\Users\\10962\\Desktop\\Deligence\\Rust\\rust_jvm\\java\\jre\\lib\\rt.jar");
        let result = zip.read_class("java/lang/Object.class").unwrap();
        println!("{:?}", result.to_vec());
    }
}










