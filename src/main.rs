use clap::{crate_description, crate_version, App};
use commands::{Command, NewCommand};

mod commands;
mod dirs;
mod notes;

fn main() {
    let app = App::new("notes")
        .about(crate_description!())
        .version(crate_version!())
        .subcommands(vec![NewCommand::command()]);

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("new", Some(args)) => NewCommand::setup(args),
        _ => {
            app.clone().print_help().expect("Cannot print help");
            println!();
        }
    }
}
