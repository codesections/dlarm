use crate::cli;
use clap::crate_authors;
use man::prelude::*;

pub fn generate_man_page() {
    let cli::CliText { message, set, app } = cli::CliText::new();
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
            Opt::new(message.long)
                .short(&format!("-{}", message.short))
                .long(&format!("--{}", message.long))
                .help(message.help),
        )
        .option(
            Opt::new(set.long)
                .short(&format!("-{}", set.short))
                .long(&format!("--{}", set.long))
                .help(set.help),
        )
        .custom(
            Sec::new("time syntax")
                .paragraph("dlarm supports two syntaxes for time: HH:MMpp (e.g., `12:34pm`) or HH:MM (e.g., `12:34`).")
                .paragraph("If you use the second syntax (without the am/pm), then dlarm will guess at the appropriate period.  Specifiably, dlarm will set an alarm for the earliest matching time that is both 1) today and 2) in the future. For example, if you set an alarm for `10:30`, then that will be interpreted as `10:30am` if the current time is before 10:30am; if it is currently after 10:30am, it will set an alarm for 10:30pm.  In either case, you will be informed of the time for which the alarm is set.")
        )
        .author(Author::new(crate_authors!()))
        .render();
    println!("{}", msg);
}
