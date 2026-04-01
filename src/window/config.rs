use std::{env, fs, process::Command};
// use crate::dialogos;
use crate::{
    libs::{desktop, dialogos, utils::notify},
    utils::game::Games,
};
#[derive(Clone)]
pub struct ConfigWin {
    pub gamespath: String,
    pub terminal: String,
}

impl Default for ConfigWin {
    fn default() -> Self {
        Self {
            gamespath: String::new(),
            terminal: String::from("x-terminal-emulator -e "),
        }
    }
}

impl ConfigWin {
    pub fn create_shortcut(&self) {
        let home = env::home_dir().unwrap().to_string_lossy().to_string();
        let path =
            format!("{home}/.local/share/applications/cat_games/Cat_launcher.desktop").to_string();
        let data = format!(
            "[Desktop Entry]\nName=Cat Games Launcher\nComment=Cat Games Launch\nExec={}\nIcon=cat_games_launcher\nTerminal=false\nType=Application\nCategories=Application;Game;\nStartupNotify=true",
            env::current_exe().unwrap().to_string_lossy(),
        )
        .to_string();
        fs::write(&path, data).ok();
        Command::new("chmod").args(["+x", &path]).spawn().ok();
    }
    pub fn draw(&mut self, ctx: &egui::Context, activo: &mut bool, games: &Games) {
        egui::Window::new("Configuraciones")
            .scroll(true)
            .open(activo)
            .default_pos(ctx.content_rect().center())
            .show(ctx, |ui| {
                ui.heading("Atajos");
                if ui.button("Crear/Actualizar Atajo").clicked() {
                    self.create_shortcut();
                };
                if ui.button("Abrir carpeta de atajos").clicked() {
                    let home = env::home_dir().unwrap().to_string_lossy().to_string();
                    Command::new("xdg-open")
                        .args([format!("{home}/.local/share/applications/cat_games")])
                        .spawn()
                        .ok();
                };
                if ui.button("Actualizar todos los atajos").clicked() {
                    if desktop::update_all_desktops(&games).is_ok() {
                        notify("Atajos Actualizados");
                    };
                }
                ui.heading("Otros Ajustes");
                ui.horizontal(|ui| {
                    ui.label("Carpeta de juegos");
                    if ui.button("Selecionar carpeta").clicked() {
                        let path = dialogos::folderpicker();
                        if !path.is_empty() {
                            self.gamespath = path;
                        }
                    };
                    ui.text_edit_singleline(&mut self.gamespath);
                });
                ui.horizontal(|ui| {
                    ui.label("Terminal");
                    ui.text_edit_singleline(&mut self.terminal);
                });
            });
    }
}
