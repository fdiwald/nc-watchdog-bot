use crate::{api::ApiWrapper, Error, config::WatchdogConfig};

pub struct Controller<'a> {
    pub api: ApiWrapper<'a>,
}

impl<'a> Controller<'a> {
    pub(crate) fn new(config: &'a WatchdogConfig) -> Controller<'a> {
        Controller {
            api: ApiWrapper { config },
        }
    }

    pub(crate) fn list_updates(&self) -> Result<(), Error> {
        for update in &self.api.get_updates()? {
            println!("Update {0}: {1:#?}", update.update_id, update.content);
        }
        Ok(())
    }
}
