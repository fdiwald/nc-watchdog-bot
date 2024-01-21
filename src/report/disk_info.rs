use std::path::PathBuf;
use sysinfo::Disk;

pub struct DiskInfo {
    pub name: String,
    pub mount_point: PathBuf,
    pub total_space_gb: u64,
    pub free_space_gb: u64,
}

impl From<&Disk> for DiskInfo {
    fn from(value: &Disk) -> Self {
        DiskInfo {
            name: String::from(value.name().to_str().unwrap_or_else(|| "")),
            mount_point: value.mount_point().to_owned(),
            total_space_gb: value.total_space() / 1024 / 1024 / 1024,
            free_space_gb: value.available_space() / 1024 / 1024 / 1024,
        }
    }
}
