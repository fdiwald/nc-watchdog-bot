use std::rc::Rc;

use crate::{api::ApiWrapper, config::WatchdogConfig, report::Report, WatchdogError};

pub struct Controller {
    api: ApiWrapper,
    config: Rc<WatchdogConfig>,
}

impl Controller {
    pub(crate) fn new(config: WatchdogConfig) -> Result<Controller, WatchdogError> {
        let rc_config = Rc::new(config);
        Ok(Controller {
            api: ApiWrapper::new(Rc::clone(&rc_config))?,
            config: Rc::clone(&rc_config),
        })
    }

    pub(crate) fn list_updates(&self) -> Result<(), WatchdogError> {
        println!("listing updates...");
        for update in &self.api.get_updates()? {
            println!("Update {0}: {1:#?}", update.update_id, update.content);
        }
        print!("done listing updates.");
        Ok(())
    }

    pub(crate) fn send_report(&self) -> Result<(), WatchdogError> {
        println!("sending report...");
        let message = Report::new(&self.config)?.create_message();
        self.api.send_text_message(&message)?;
        print!("done sending report.");
        Ok(())
    }
    
    pub(crate) fn print_report(&self) -> Result<(), WatchdogError> {
        println!("sending report...");
        let message = Report::new(&self.config)?.create_message();
        print!("{message}");
        Ok(())
    }
}
