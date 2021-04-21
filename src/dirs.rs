use directories::ProjectDirs;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub struct NotesDirs {
    config_dir: PathBuf,
    data_dir: PathBuf,
}

impl NotesDirs {
    pub fn load() -> Self {
        if let Some(dirs) = ProjectDirs::from("com", "jheysonsaav", "notes") {
            if !dirs.config_dir().exists() {
                create_dir_all(dirs.config_dir()).expect("Cannot create config directory");
            }

            if !dirs.data_dir().exists() {
                create_dir_all(dirs.data_dir()).expect("Cannot create data directory");
            }

            Self {
                config_dir: dirs.config_dir().to_owned(),
                data_dir: dirs.data_dir().to_owned(),
            }
        } else {
            Self {
                config_dir: PathBuf::new(),
                data_dir: PathBuf::new(),
            }
        }
    }

    pub fn config_dir(&self) -> &Path {
        self.config_dir.as_path()
    }

    pub fn data_dir(&self) -> &Path {
        self.data_dir.as_path()
    }
}
