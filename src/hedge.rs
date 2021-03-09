use std::process::Command;
use std::io::Error;
use std::fmt::Display;

type Result = std::result::Result<(), Error>;

const HEDGE_URL_PREFIX: &str = "hedge://";

fn action(action_str: &str) -> Result {
    let url: String = format!("{}{}", HEDGE_URL_PREFIX, action_str);

    Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg(url)
        .spawn()
        .map(|_| ())
}

pub fn open() -> Result {
    action("open")
}

pub fn quit() -> Result {
    action("quit")
}

pub fn restart() -> Result {
    action("restart")
}

pub fn update() -> Result {
    action("update")
}

pub fn set_source<I, S>(paths: I, label: Option<&str>) -> Result where I: IntoIterator<Item = S>, S: AsRef<str>, S: Display {
    let mut action = String::from("setSource?");

    if label.is_some() {
        action.push_str(&format!(r#"label="{}"&"#, label.unwrap()));
    }

    action.push_str("paths=[");
    for path in paths {
        action.push_str(&format!(r#""{}","#, path));
    }
    action.pop();
    action.push(']');

    println!("{}", action);
    Ok(())
}