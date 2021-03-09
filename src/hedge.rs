use std::process::Command;
use std::io::Error;

type Result = std::result::Result<(), Error>;

pub fn action(url: &str) -> Result {
    Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg(url)
        .spawn()
        .map(|_| ())
}