mod diskinfo;
use frankenstein::MessageEntity;

use self::diskinfo::DiskInfo;
use crate::{config::WatchdogConfig, WatchdogError, api::hypertext_element::{compile_hypertext_elements, Hypertext}};
use std::{
    ffi::OsString,
    path::PathBuf,
    time::{Duration, SystemTime},
};
type HT = Hypertext;

pub(crate) struct Report {
    pub disk_info: Vec<DiskInfo>,
    // pub file_info: Vec<fs::Metadata>,
    pub last_backup: SystemTime,
}

impl Report {
    pub(crate) fn new(config: &WatchdogConfig) -> Result<Report, WatchdogError> {
        Ok(Report {
            last_backup: get_last_backup_time(config)?,
            disk_info: get_disk_infos(config)?,
            //            file_info: todo!(),
        })
    }

    pub(crate) fn create_message(&self) -> Result<(String, Vec<MessageEntity>), WatchdogError> {
        let mut hypertexts = vec![];
        hypertexts.append(self.create_backup_age_messages()?.as_mut());
        hypertexts.append(self.create_disk_usage_message()?.as_mut());
        
        Ok(compile_hypertext_elements(hypertexts))
    }

    fn create_backup_age_messages(&self) -> Result<Vec<Hypertext>, WatchdogError> {
        let header = HT::bold("Backup\n");
        let backup_age = SystemTime::now().duration_since(self.last_backup)?;
        let icon = if backup_age > Duration::from_secs(60 * 60 * 24 * 2) {
            "🔴"
        } else {
            "🟢"
        };
        let message = HT::text(&format!("{icon} Last backup ran {0}.\n", format_duration(backup_age)));
        Ok(vec![header, message])
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
                        disk_info.name.to_str().expect(&format!("Could not convert disk name {0:?} to a string.", disk_info.name)),
                        disk_info.mount_point.display(),
                        disk_info.available_space_gb
                    ))
                })
                .collect::<Vec<_>>()
                .as_mut(),
        );
        Ok(result)
    }
}

fn format_duration(duration: Duration) -> String {
    let days = duration.as_secs() / (60 * 60 * 24);
    if days != 0 {
        return format!("{days} days ago");
    }

    let hours = duration.as_secs() / (60 * 60);
    if hours != 0 {
        return format!("{hours} hours ago");
    }

    let minutes = duration.as_secs() / (60);
    if minutes != 0 {
        return format!("{minutes} minutes ago");
    }

    String::from("just now")
}

fn get_last_backup_time(config: &WatchdogConfig) -> Result<SystemTime, WatchdogError> {
    let log_path_str = config
        .log_path
        .as_ref()
        .ok_or(WatchdogError::NoLogPathConfigured(String::from(
            "Report::new",
        )))?;
    let mysqldump_path = PathBuf::from(log_path_str).join("mysqldump.sql");
    println!("calculating last_backup from {:?}...", mysqldump_path);
    let last_backup_time = mysqldump_path.metadata()?.modified();
    println!("last_backup: {last_backup_time:#?}");
    Ok(last_backup_time)
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
