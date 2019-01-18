use crate::cli;
use clap::crate_authors;
use man::prelude::*;

pub fn generate_man_page() {
    let cli::CliText {
        set,
        off,
        snooze,
        app,
        message,
    } = cli::CliText::new();
    let msg = Manual::new(app.name)
        .about(app.description)
        .flag(
            Flag::new()
                .short("-h")
                .long("--help")
                .help("Prints help information."),
        )
        .flag(
            Flag::new()
                .short("-V")
                .long("--version")
                .help("Prints version information."),
        )
        .option(
            Opt::new(set.value_name)
                .short(set.short)
                .long(set.long)
                .help(set.help),
        )
        .flag(
            Flag::new()
                .short(off.short)
                .long(off.long)
                .help(off.help)
        )
        .option(
            Opt::new(snooze.value_name)
                .short(snooze.short)
                .long(snooze.long)
                .help(snooze.help)
        )
        .option(
            Opt::new(message.value_name)
                .short(message.short)
                .long(message.long)
                .help(&format!("{} [default: {}]", message.help, message.default_value)),
        )
        .custom(
            Section::new("time syntaxes")
                .paragraph("dlarm supports two syntaxes for time setting absolute time: HH:MMpp (e.g., `12:34pm`) or HH:MM (e.g., `12:34`).")
                .paragraph("If you use the second syntax (without the am/pm), then dlarm will guess at the appropriate period.  Specifiably, dlarm will set an alarm for the earliest matching time that is both 1) today and 2) in the future. For example, if you set an alarm for `10:30`, then that will be interpreted as `10:30am` if the current time is before 10:30am; if it is currently after 10:30am, it will set an alarm for 10:30pm.  In either case, you will be informed of the time for which the alarm is set.")
                .paragraph("Additionally, dlarm supports setting a relative time.  To use this syntax, use a `+` prefix with your time and add a unit after your time.  Supported units include `min` or `m` for minutes and `hours` or `h` for hours.  Thus, `+25min` is a valid relative time.")
        )
        .author(Author::new(crate_authors!()))
        // TODO: add EXAMPLES section
        // TODO: add version info
        // TODO: update EXIT STATUS
        .render();
    println!("{}", msg);
}
