use crate::libs::imagen::load_image;
use crate::libs::utils::{create_file_all, notify, process_options};
use crate::libs::vars::Variables;
use crate::libs::{dialogos, vars};
use crate::utils::config::{Config, RunnerOption, RunnerVarOption};
use egui::TextureHandle;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::{collections::HashMap, env};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameConfig {
    pub name: String,
    #[serde(default)]
    pub icon: String,
    pub bin: String,
    pub cwd: String,
    pub args: String,
    pub prefix: String,
    pub global: Vec<RunnerOption>,
    pub runner_name: String,
    pub options: Vec<RunnerOption>,
    pub vars: Vec<RunnerVarOption>,
    pub command_base: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installer: Option<String>,
}

impl GameConfig {
    pub fn shortcut(&self, id: &String) -> std::string::String {
        let mut icon = "cat_games_launcher".to_string();
        if !self.icon.is_empty() {
            icon = self.icon.clone();
        }
        let data = format!(
            "[Desktop Entry]
Name={}
Comment=Juego {} Cat Games Launcher
Exec={} run {}
Icon={}
Terminal=false
Type=Application
Categories=Application;Game;
StartupNotify=true
Path={}",
            self.name,
            self.name,
            env::current_exe().unwrap().to_string_lossy().to_string(),
            id,
            icon,
            self.cwd
        )
        .to_string();
        return data;
    }

    pub fn new(
        config: &Config,
        runner_name: &String,
        name: &String,
        add_bin: &String,
        cwd: &String,
        args: &String,
        prefix: &String,
        icon: &String,
    ) -> GameConfig {
        let runner = config.runners.get(runner_name).unwrap();
        let mut local_runner_options = vec![];
        for i in &runner.options {
            if i.enable {
                local_runner_options.push(i.clone());
            }
        }
        let mut local_global_options = vec![];
        for i in &config.global {
            if i.enable {
                local_global_options.push(i.clone());
            }
        }
        let data = GameConfig {
            name: name.clone(),
            bin: add_bin.clone(),
            cwd: cwd.clone(),
            args: args.clone(),
            prefix: prefix.clone(),
            runner_name: runner_name.clone(),
            global: local_global_options,
            options: local_runner_options,
            command_base: runner.command.clone(),
            vars: runner.variables.clone(),
            icon: icon.clone(),
            installer: Some(String::new()),
        };
        println!("{:#?}", data);
        return data;
    }

    // pub fn from_game(&self, config: &mut Config) {
    //     if let Some(runner) = config.runners.get_mut(&self.runner_name) {
    //         for option in &self.options {
    //             if let Some(runner_option) =
    //                 runner.options.iter_mut().find(|o| o.name == option.name)
    //             {
    //                 runner_option.input = option.input.clone();
    //                 runner_option.enable = option.enable;
    //             }
    //         }
    //         for var in &self.vars {
    //             if let Some(runner_var) = runner.variables.iter_mut().find(|v| v.name == var.name) {
    //                 runner_var.input = var.input.clone();
    //             }
    //         }
    //     }
    //     for globaloption in &self.global {
    //         if let Some(runner_var) = config
    //             .global
    //             .iter_mut()
    //             .find(|v| v.name == globaloption.name)
    //         {
    //             runner_var.input = globaloption.input.clone();
    //             runner_var.enable = globaloption.enable;
    //         }
    //     }
    // }

    pub fn gen_cmd(game: &GameConfig) -> String {
        let mut cmd = game
            .command_base
            .clone()
            .replace("$add_bin", format!("\"{}\"", game.bin).as_str());
        let mut cmd_mod: HashMap<String, String> = HashMap::new();
        let mut cmd_prefix = game.prefix.clone();
        let mut cmd_command = String::new();
        let mut cmd_args = game.args.clone();

        let (global_prefix, global_command, global_args) = process_options(&game.global);
        cmd_prefix += &global_prefix;
        cmd_command += &global_command;
        cmd_args += &global_args;

        let (options_prefix, options_command, options_args) = process_options(&game.options);
        cmd_prefix += &options_prefix;
        cmd_command += &options_command;
        cmd_args += &options_args;

        for option in &game.vars {
            cmd_mod.insert(option.var.clone(), option.input.clone());
        }
        for (key, value) in cmd_mod {
            cmd = cmd.replace(key.as_str(), value.as_str());
        }
        return format!("{cmd_command}env {cmd_prefix}{cmd} {cmd_args}")
            .replace("$cwd", &game.cwd)
            .replace("$localdata", &Variables::lOCAL())
            .trim()
            .to_string();
    }

