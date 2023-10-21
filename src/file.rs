use std::fs;
use serde::Deserialize;
use directories::ProjectDirs;

//struct to store config
#[derive(Deserialize)]
pub struct Config {
    pub directories: Directories,
    pub executables: Executables,
}

#[derive(Deserialize)]
pub struct Directories {
    pub titles: Vec<String>,
    pub paths: Vec<String>,
}

#[derive(Deserialize)]
pub struct Executables {
    pub titles: Vec<String>,
    pub commands: Vec<String>,
}

pub fn get_config() -> Config {
    let proj_dirs = ProjectDirs::from("", "tmux-greeter", "tmux-greeter").unwrap();
    let config_dir = proj_dirs.config_dir();
    let config_file = match fs::read_to_string(config_dir.join("config.toml")) {
        Ok(result) => result,
        Err(_) => String::from("
            [directories]
            titles = []
            paths = []

            [executables]
            titles = []
            commands = []
            ")
    };

    //convert to string to struct
    let mut config: Config = match toml::from_str(&config_file) {
        Ok(result) => result,
        Err(_) => Config {
            directories: Directories {
                titles: vec![], paths: vec![], },
            executables: Executables {
                titles: vec![], commands: vec![], }
        },
    };

    if config.executables.titles.len() != config.executables.commands.len() {
        config = Config {
            directories: Directories {
                titles: vec![], paths: vec![], },
            executables: Executables {
                titles: vec![], commands: vec![], }
        };

    }

    if config.directories.titles.len() != config.directories.paths.len() {
        config = Config {
            directories: Directories {
                titles: vec![], paths: vec![], },
            executables: Executables {
                titles: vec![], commands: vec![], }
        };
    }

    config
}

pub fn get_theme() -> String {
    let proj_dirs = ProjectDirs::from("", "tmux-greeter", "tmux-greeter").unwrap();
    let config_dir = proj_dirs.config_dir();

    let theme_file = match fs::read_to_string(config_dir.join("theme.toml")) {
        Ok(result) => result,
        Err(_) => String::new(),
    };

    theme_file
}

