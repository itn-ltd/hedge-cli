use clap::{ App, Arg, ArgMatches, SubCommand, Result };
use crate::hedge::{self, set_incrementer};

pub struct SetFolderFormatCommand;

impl SetFolderFormatCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
            .arg(Arg::with_name("format")
                .index(1)
                .required(true)
                .takes_value(true))
        .about("Define the Folder Format used by Hedge to create folders on destinations.")
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        let format = args.value_of("format").expect("Format has not been provided.");
        
        hedge::set_folder_format(format).map_err(|err| clap::Error::from(err))
    }
}

pub struct SetIncrementerCommand;

impl SetIncrementerCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
            .arg(Arg::with_name("incrementer")
                .index(1)
                .required(true)
                .takes_value(true))
        .about("Sets the next source counter.")
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        let incrementer = args.value_of("incrementer").expect("Incrementer has not been provided.");
        
        hedge::set_incrementer(incrementer).map_err(|err| clap::Error::from(err))
    }
}

pub struct SetPreferencesCommand;

impl SetPreferencesCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
            .arg(Arg::with_name("preferences")
                .index(1)
                .required(true)
                .takes_value(true))
        .about("Sets a specific preference to a specific value.")
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        let preferences = args.value_of("preferences").expect("Preferences have not been provided.");
        
        hedge::set_preferences(preferences).map_err(|err| clap::Error::from(err))
    }
}