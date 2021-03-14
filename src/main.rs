use clap::{ App, AppSettings, Result };
use self::commands::{ ActionsCommand };
use self::commands::app::{ OpenCommand, QuitCommand, RestartCommand, UpdateCommand };
use self::commands::transfer::{ SetSourceCommand, ResetDisksCommand, SetDestinationCommand, AddTransfersCommand };

mod commands;
mod hedge;

fn main() -> Result<()> {
    let app = App::new("Hedge CLI")
        .version("0.1.0")
        .author("Nick Moores <nick.moores@itn.co.uk>")
        .about("CLI for interacting with the Hedge application.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(OpenCommand::from_name("open"))
        .subcommand(QuitCommand::from_name("quit"))
        .subcommand(RestartCommand::from_name("restart"))
        .subcommand(UpdateCommand::from_name("update"))
        .subcommand(SetSourceCommand::from_name("source"))
        .subcommand(ResetDisksCommand::from_name("reset"))
        .subcommand(SetDestinationCommand::from_name("destination"))
        .subcommand(AddTransfersCommand::from_name("transfers"))
        .subcommand(ActionsCommand::from_name("actions"));

    match app.get_matches().subcommand() {
        ("open", Some(cmd)) => OpenCommand::execute(cmd),
        ("quit", Some(cmd)) => QuitCommand::execute(cmd),
        ("restart", Some(cmd)) => RestartCommand::execute(cmd),
        ("update", Some(cmd)) => UpdateCommand::execute(cmd),
        ("source", Some(cmd)) => SetSourceCommand::execute(cmd),
        ("reset", Some(cmd)) => ResetDisksCommand::execute(cmd),
        ("destination", Some(cmd)) => SetDestinationCommand::execute(cmd),
        ("transfers", Some(cmd)) => AddTransfersCommand::execute(cmd),
        ("actions", Some(cmd)) => ActionsCommand::execute(cmd),
        _ => Ok(())
    }
}
