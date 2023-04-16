use crate::classpath::entry::Entry;
use crate::classpath::ClassPath;
use crate::cmd::Cmd;
use clap::Parser;

mod classpath;
mod cmd;

fn main() {
    let _str1 = "HelloWorld";

    let cmd = Cmd::parse();

    start_jvm(cmd);
}

fn start_jvm(cmd: Cmd) {
    let jre = &cmd.jre;
    let class = &cmd.class;
    let cp = match cmd.class_path {
        None => "".to_string(),
        Some(classpath) => classpath,
    };

    println!(
        "\nclasspath: {jre}\t class: {}\t args: {:?}\n",
        &class, cmd.args
    );

    let class_path = ClassPath::parse(jre, &cp);
    let class_name = class.replace('.', "/");

    match class_path.read_class(class_name.as_str()) {
        Ok(data) => println!("class data: \n {:?}", data),
        Err(err) => panic!("Could not find or load main class {}: {}", class, err),
    }
}
