use crate::error_types::{error, Error::*};
use crate::settings::AlarmSettings;
use std::{fs, process};
pub struct DataFile {
    file_path: std::path::PathBuf,
}

impl DataFile {
    pub fn new(filename: &str) -> DataFile {
        let mut file_path = dirs::home_dir().unwrap();
        &file_path.push(filename);
        DataFile { file_path }
    }
    pub fn read_to_string(&self) -> String {
        fs::read_to_string(&self.file_path).unwrap_or(String::new())
    }

    pub fn write_to_data_file(&self, alarm_settings: &AlarmSettings) {
        fs::write(
            &self.file_path,
            format!(
                "alarm-time={}\nalarm-message={}",
                alarm_settings.time, alarm_settings.message
            ),
        )
        .unwrap_or_else(|_| {
            eprintln!("{}", error(OpeningDataFile));
            process::exit(3);
        });
    }
}
