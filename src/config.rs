use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WatchdogConfig {
    pub api_token: String,
    pub chat_id: Option<String>,
    pub user_id: Option<String>,
    pub log_path: Option<PathBuf>,
}
