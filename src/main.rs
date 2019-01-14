mod clap_config;
mod data_file;
mod error_types;
mod settings;
use crate::data_file::DataFile;
use crate::settings::AlarmSettings;
use dirs;

pub fn main() {
    let cli_arguments = clap_config::get_cli_args();

    let data_file = DataFile::new(r".dlarm.rc");
    let alarm_settings_string = data_file.read_to_string();
    let mut alarm_settings = AlarmSettings::from_string(&alarm_settings_string);

    alarm_settings.update_based_on(cli_arguments);
    data_file.write_to_data_file(&alarm_settings);
}
