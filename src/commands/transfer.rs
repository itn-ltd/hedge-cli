use clap::{ ArgMatches };
mod hedge;

pub struct SetSourceCommand;

impl SetSourceCommand {
    pub fn execute(args: &ArgMatches) -> Result<()> {
        let label = args.value_of("label");
        let sources = args.values_of("sources").expect("");

        hedge::set_source(sources, label);
        Ok(());
    }
};