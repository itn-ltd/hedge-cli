pub mod transfer;
pub mod app;

use crate::hedge;
use clap::{ App, Arg, ArgMatches, SubCommand, Result };

pub struct ActionsCommand;

impl ActionsCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
            .arg(Arg::with_name("file")
                .short("i")
                .long("in")
                .required(false))
            .arg(Arg::with_name("json")
                .short("j")
                .long("json")
                .required_unless(false))
            .about("Executes a sequence of actions.")
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        let wait = if args.is_present("wait") { Some(true) } else { None };
        hedge::open(wait).map_err(|err| clap::Error::from(err))
    }
}