    pub fn execute(cmd: String, cwd: String) -> std::process::Child {
        #[cfg(target_os = "linux")]
        {
            println!("Ejecutando comando: {}", cmd);
            use std::process::Command;
            let pid = Command::new("setsid")
                .arg("bash")
                .current_dir(cwd)
                .arg("-c")
                .arg(cmd)
                .spawn()
                .expect("failed to execute process");
            println!("Proceso iniciado con PID: {}", pid.id());
            return pid;
        }
    }

    pub fn play(self, child: &mut Option<std::process::Child>) {
        // if !child.is_some() {
        notify(format!("{} iniciado", self.name).as_str());
        *child = Some(GameConfig::execute(
            GameConfig::gen_cmd(&self),
            self.cwd.clone(),
        ));
        // }
    }

    pub fn create_shortcut(self, id: &String) {
        let home = env::home_dir().unwrap().to_string_lossy().to_string();
        let path = format!("{home}/.local/share/applications/cat_games");
        let file = format!("{path}/{id}.desktop");
        fs::create_dir_all(&path).ok();
        create_file_all(&file, self.shortcut(&id).as_str());
        Command::new("chmod").args(["+x", &file]).spawn().ok();
        Command::new("update-desktop-database")
            .arg(format!("{home}/.local/share/applications"))
            .spawn()
            .ok();
    }
}

pub struct Games {
    pub configs: HashMap<String, GameConfig>,
    pub play: String,
    pub child: Option<std::process::Child>,
}

impl Games {
    pub fn play(&mut self) {
        if let Some(game) = self.configs.get(&self.play) {
            game.clone().play(&mut self.child);
            println!("Iniciado");
        }
    }
    pub fn shortcut(&mut self) {
        if let Some(game) = self.configs.get(&self.play) {
            game.clone().create_shortcut(&self.play);
        }
    }
    pub fn add_game(
        &mut self,
        path: &String,
        juego: &GameConfig,
        game_id: Option<String>,
        images: &mut HashMap<String, TextureHandle>,
        ctx: &egui::Context,
    ) {
        let mut id = String::from("cat_game1");
        let mut counter = 1;
        while self.configs.contains_key(&id) {
            id = format!("cat_game{}", counter);
            counter += 1;
        }
        if let Some(gid) = game_id {
            id = gid.clone();
            println!("Editing game with id: {}", id);
        }
        self.configs.insert(id.clone(), juego.clone());
        if Path::new(&juego.icon).exists() {
            if let Ok(img) = load_image(ctx, &juego.icon, &format!("{id}_icon")) {
                images.insert(id, img);
            }
        }
        let data = serde_json::to_string_pretty(&self.configs).unwrap();
        fs::write(path, data).unwrap();
    }

    pub fn delete_game(&mut self, game_id: &String) {
        if dialogos::confirm(
            "Confirmar",
            &format!(
                "¿Eliminar el juego \"{}\"?",
                self.configs.get(game_id).unwrap().name
            ),
        ) {
            let vars = vars::Variables::default();
            self.configs.remove(game_id);
            let data = serde_json::to_string_pretty(&self.configs).unwrap();
            let path = format!("{}/games.json", vars.CONFIG);
            fs::remove_file(format!("{}/{}.desktop", vars.DESKTOP, game_id)).ok();
            Command::new("update-desktop-database")
                .arg(format!("{}/.local/share/applications", vars.HOME))
                .spawn()
                .ok();
            fs::write(path, data).unwrap();
        }
    }
}
