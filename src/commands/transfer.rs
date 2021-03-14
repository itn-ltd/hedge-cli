use clap::{ App, Arg, ArgMatches, SubCommand, Result };
use crate::hedge::{self, reset_disks};

pub struct SetSourceCommand;

impl SetSourceCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
            .arg(Arg::with_name("paths")
                .index(1)
                .required(true)
                .takes_value(true)
                .multiple(true))
            .arg(Arg::with_name("label")
                .short("l")
                .long("label")
                .required(false)
                .takes_value(true))
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        let label = args.value_of("label");
        let paths = args.values_of("paths").expect("Paths have not been provided.");

        hedge::set_source(paths, label).map_err(|err| clap::Error::from(err))
    }
}

pub struct ResetDisksCommand;

impl ResetDisksCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
            .arg(Arg::with_name("types")
                .short("t")
                .long("type")
                .required(false)
                .multiple(true)
                .takes_value(true)
                .possible_values(&["sources", "destinations"]))
            .about("Set sources and/or destinations to unused. Existing transfers are not affected. If type is not defined, both sources and destinations will be reset.")
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        let type_val = args.values_of("types").map(|mut val| val.next().unwrap());
        
        hedge::reset_disks(type_val).map_err(|err| clap::Error::from(err))
    }
}

pub struct SetDestinationCommand;

impl SetDestinationCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
            .arg(Arg::with_name("path")
                .index(1)
                .required(true)
                .takes_value(true))
            .about("Set a destination.")
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        let path = args.value_of("path").expect("Path has not been provided.");

        hedge::set_destination(path).map_err(|err| clap::Error::from(err))
    }
}

pub struct AddTransfersCommand;

impl AddTransfersCommand {
    pub fn from_name(name: &str) -> App {
        SubCommand::with_name(name)
            .about("Creates transfers for all new combinations of sources and destinations. Identical to the Add Transfers button in Hedge.")
    }

    pub fn execute(args: &ArgMatches) -> Result<()> {
        hedge::add_transfers().map_err(|err| clap::Error::from(err))
    }
}