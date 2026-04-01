// use egui::TextureHandle;

// use crate::libs::imagen::load_image;
// use crate::libs::dialogos;
// use crate::utils::game::GameConfig;
use crate::utils::config::{RunnerOption};
use std::process::Command;
use std::{fs, path::Path};


pub fn read_file(path: &str) -> String {
    return fs::read_to_string(path).unwrap();
}

pub fn process_options(options: &Vec<RunnerOption>) -> (String, String, String) {
    let mut cmd_prefix = String::new();
    let mut cmd_command = String::new();
    let mut cmd_args = String::new();

    for option in options {
        let mut option_cmd = option.cmd.clone();
        if option.mode == "input" || option.mode == "folder" || option.mode == "file_names" {
            option_cmd = option_cmd.replace("$1", option.input.as_str());
            if option.input.contains(" ") {
                option_cmd = option_cmd.replace("$1", format!("\"{}\"", option.input).as_str());
            }
        }
        if option.tipo == "prefix" {
            cmd_prefix += format!("{option_cmd} ").as_str();
        } else if option.tipo == "command" {
            cmd_command += format!("{option_cmd} ").as_str();
        } else if option.tipo == "arg" {
            cmd_args += format!("{option_cmd} ").as_str();
        }
    }

    (cmd_prefix, cmd_command, cmd_args)
}


pub fn create_file_all(path: &str, data: &str) {
    if !Path::new(&path).exists() {
        fs::write(&path, data).unwrap();
    }
}


pub fn notify(text: &str) {
    println!("{text}");
    Command::new("notify-send")
        .args(["-i", "cat_games_launcher", "-a", "Cat games Launcher", text])
        .spawn()
        .ok();
}