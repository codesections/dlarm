use crate::error_types::{error, Error::*};
use chrono::prelude::*;
use chrono::{Duration, Local, NaiveTime};
use clap::value_t;
use colored::*;
use std::process;

pub struct AlarmSettings {
    pub time: String,
    pub message: String,
}

impl AlarmSettings {
    pub fn from_string(file_contents: &String) -> AlarmSettings {
        let mut time = String::from("100");
        let mut message = String::from("default msg");
        for line in file_contents.lines().collect::<Vec<&str>>() {
            let (key, value) = (
                line.split('=').collect::<Vec<&str>>()[0],
                line.split('=').collect::<Vec<&str>>()[1],
            );
            match key {
                "alarm-time" => time = String::from(value),
                "alarm-message" => message = String::from(value),
                _ => {
                    eprintln!("{}", error(DataFile));
                    process::exit(2);
                }
            }
        }
        AlarmSettings { time, message }
    }

    pub fn update_based_on(&mut self, cli_arguments: clap::ArgMatches) {
        if let Some(message) = cli_arguments.value_of("message") {
            self.message = String::from(message);
        }

        if cli_arguments.is_present("off") {
            self.time = String::from("99999999999");
        } else if cli_arguments.is_present("snooze") {
            self.snooze(&cli_arguments);
        } else if let Some(usr_input) = cli_arguments.value_of("set") {
            self.set_alarm_time(usr_input);
        }
    }

    fn set_alarm_time(&mut self, usr_input: &str) {
        let time;
        if let Ok(time_with_am_pm) = NaiveTime::parse_from_str(usr_input, "%I:%M%p") {
            time = time_with_am_pm;
        } else if usr_input.starts_with("+") {
            time = parse_relative_time(&usr_input);
        } else {
            time = guess_at_am_or_pm_period(&usr_input);
        };
        let local = Local::today().and_time(time).unwrap();
        let (timestamp, human_time) = (local.timestamp(), local.format("%I:%M%P"));
        self.time = timestamp.to_string();
        println!("Alarm set for {}", human_time.to_string().blue());
    }

    fn snooze(&mut self, cli_arguments: &clap::ArgMatches) {
        if self.time == "99999999999" {
            eprintln!("{}", error(SnoozeWhenAlarmUnset));
            process::exit(5);
        }
        let snooze_duration = if cli_arguments.value_of("snooze").is_none() {
            5
        } else {
            value_t!(cli_arguments, "snooze", i64).unwrap_or_else(|_| {
                eprintln!("{}", error(InvalidSnoozeTime));
                process::exit(4);
            })
        };
        let time = &self.time;
        let time = NaiveDateTime::from_timestamp(time.parse().unwrap(), 0);
        let new_time = time + Duration::minutes(snooze_duration);
        let timestamp = new_time.timestamp();
        self.time = timestamp.to_string();

        let timezone_offset_in_hours = Local::today().offset().fix().utc_minus_local() / (60 * 60);
        let local = new_time - Duration::hours(timezone_offset_in_hours as i64);
        let human_time = local.format("%I:%M%P");
        println!(
            "Alarm snoozed for {} minutes.\nZzz… {}\nAlarm now set for {}",
            snooze_duration,
            "Zzz…".dimmed(),
            human_time.to_string().blue()
        );
    }
}

fn guess_at_am_or_pm_period(usr_input: &str) -> NaiveTime {
    let mut hours: String = usr_input.chars().filter(|c| c.is_numeric()).collect();
    if hours.len() < 3 {
        eprintln!("{} {}", usr_input, error(InvalidTime));
        process::exit(1);
    }
    hours.truncate(hours.len() - 2);
    let (input_hour, current_hour): (u32, _) = (hours.parse().unwrap(), Local::now().hour());
    let am_pm = if input_hour >= current_hour && input_hour < 12 {
        "am"
    } else {
        "pm"
    };
    let mut modified_usr_input = String::from(usr_input);
    modified_usr_input.push_str(am_pm);
    let modified_usr_input = &modified_usr_input[..];
    if let Ok(time) = NaiveTime::parse_from_str(modified_usr_input, "%I:%M%p") {
        time
    } else {
        eprintln!("{} {}", usr_input.red().bold(), error(InvalidTime));
        process::exit(1);
    }
}

fn parse_relative_time(usr_input: &str) -> NaiveTime {
    use std::time::Duration;
    let time_string = String::from(usr_input).split_off(1);
    let duration: Duration = time_string
        .parse::<humantime::Duration>()
        .unwrap_or_else(|_| {
            eprintln!("{} {}", usr_input.red().bold(), error(InvalidTime));
            process::exit(1);
        })
        .into();

    let time = Local::now() + chrono::Duration::seconds(duration.as_secs() as i64);

    NaiveTime::from_hms(time.hour(), time.minute(), time.second())
}
