pub mod transfer;
pub mod app;

use crate::hedge;
use clap::{ App, Arg, ArgMatches, SubCommand, Result };
use std::{fs};

pub struct ActionsCommand;

impl ActionsCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
            .arg(Arg::with_name("file")
                .short("i")
                .long("in")
                .required(false)
                .takes_value(true))
            .arg(Arg::with_name("json")
                .short("j")
                .long("json")
                .required(false)
                .takes_value(true))
            .about("Executes a sequence of actions.")
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        let wait = args.is_present("wait");
        let json_arg = args.value_of("json");

        if let Some(json) = json_arg {
            return hedge::actions(json, wait).map_err(|err| clap::Error::from(err));
        }

        if let Some(file_path) = args.value_of("file") {
            let json = fs::read_to_string(file_path).expect("Something went wrong reading the file");
            return hedge::actions(&json, wait).map_err(|err| clap::Error::from(err));
        }

        Ok(())
    }
}