use crate::hedge;
use clap::{ App, Arg, ArgMatches, SubCommand, Result };

pub struct OpenCommand;

impl OpenCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
            .arg(Arg::with_name("wait")
                .short("w")
                .long("wait")
                .takes_value(false)
                .required(false))
            .about("Opens Hedge.")
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        let wait = args.is_present("wait");
        hedge::open(wait).map_err(|err| clap::Error::from(err))
    }
}

pub struct QuitCommand;

impl QuitCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
            .about("Quits Hedge.")
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        hedge::quit().map_err(|err| clap::Error::from(err))
    }
}

pub struct RestartCommand;

impl RestartCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
            .about("Restarts Hedge.")
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        hedge::restart().map_err(|err| clap::Error::from(err))
    }
}

pub struct UpdateCommand;

impl UpdateCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
            .about("Checks for updates.")
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        hedge::update().map_err(|err| clap::Error::from(err))
    }
}