use clap::{ ArgMatches };

pub struct SetSourceCommand;

impl SetSourceCommand {
    pub fn execute(args: &ArgMatches) -> Result<()> {
        let api_key = args.value_of("api_key")
            .expect("API Key not provided.");
        let temi = Client::new(api_key);
        let result = temi.get_account_details();

        match result {
            Ok(account) => println!("{:?}", account),
            Err(GetAccountDetailsErrorKind::Unauthorized(_)) => println!("Unauthorized!"),
            Err(err) => println!("{:?}", err)
        }

        Ok(())
    }
}