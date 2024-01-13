use serde::Deserialize;
use serde::Serialize;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WatchdogConfig {
    pub api_token: Option<String>,
    pub chat_id: Option<String>,
    pub user_id: Option<String>,
    pub log_path: Option<String>,
    pub monitor_disks: Option<Vec<String>>,
}

impl WatchdogConfig {
    pub(crate) fn with_missing_values(&self) -> WatchdogConfig {
        fn clone_or_default<T:Clone + From<D>, D>(option: &Option<T>, default: D) -> Option<T> {
            Some(if let Some(content) = option.as_ref() {
                content.clone()
            } else {
                T::from(default)
            })
        }

        WatchdogConfig {
            api_token: clone_or_default(&self.api_token, "<API-Token>"),
            chat_id: clone_or_default(&self.chat_id, "<Chat-ID>"),
            user_id: clone_or_default(&self.user_id, "<User-ID>"),
            log_path: clone_or_default(&self.log_path, "/media/backup"),
            monitor_disks: clone_or_default(&self.monitor_disks, vec![
                String::from("/dev/sda1"),
                String::from("/dev/sdb1"),
            ]),
        }
    }
}
