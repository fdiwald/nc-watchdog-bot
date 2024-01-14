use std::{path::PathBuf, ffi::OsString};
use sysinfo::Disk;

pub struct DiskInfo {
    pub name: OsString,
    pub mount_point: PathBuf,
    pub total_space_gb: u64,
    pub available_space_gb: u64,
}

impl From<&Disk> for DiskInfo {
    fn from(value: &Disk) -> Self {
        DiskInfo {
            name: OsString::from(value.name()),
            mount_point: value.mount_point().to_owned(),
            total_space_gb: value.total_space() / 1024 / 1024 / 1024,
            available_space_gb: value.available_space() / 1024 / 1024 / 1024,
        }
    }
}
