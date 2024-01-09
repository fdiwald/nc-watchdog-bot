use std::env::args;

use controller::Controller;

use crate::config::WatchdogConfig;

mod api;
mod config;
mod controller;

fn main() -> Result<(), Error> {
    let config: WatchdogConfig =
        confy::load("nc-watchdog-bot", None).or_else(|err| Err(Error::ConfyError(err))).expect("error loading config");

    let controller = Controller::new(&config);

    println!("args: {0:?}", args());
    if args().skip(1).take(1).any(|arg| arg == "list-updates") {
        println!("listing updates...");
        controller.list_updates()?;
        print!("done listing updates.");
    }

    Ok(())
}

#[derive(Debug)]
enum Error {
    ConfyError(confy::ConfyError),
    FrankensteinError(frankenstein::Error),
    NoChatIdConfigured(String),
}

impl From<frankenstein::Error> for Error {
    fn from(value: frankenstein::Error) -> Self {
        Self::FrankensteinError(value)
    }
}

impl From<confy::ConfyError> for Error {
    fn from(value: confy::ConfyError) -> Self {
        Self::ConfyError(value)
    }
}
