mod cli;
mod data_file;
mod error_types;
mod man;
mod settings;
use crate::data_file::DataFile;
use crate::settings::AlarmSettings;
use dirs;
use std::io;

pub fn main() {
    let cli_arguments = cli::build_cli().get_matches();
    match cli_arguments.subcommand() {
        ("generate", Some(sub_matches)) => {
            let target = sub_matches.value_of("target").unwrap();
            if target == "man" {
                man::generate_man_page();
            } else {
                cli::build_cli().gen_completions_to(
                    "dlarm",
                    target.parse().unwrap(),
                    &mut io::stdout(),
                );
            }
        }
        (_, _) => (), // for brevity
    }
    let data_file = DataFile::new(r".dlarm.rc");
    let alarm_settings_string = data_file.read_to_string();
    let mut alarm_settings = AlarmSettings::from_string(&alarm_settings_string);

    alarm_settings.update_based_on(cli_arguments);
    data_file.write_to_data_file(&alarm_settings);
}
