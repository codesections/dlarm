use clap::{crate_authors, crate_version, App, Arg, ArgGroup, SubCommand};
pub fn build_cli() -> App<'static, 'static> {
    let CliText { message, set, app } = CliText::new();
    App::new(app.name)
        .version(crate_version!())
        .author(crate_authors!())
        .about(app.description)
        .arg(
            Arg::with_name(set.long)
                .short(set.short)
                .long(set.long)
                .help(set.help)
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
            Arg::with_name(message.long)
                .short(message.short)
                .long(message.long)
                .help(message.help)
                .takes_value(true)
                .value_name("MSG")
                .default_value("ALARM ALARM ALARM"),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generates either a completion script for the specified shell or a man(1) page")
                .arg(
                    Arg::with_name("target")
                        .required(true)
                        .possible_values(&["bash", "fish", "zsh", "man"])
                        .help("The shell to generate the script for, or man to generate the man(1) page"),
                ),
        )
}

pub struct Argument {
    pub long: &'static str,
    pub short: &'static str,
    pub help: &'static str,
}
pub struct HeaderInfo {
    pub name: &'static str,
    pub description: &'static str,
}
pub struct CliText {
    pub app: HeaderInfo,
    pub message: Argument,
    pub set: Argument,
}

impl CliText {
    pub fn new() -> CliText {
        CliText {
            app: HeaderInfo {
                name: "dlarm",
                description: "Sets an alarm to be displayed by dwm",
            },
            message: Argument {
                long: "message",
                short: "m",
                help: "Sets the alarm message.",
            },
            set: Argument {
                long: "set",
                short: "s",
                help: "Sets a new alarm for <TIME>, e.g., 3:35pm",
            },
        }
    }
}
