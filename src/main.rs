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

        if args().skip(1).take(1).any(|arg| arg == "log-config") {
            println!(
                "{}",
                confy::get_configuration_file_path(app_name, None).map_or_else(
                    |err| format!("Error getting configuration file path{:#?}", err),
                    |path| format!("Configuration file path: {:#?}", path)
                )
            );
            println!("{:#?}", controller.config);
        } else if args().skip(1).take(1).any(|arg| arg == "list-updates") {
            controller.list_updates()?;
        } else if args().skip(1).take(1).any(|arg| arg == "send-report") {
            controller.send_report()?;
        } else if args().skip(1).take(1).any(|arg| arg == "log-report") {
            controller.log_report()?;
        } else {
            println!("A telegram bot that sends you a report about your system's status.");
            println!("");
            println!("Syntax:");
            println!("   nc-watchdog-bot [COMMAND]");
            println!("");
            println!("Commands:");
            println!("  initialize-config   Fill all empty config entries with dummy values, remove unneeded entries and print the config file path to stdout.");
            println!("  log-config          Print the config file's path and content to stdout.");
            println!("  list-updates        Retrieve updates from the telegram server and print them to stdout.");
            println!("  log-report          Write the system report to stdout.");
            println!("  send-report         Create and send the system report to the configured telegram chat.");
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
    IoError(std::io::Error),
    NoApiKeyConfigured(String),
    SystemTimeError(std::time::SystemTimeError),
}

impl From<std::time::SystemTimeError> for WatchdogError {
    fn from(v: std::time::SystemTimeError) -> Self {
        Self::SystemTimeError(v)
    }
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
