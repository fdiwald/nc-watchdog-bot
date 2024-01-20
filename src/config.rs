use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Default, Deserialize)]
pub(crate) struct WatchdogConfig {
    pub api_token: Option<String>,
    pub chat_id: Option<String>,
    pub user_id: Option<String>,
    pub monitored_disks: Option<Vec<String>>,
    pub log_files: Option<Vec<LogFile>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct LogFile {
    pub path: Option<String>,
    pub error_path: Option<String>,
    pub max_age_seconds: Option<u64>,
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
            monitored_disks: clone_or_default(
                &self.monitored_disks,
                vec![String::from("/"), String::from("/media/backup")],
            ),
            log_files: clone_or_default(
                &self.log_files,
                vec![
                    LogFile {
                        path: Some(String::from("/var/log/backup.log")),
                        error_path: Some(String::from("/var/log/backup.err.log")),
                        max_age_seconds: Some(60 * 60 * 24 * 2),
                    },
                    LogFile {
                        path: Some(String::from("/var/log/server.log")),
                        error_path: Some(String::from("/var/log/server.err.log")),
                        max_age_seconds: Some(60 * 60 * 24 * 2),
                    },
                ],
            ),
        }
    }
}
