mod diskinfo;
use time::OffsetDateTime;

use self::diskinfo::DiskInfo;
use crate::{config::WatchdogConfig, WatchdogError};
use std::{
    ffi::{OsStr, OsString},
    fmt::Debug,
    ops::Add,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub(crate) struct Report {
    pub disk_info: Vec<DiskInfo>,
    // pub file_info: Vec<fs::Metadata>,
    pub last_backup: SystemTime,
}

impl Report {
    pub(crate) fn new(config: &WatchdogConfig) -> Result<Report, WatchdogError> {
        let log_path_str = config
            .log_path
            .as_ref()
            .ok_or(WatchdogError::NoLogPathConfigured(String::from(
                "Report::new",
            )))?;
        let mysqldump_path = PathBuf::from(log_path_str).join("mysqldump.sql");
        println!("calculating last_backup...");
        let last_backup = mysqldump_path.metadata()?.created()?;
        println!("last_backup: {last_backup:#?}");

        let monitor_disks = config
            .monitor_disks
            .as_ref()
            .ok_or_else(|| WatchdogError::NoMonitorDisksConfigured(String::from("Report::new")))?
            .iter()
            .map(|monitor_disk| OsString::from(monitor_disk))
            .collect::<Vec<_>>();
        let disk_info = sysinfo::Disks::new_with_refreshed_list();
        println!("disk_info: {disk_info:#?}");
        let disk_info = disk_info
            .iter()
            .filter(|disk| {
                monitor_disks
                    .iter()
                    .any(|monitor_disk| monitor_disk == disk.name())
            })
            .map(|disk| DiskInfo::from(disk))
            .collect();

        println!("Report created.");
        Ok(Report {
            last_backup,
            disk_info,
            //            file_info: todo!(),
        })
    }

    pub(crate) fn create_message(&self) -> String {
        //todo!("Build a nice message out of the gathered information");
        // let formatter = timeago::Formatter::new();
        let backup_age = OffsetDateTime::from(self.last_backup) - OffsetDateTime::now_utc();

        let message = format!("Last backup ran {0} ago.", backup_age);
        message
    }
}
