use std::{borrow::Borrow, process::Command as Cmd };
use std::io::Error;
use std::fmt::Display;
use url::Url;

type Result = std::result::Result<(), Error>;

const HEDGE_URL_PREFIX: &str = "hedge://";

struct Command {
    url: Url
}

impl Command {
    fn for_action(name: &str) -> Command {
        let url_str = format!("{}{}", HEDGE_URL_PREFIX, name);

        Command {
            url: Url::parse(&url_str).expect("Unable to construct Hedge URL")
        }
    }

    fn for_action_with_params<I, K, V>(name: &str, params: I) -> Command where I: IntoIterator, I::Item: Borrow<(K, V)>, K: AsRef<str>, V: AsRef<str> {
        let url_str = format!("{}{}", HEDGE_URL_PREFIX, name);

        Command {
            url: Url::parse_with_params(&url_str, params).expect("Unable to construct Hedge URL.")
        }
    }

    fn execute(&self) -> Result {
        println!("Running: {}", self.url.as_str());

        Cmd::new("cmd")
            .arg("/C")
            .arg("start")
            .arg(self.url.as_str())
            .spawn()
            .map(|_| ())
    }

    fn execute_and_wait(&self) -> Result {
        println!("Running: {}", self.url.as_str());

        Cmd::new("cmd")
            .arg("/C")
            .arg("start")
            .arg("/WAIT")
            .arg(self.url.as_str())
            .spawn()
            .and_then(|mut proc| proc.wait())
            .map(|_| ())
    }
}

pub fn actions(actions_str: &str, wait_for_exit: bool) -> Result {
    let cmd = Command::for_action_with_params("actions", &[("json", actions_str)]);

    if wait_for_exit { 
        cmd.execute_and_wait()
    } else {
         cmd.execute()
    }
}

pub fn open(wait_for_exit: bool) -> Result {
    let cmd = Command::for_action("open");

    if wait_for_exit {
        cmd.execute_and_wait()
    } else {
        cmd.execute()
    }
}

pub fn quit() -> Result {
    Command::for_action("quit").execute()
}

pub fn restart() -> Result {
    Command::for_action("restart").execute()
}

pub fn update() -> Result {
    Command::for_action("update").execute()
}

pub fn set_source<I, S>(paths: I, label: Option<&str>) -> Result where I: IntoIterator<Item = S>, S: AsRef<str>, S: Display {
    let mut params = vec![("paths", "")];

    if label.is_some() {
        params.push(("label", label.unwrap()));
    }
    
    Command::for_action_with_params("setSource", params).execute()
}

pub fn reset_disks(types: Option<&str>) -> Result {
    let cmd = if let Some(t) = types {
        Command::for_action_with_params("reset", &[("type", t)])
    } else {
        Command::for_action("reset")
    };

    cmd.execute()
}

pub fn set_destination(destination: &str) -> Result {
    Command::for_action_with_params("setDestination", &[("path", destination)]).execute()
}

pub fn add_transfers() -> Result {
    Command::for_action("addTransfers").execute()
}