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
        .author(Author::new(crate_authors!()))
        .render();
    println!("{}", msg);
}
