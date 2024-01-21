use std::time::SystemTime;

#[derive(Debug)]
pub(crate) enum LogFileError {
    NoLogFilesDefined,
    FileNotFound,
    ErrorsFound,
    FileAgeExceeded { modified_time: SystemTime },
    IoError(std::io::Error),
    SystemTimeError(std::time::SystemTimeError),
}

impl From<std::io::Error> for LogFileError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<std::time::SystemTimeError> for LogFileError {
    fn from(value: std::time::SystemTimeError) -> Self {
        Self::SystemTimeError(value)
    }
}
