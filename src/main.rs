use clap::{crate_description, crate_version, App, Arg, Shell, SubCommand};
use commands::{Command, NewCommand};
use std::io;

mod commands;
mod dirs;
mod notes;

fn main() {
    let app = App::new("notes")
        .about(crate_description!())
        .version(crate_version!())
        .subcommands(vec![
            NewCommand::command(),
            SubCommand::with_name("completions")
                .about("Generate shells completions")
                .arg(
                    Arg::with_name("shell")
                        .possible_values(&["zsh", "bash", "fish", "powershell"])
                        .takes_value(true)
                        .required(true),
                ),
        ]);

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("new", Some(args)) => NewCommand::setup(args),
        ("completions", Some(args)) => {
            let shell: Shell;
            match args.value_of("shell") {
                Some("zsh") => shell = Shell::Zsh,
                Some("fish") => shell = Shell::Fish,
                Some("powershell") => shell = Shell::PowerShell,
                Some("bash") => shell = Shell::Bash,
                None | Some(_) => panic!("Cannot get shell completions"),
            }

            app.clone()
                .gen_completions_to("notes", shell, &mut io::stdout())
        }
        _ => {
            app.clone().print_help().expect("Cannot print help");
            println!();
        }
    }
}
