use clap::{crate_authors, crate_version, App, Arg, ArgGroup, SubCommand};
pub fn build_cli() -> App<'static, 'static> {
    let CliText {
        message,
        set,
        off,
        app,
        snooze,
    } = CliText::new();
    App::new(app.name)
        .version(crate_version!())
        .author(crate_authors!())
        .about(app.description)
        .arg(
            Arg::with_name(set.name)
                .short(set.short)
                .long(set.long)
                .help(set.help)
                .takes_value(true)
                .value_name(set.value_name),
        )
        .arg(
            Arg::with_name(off.name)
                .short(off.short)
                .long(off.long)
                .help(off.help),
        )
        .arg(
            Arg::with_name(snooze.name)
                .short(snooze.short)
                .long(snooze.long)
                .help(snooze.help)
                .takes_value(true)
                .value_name(snooze.value_name)
                .hide_possible_values(true)
                .min_values(0)
                .max_values(1),
        )
        .group(ArgGroup::with_name("setting the alarm").args(&["set", "off", "snooze"]))
        .arg(
            Arg::with_name(message.name)
                .short(message.short)
                .long(message.long)
                .help(message.help)
                .takes_value(true)
                .value_name(message.value_name)
                .default_value(message.default_value),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generates either a completion script for the specified shell or a man(1) page")
                .arg(
                    Arg::with_name("target")
                        .required(true)
                        .possible_values(&["bash", "fish", "zsh", "elvish", "man"])
                        .help("The shell to generate the script for, or man to generate the man(1) page"),
                ),
        )
}

pub struct ArgValues {
    pub name: &'static str,
    pub long: &'static str,
    pub short: &'static str,
    pub help: &'static str,
}
pub struct OptValues {
    pub name: &'static str,
    pub long: &'static str,
    pub short: &'static str,
    pub help: &'static str,
    pub value_name: &'static str,
    pub default_value: &'static str,
}
pub struct HeaderInfo {
    pub name: &'static str,
    pub description: &'static str,
}
pub struct CliText {
    pub app: HeaderInfo,
    pub set: OptValues,
    pub off: ArgValues,
    pub snooze: OptValues,
    pub message: OptValues,
}

impl CliText {
    pub fn new() -> CliText {
        CliText {
            app: HeaderInfo {
                name: "dlarm",
                description: "Sets an alarm to be displayed by dwm",
            },
            set: OptValues {
                name: "set",
                long: "--set",
                short: "-s",
                help: "Sets a new alarm for <TIME> (like 3:35pm or +25min)",
                value_name: "TIME",
                default_value: "",
            },
            off: ArgValues {
                name: "off",
                long: "--off",
                short: "-o",
                help: "Turns off the currently active alarm",
            },
            snooze: OptValues {
                name: "snooze",
                long: "--snooze",
                short: "-z",
                help: "Snoozes the alarm for <MIN> minutes [default: 5]",
                value_name: "MIN",
                default_value: "",
            },
            message: OptValues {
                name: "message",
                long: "--message",
                short: "-m",
                help: "Sets the alarm message.",
                value_name: "MSG",
                default_value: "ALARM ALARM ALARM",
            },
        }
    }
}
