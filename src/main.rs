use std::env::args;

use controller::Controller;

use crate::config::WatchdogConfig;

mod api;
mod config;
mod controller;
mod report;

fn main() -> Result<(), WatchdogError> {
    let app_name = "nc-watchdog-bot";

    if Some("initialize-config") == args().nth(1).as_deref() {
        initialize_config(app_name)?;
    } else {
        let config: WatchdogConfig = confy::load(app_name, None)?;
        let controller = Controller::new(config)?;
        
        if args().skip(1).take(1).any(|arg| arg == "list-updates") {
            controller.list_updates()?;
        } else if args().skip(1).take(1).any(|arg| arg == "send-report") {
            controller.send_report()?;
        } else if args().skip(1).take(1).any(|arg| arg == "print-report") {
            controller.print_report()?;
        }
    }

    Ok(())
}

fn initialize_config(app_name: &str) -> Result<(), WatchdogError> {
    println!("initializing config file with dummy values...");
    let config_file_path = confy::get_configuration_file_path(app_name, None)?;
    let config: WatchdogConfig = confy::load(app_name, None)?;

    confy::store(app_name, None, config.with_missing_values())?;
    println!("created {0}", config_file_path.display());
    Ok(())
}

#[derive(Debug)]
enum WatchdogError {
    ConfyError(confy::ConfyError),
    FrankensteinError(frankenstein::Error),
    NoChatIdConfigured(String),
    NoLogPathConfigured(String),
    IoError(std::io::Error),
    NoApiKeyConfigured(String),
    NoMonitorDisksConfigured(String),
}

impl From<std::io::Error> for WatchdogError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<frankenstein::Error> for WatchdogError {
    fn from(value: frankenstein::Error) -> Self {
        Self::FrankensteinError(value)
    }
}

impl From<confy::ConfyError> for WatchdogError {
    fn from(value: confy::ConfyError) -> Self {
        Self::ConfyError(value)
    }
}
