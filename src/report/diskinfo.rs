use std::ffi::OsString;

use sysinfo::Disk;

pub struct DiskInfo{
    pub name: OsString,
    pub total_space: u64,
    pub available_space: u64,
}

impl From<&Disk> for DiskInfo  {
    fn from(value: &Disk) -> Self {
        DiskInfo {
            name: value.name().to_owned(),
            total_space: value.total_space(),
            available_space: value.available_space(),
        }
    }
}