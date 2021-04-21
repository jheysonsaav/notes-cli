use crate::dirs::NotesDirs;
use clap::{App, Arg, ArgMatches, SubCommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use uuid::Uuid;

use super::Command;

pub struct NewCommand;

#[derive(Clone, Deserialize, Serialize)]
pub struct Note {
    id: Option<String>,
    title: Option<String>,
    body: Option<String>,
}

impl<'a, 'b> Command<'a, 'b> for NewCommand {
    fn command() -> App<'a, 'b> {
        SubCommand::with_name("new")
            .about("Create new note")
            .arg(
                Arg::with_name("title")
                    .long("title")
                    .short("t")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::with_name("body")
                    .long("body")
                    .short("b")
                    .takes_value(true)
                    .required(false),
            )
    }

    fn setup(args: &ArgMatches) {
        let mut note_title = String::new();
        let mut note_body = String::new();

        if args.value_of("title") == None {
            print!("Title: ");
            io::stdout().flush().unwrap();

            let mut buf = String::new();

            match io::stdin().read_line(&mut buf) {
                Ok(_) => note_title = buf.replace("\n", ""),
                Err(err) => eprintln!("Error: {:?}", err),
            }
        } else {
            note_title = args.value_of("title").unwrap().to_string();
        }

        if args.value_of("body") == None {
            print!("Body: ");
            io::stdout().flush().unwrap();

            let mut buf = String::new();

            match io::stdin().read_line(&mut buf) {
                Ok(_) => note_body = buf.replace("\n", ""),
                Err(err) => eprintln!("Error: {:?}", err),
            }
        } else {
            note_body = args.value_of("body").unwrap().to_string();
        }

        let note = Note {
            id: Some(Uuid::new_v4().to_string()),
            title: Some(note_title),
            body: Some(note_body),
        };

        #[cfg(unix)]
        let note_file = format!(
            "{}/notes.yml",
            NotesDirs::load().data_dir().to_str().unwrap()
        );

        #[cfg(windows)]
        let note_file = format!(
            "{}/notes.yml",
            NotesDirs::load().data_dir().to_str().unwrap()
        );

        if !Path::new(note_file.as_str()).exists() {
            File::create(&note_file).expect("Cannot create notes file");

            let default_content: Vec<Note> = vec![Note {
                id: Some(Uuid::new_v4().to_string()),
                title: Some(String::from("DEFAULT")),
                body: Some(String::from("DEFAULT")),
            }];

            fs::write(
                &note_file,
                serde_yaml::to_string(&default_content).unwrap(),
            )
            .expect("cannot write DEFAULT item in note file");
        }

        let notes = fs::read_to_string(&note_file).unwrap();

        let notes_yaml: Vec<Note> =
            serde_yaml::from_str(notes.as_str()).unwrap();

        let mut all_notes: Vec<Note> = vec![];

        for i in notes_yaml {
            if i.title != Some(String::from("DEFAULT"))
                && i.body != Some(String::from("DEFAULT"))
            {
                all_notes.push(i);
            }
        }

        all_notes.push(note);

        fs::write(&note_file, format!("# This file is generated automatically by Notes, please do not modify\n{}", serde_yaml::to_string(&all_notes).unwrap()))
            .expect("Cannot write notes");
    }
}
