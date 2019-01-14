//! dlarm is the alarm system for [dwm](https://dwm.suckless.org/).  
//!
//! Like dwm, dlarm is minimalist, keyboard-focused, and terminal centric.  All dlarm does is to allow you to set an alarm for a specified time later today.  When that alarm goes off, your dwm status bar will flash with either a string you specify or the text `ALARM ALARM ALARM`.  You can then turn off or snooze the alarm.
//!
//! # Usage Example
//!
//! ```bash
//! $ dlarm --set "3:30pm" --message "Meeting"
//! Alarm set for 03:30pm
//!
//! $ dlarm --off
//!
//! $ dlarm -s "2:20"
//! Alarm set for 02:20pm
//!
//! $ dlarm -z
//! Alarm snoozed for 5 minutes.
//! Zzz…
//! Alarm now set for 02:25pm
//! ```

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
