use clap::{crate_authors, crate_version, App, Arg, ArgGroup, ArgMatches};
pub fn get_cli_args() -> ArgMatches<'static> {
    App::new("dlarm")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Sets an alarm for dmenu.")
        .arg(
            Arg::with_name("set")
                .short("s")
                .long("set")
                .help("Sets a new alarm for <TIME>, e.g., 3:35pm")
                .takes_value(true)
                .value_name("TIME"),
        )
        .arg(
            Arg::with_name("off")
                .short("o")
                .long("off")
                .help("Turns off the currently active alarm"),
        )
        .arg(
            Arg::with_name("snooze")
                .short("z")
                .long("snooze")
                .help("Snoozes the alarm for <MIN> minutes [default: 5]")
                .takes_value(true)
                .value_name("MIN")
                .hide_possible_values(true)
                .min_values(0)
                .max_values(1),
        )
        .group(ArgGroup::with_name("setting the alarm").args(&["set", "off", "snooze"]))
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .help("Sets the alarm message.")
                .takes_value(true)
                .value_name("MSG")
                .default_value("ALARM ALARM ALARM"),
        )
        .get_matches()
}
