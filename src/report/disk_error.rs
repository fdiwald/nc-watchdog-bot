use super::disk_info::DiskInfo;

pub (super) enum DiskError {
    DiskTooFull(DiskInfo),
    MountPointNotFound,
}