use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WatchdogConfig {
    pub api_token: Option<String>,
    pub chat_id: Option<String>,
    pub user_id: Option<String>,
    pub log_path: Option<String>,
    pub monitored_disks: Option<Vec<String>>,
    pub error_files: Option<Vec<String>>,
}

impl WatchdogConfig {
    pub(crate) fn with_missing_values(&self) -> WatchdogConfig {
        fn clone_or_default<T: Clone + From<D>, D>(option: &Option<T>, default: D) -> Option<T> {
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
            monitored_disks: clone_or_default(
                &self.monitored_disks,
                vec![String::from("/"), String::from("/media/backup")],
            ),
            error_files: clone_or_default(
                &self.error_files,
                vec![String::from("/var/log/backup.err.log"), String::from("/var/log/server.err.log")],
            ),
        }
    }
}
