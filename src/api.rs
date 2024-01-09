use crate::{config::WatchdogConfig, Error};
use frankenstein::{Api, GetUpdatesParams, SendMessageParams, TelegramApi, Update};

pub(crate) struct ApiWrapper<'a> {
    pub(crate) config: &'a WatchdogConfig,
}

impl<'a> ApiWrapper<'a> {
    pub(crate) fn send_text_message(&self, message: &str) -> Result<(), Error> {
        let chat_id = self
            .config
            .chat_id
            .clone()
            .ok_or_else(|| Error::NoChatIdConfigured(String::from("send_text_message")))?;
        let send_message_parms = SendMessageParams::builder()
            .chat_id(chat_id)
            .text(message)
            .build();

        let api = Api::new(&self.config.api_token);
        api.send_message(&send_message_parms)?;
        Ok(())
    }

    pub(crate) fn get_updates(&self) -> Result<Vec<Update>, Error> {
        let get_updates_parms = GetUpdatesParams::builder().build();
        let api = Api::new(&self.config.api_token);
        Ok(api
            .get_updates(&get_updates_parms)
            .and_then(|method_result| Ok(method_result.result))?)
    }
}
