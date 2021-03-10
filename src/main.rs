use clap::{ App, AppSettings, Result };
use self::commands::app::{ OpenCommand, QuitCommand, RestartCommand, UpdateCommand };

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
        .subcommand(UpdateCommand::from_name("update"));

    match app.get_matches().subcommand() {
        ("open", Some(cmd)) => OpenCommand::execute(cmd),
        ("quit", Some(cmd)) => QuitCommand::execute(cmd),
        ("restart", Some(cmd)) => RestartCommand::execute(cmd),
        ("update", Some(cmd)) => UpdateCommand::execute(cmd),
        _ => Ok(())
    }
}
