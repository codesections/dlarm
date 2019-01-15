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

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use colored::*;
    use std::process::Command;
    #[test]
    fn it_turns_off_the_alarm() {
        Command::main_binary().unwrap().arg("-o").assert().success();
        Command::main_binary()
            .unwrap()
            .arg("--off")
            .assert()
            .success();
    }

    #[test]
    fn it_sets_the_alarm() {
        Command::main_binary()
            .unwrap()
            .arg("-s 3:30pm")
            .assert()
            .success()
            .stdout(format!("Alarm set for {}\n", "03:30pm".blue()));
        Command::main_binary()
            .unwrap()
            .arg("--set=3:30pm")
            .assert()
            .success()
            .stdout(format!("Alarm set for {}\n", "03:30pm".blue()));
        Command::main_binary()
            .unwrap()
            .arg("--set")
            .arg("3:30pm")
            .assert()
            .success()
            .stdout(format!("Alarm set for {}\n", "03:30pm".blue()));
    }

    #[test]
    fn it_guesses_the_period_when_setting_the_alarm() {
        use chrono::prelude::*;
        use chrono::Local;
        Local::now().hour();
        if Local::now().hour() < 9 {
            Command::main_binary()
                .unwrap()
                .arg("-s 9:00")
                .assert()
                .success()
                .stdout(format!("Alarm set for {}\n", "09:00am".blue()));
            Command::main_binary()
                .unwrap()
                .arg("--set=9:00")
                .assert()
                .success()
                .stdout(format!("Alarm set for {}\n", "09:00am".blue()));
            Command::main_binary()
                .unwrap()
                .arg("--set")
                .arg("9:00")
                .assert()
                .success()
                .stdout(format!("Alarm set for {}\n", "09:00am".blue()));
        } else {
            Command::main_binary()
                .unwrap()
                .arg("-s 9:00")
                .assert()
                .success()
                .stdout(format!("Alarm set for {}\n", "09:00pm".blue()));
            Command::main_binary()
                .unwrap()
                .arg("--set=9:00")
                .assert()
                .success()
                .stdout(format!("Alarm set for {}\n", "09:00pm".blue()));
            Command::main_binary()
                .unwrap()
                .arg("--set")
                .arg("9:00")
                .assert()
                .success()
                .stdout(format!("Alarm set for {}\n", "09:00pm".blue()));
        }
    }

    #[test]
    fn it_should_snooze_the_alarm() {
        use predicates::prelude::*;
        Command::main_binary()
            .unwrap()
            .arg("-z")
            .assert()
            .success()
            .stdout(predicate::str::starts_with("Alarm snoozed for 5 minutes."));
        Command::main_binary()
            .unwrap()
            .arg("--snooze")
            .assert()
            .success()
            .stdout(predicate::str::starts_with("Alarm snoozed for 5 minutes."));
    }

    #[test]
    fn it_should_snooze_the_alarm_for_custom_time() {
        use predicates::prelude::*;
        Command::main_binary()
            .unwrap()
            .arg("-z")
            .arg("12")
            .assert()
            .success()
            .stdout(predicate::str::starts_with("Alarm snoozed for 12 minutes."));
        Command::main_binary()
            .unwrap()
            .arg("--snooze=99")
            .assert()
            .success()
            .stdout(predicate::str::starts_with("Alarm snoozed for 99 minutes."));
        Command::main_binary()
            .unwrap()
            .arg("--snooze")
            .arg("23")
            .assert()
            .success()
            .stdout(predicate::str::starts_with("Alarm snoozed for 23 minutes."));
    }

}
