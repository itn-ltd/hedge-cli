use clap::{ App, ArgMatches, SubCommand, Result };
use crate::hedge;

pub struct SetSourceCommand;

impl SetSourceCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        let label = args.value_of("label");
        let sources = args.values_of("sources").expect("");

        hedge::set_source(sources, label).map_err(|err| clap::Error::from(err))
    }
}