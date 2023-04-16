use std::fmt::{Display, Formatter};
use crate::classpath::entry::{Entry, new_entry, PATH_SEPARATOR};

pub struct CompositeEntry{
    entries: Vec<Box<dyn Entry>>
}

impl CompositeEntry {
    pub fn new(path_list: &str)-> Self{
        let mut entries = vec![];
        path_list.split(PATH_SEPARATOR).for_each(|path|{
            entries.push(new_entry(path));
        });

        CompositeEntry{
            entries,
        }
    }
}

impl Entry for CompositeEntry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {
        for entry in &self.entries {
            match entry.read_class(class_name) {
                Ok(data) => {
                    return Ok(data);
                }
                Err(_) => {}
            }
        }
        Err(format!("{class_name} not found"))
    }
}

impl Display for CompositeEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut vec = vec![];
        self.entries.iter().for_each(|entry|{
            vec.push(format!("{entry}"));
        });
        write!(f, "{}", vec.join(&PATH_SEPARATOR.to_string()))
    }
}

#[cfg(test)]
mod test{
    use crate::classpath::composite_entry::CompositeEntry;
    #[test]
    fn test_display(){
        let entries
            = CompositeEntry::new("C:/Users/10962/Desktop/Deligence/Rust/rust_jvm/src/main.rs;C:/Users/10962/Desktop/Deligence/Rust/rust_jvm/src/cmd.rs");
        // entries.iter().for_each(|entry|{
        //     println!("{entry}");
        // });
        println!("{entries}");
    }
}