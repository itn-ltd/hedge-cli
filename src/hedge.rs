use std::process::Command;
use std::io::Error;

type Result = std::result::Result<(), Error>;

const HEDGE_URL_PREFIX: &str = "hedge://";

pub fn action(url: &str) -> Result {
    let url = String::from(HEDGE_URL_PREFIX)
        .push_str(url);

    Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg(url)
        .spawn()
        .map(|_| ())
}