use clap::{Parser};

#[derive(Parser, Debug)]
#[command(
    name    = "RustJVM ",
    version = "0.0.1",
    author  = "FaustoFan. <faustofanb@gmail.com>",
    about   = "A toy java virtual machine on rust.",
    long_about = None
)]
pub struct Cmd {
    #[arg(short, help="user class path")]
    pub class_path: Option<String>,
    pub jre: String,
    pub class: String,
    pub args: Vec<String>,
}

