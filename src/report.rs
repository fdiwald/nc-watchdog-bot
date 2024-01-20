mod diskinfo;
mod logfile_error;
use self::{diskinfo::DiskInfo, logfile_error::LogFileError};
use crate::{
    api::hypertext_element::{compile_hypertext_elements, Hypertext},
    config::{LogFile, WatchdogConfig},
    WatchdogError,
};
use frankenstein::MessageEntity;
use std::{
    ffi::OsString,
    fs::File,
    path::Path,
    time::{Duration, SystemTime},
};
type HT = Hypertext;

pub(crate) struct Report {
    pub disk_info: Vec<DiskInfo>,
    pub log_files_status: Vec<(String, Result<(), LogFileError>)>,
}

impl Report {
    pub(crate) fn new(config: &WatchdogConfig) -> Result<Report, WatchdogError> {
        Ok(Report {
            log_files_status: get_log_files_status(config),
            disk_info: get_disk_infos(config)?,
        })
    }

    pub(crate) fn create_message(&self) -> Result<(String, Vec<MessageEntity>), WatchdogError> {
        let mut hypertexts = vec![];
        hypertexts.append(self.create_disk_usage_message()?.as_mut());
        hypertexts.append(self.create_log_files_message()?.as_mut());

        Ok(compile_hypertext_elements(hypertexts))
    }

    fn create_disk_usage_message(&self) -> Result<Vec<Hypertext>, WatchdogError> {
        let mut result = vec![HT::bold("Disks\n")];
        result.append(
            self.disk_info
                .iter()
                .map(|disk_info| {
                    let icon = if disk_info.available_space_gb < 10 {
                        "🔴"
                    } else {
                        "🟢"
                    };
                    HT::text(format!(
                        "{icon} {0} ({1}) {2}GB free\n",
                        disk_info.name.to_str().expect(&format!(
                            "Could not convert disk name {0:?} to a string.",
                            disk_info.name
                        )),
                        disk_info.mount_point.display(),
                        disk_info.available_space_gb
                    ))
                })
                .collect::<Vec<_>>()
                .as_mut(),
        );
        Ok(result)
    }

    fn create_log_files_message(&self) -> Result<Vec<Hypertext>, WatchdogError> {
        let mut result = vec![HT::bold("Logs\n")];
        result.append(
            self.log_files_status
                .iter()
                .map(|(file, status)| match status {
                    Ok(_) => vec![format!("🟢 {file}\n").into()],
                    Err(LogFileError::ErrorsFound { .. }) => {
                        vec!["🔴 Found errors in ".into(), HT::bold(format!("{file}\n"))]
                    }
                    Err(LogFileError::FileAgeExceeded { modified_time }) => {
                        vec![format!(
                            "🔴 Age of {file} ({}) exceeds the limit.\n",
                            format_duration_since(*modified_time)
                        )
                        .into()]
                    }
                    Err(error) => vec![format!("🔴 {:#?}\n", error).into()],
                })
                .flatten()
                .collect::<Vec<_>>()
                .as_mut(),
        );
        Ok(result)
    }
}

fn format_duration_since(time: SystemTime) -> String {
    match SystemTime::now().duration_since(time) {
        Ok(duration) => {
            let days = duration.as_secs() / (60 * 60 * 24);
            if days != 0 {
                return format!("{days} days");
            }

            let hours = duration.as_secs() / (60 * 60);
            if hours != 0 {
                return format!("{hours} hours");
            }

            let minutes = duration.as_secs() / (60);
            return format!("{minutes} minutes");
        }
        Err(error) => format!("{error}"),
    }
}

fn get_log_files_status(config: &WatchdogConfig) -> Vec<(String, Result<(), LogFileError>)> {
    if let Some(log_files) = &config.log_files {
        log_files
            .iter()
            .map(|log_file| {
                if let Some(log_file_path) = log_file.path.clone() {
                    (
                        log_file_path,
                        if let Err(error) = check_error_file(&log_file.error_path) {
                            Err(error)
                        } else {
                            check_log_file(&log_file)
                        },
                    )
                } else {
                    (
                        String::from("<No path defined>"),
                        Err(LogFileError::NoLogFileDefined),
                    )
                }
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    }
}

fn check_log_file(log_file: &LogFile) -> Result<(), LogFileError> {
    if let Some(log_file_path) = &log_file.path {
        if !Path::new(log_file_path).exists() {
            Err(LogFileError::FileNotFound)
        } else {
            {
                let modified_time = File::open(log_file_path)?.metadata()?.modified()?;
                let max_age =
                    Duration::from_secs(log_file.max_age_seconds.unwrap_or(60 * 60 * 24u64));
                if SystemTime::now().duration_since(modified_time)? > max_age {
                    Err(LogFileError::FileAgeExceeded { modified_time })
                } else {
                    Ok(())
                }
            }
        }
    } else {
        Ok(())
    }
}

fn check_error_file(error_path: &Option<String>) -> Result<(), LogFileError> {
    if let Some(error_path) = error_path {
        // TODO Geht das in der Zeile oben mit?
        if Path::new(&error_path).exists() {
            return if File::open(error_path)?.metadata()?.len() > 0 {
                Err(LogFileError::ErrorsFound)
            } else {
                Ok(())
            };
        };
    };
    return Ok(());
}

fn get_disk_infos(config: &WatchdogConfig) -> Result<Vec<DiskInfo>, WatchdogError> {
    let mut disk_infos = sysinfo::Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| DiskInfo::from(disk))
        .collect::<Vec<_>>();

    if let Some(monitor_disks) = config.monitored_disks.as_ref() {
        let monitor_disks = monitor_disks
            .iter()
            .map(|monitor_disk| OsString::from(monitor_disk))
            .collect::<Vec<_>>();

        disk_infos.retain(|disk| {
            monitor_disks
                .iter()
                .any(|monitor_disk| monitor_disk == &disk.mount_point)
        });
    }

    let result = disk_infos;

    Ok(result)
}
