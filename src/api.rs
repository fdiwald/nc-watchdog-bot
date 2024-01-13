use std::rc::Rc;

use crate::{config::WatchdogConfig, WatchdogError};
use frankenstein::{Api, GetUpdatesParams, SendMessageParams, TelegramApi, Update};

pub(crate) struct ApiWrapper {
    config: Rc<WatchdogConfig>,
    api: Api,
}

impl ApiWrapper {
    pub(crate) fn new(config: Rc<WatchdogConfig>) -> Result<ApiWrapper, WatchdogError> {
        let api =
            Api::new(config.api_token.as_ref().ok_or_else(|| {
                WatchdogError::NoApiKeyConfigured(String::from("ApiWrapper::new"))
            })?);

        Ok(ApiWrapper { config, api })
    }

    pub(crate) fn send_text_message(&self, message: &str) -> Result<(), WatchdogError> {
        let chat_id =
            self.config.chat_id.clone().ok_or_else(|| {
                WatchdogError::NoChatIdConfigured(String::from("send_text_message"))
            })?;
            let send_message_parms = SendMessageParams::builder()
            .chat_id(chat_id)
            .text(message)
            .build();
        
        println!("Sending message...");
        self.api.send_message(&send_message_parms)?;
        Ok(())
    }

    pub(crate) fn get_updates(&self) -> Result<Vec<Update>, WatchdogError> {
        let get_updates_parms = GetUpdatesParams::builder().build();
        Ok(self
            .api
            .get_updates(&get_updates_parms)
            .and_then(|method_result| Ok(method_result.result))?)
    }
}
