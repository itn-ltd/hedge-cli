use clap::{ ArgMatches };
mod hedge;

pub struct SetSourceCommand;

impl SetSourceCommand {
    pub fn execute(args: &ArgMatches) -> Result<()> {
        let api_key = args.value_of("label");

        hedge::action("")

        Ok(())
    }
}