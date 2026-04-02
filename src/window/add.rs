use egui::TextureHandle;

use crate::libs::{desktop::IconProcesor, dialogos, lutris, utils::notify};
use crate::utils::config::{Config, Runner, RunnerOption, RunnerVarOption};
use crate::utils::game::{GameConfig, Games};
use std::env;
use std::{collections::HashMap, path::Path};

#[derive(Default, Clone)]
pub struct WinAdd {
    pub cerrar: bool,
    pub config: ConfigAdd,
    pub edit: bool,
    pub title: String,
    pub iconfinder: IconProcesor,
}

#[derive(Default, Clone)]
pub struct ConfigAdd {
    pub icon: String,
    pub runner: String,
    pub name: String,
    pub bin: String,
    pub cwd: String,
    pub args: String,
    pub env: String,
}

impl ConfigAdd {
    pub fn import_game(&mut self, game: &GameConfig, configs: &mut Config) {
        self.name = game.name.clone();
        self.bin = game.bin.clone();
        self.cwd = game.cwd.clone();
        self.args = game.args.clone();
        self.env = game.prefix.clone();
        self.icon = game.icon.clone();
        self.runner = game.runner_name.clone();

        if let Some(runner) = configs.runners.get_mut(&game.runner_name) {
            for option in &game.options {
                if let Some(runner_option) =
                    runner.options.iter_mut().find(|o| o.name == option.name)
                {
                    runner_option.input = option.input.clone();
                    runner_option.enable = option.enable;
                }
            }

            for var in &game.vars {
                if let Some(runner_var) = runner.variables.iter_mut().find(|v| v.name == var.name) {
                    runner_var.input = var.input.clone();
                }
            }
        }

        for global_option in &game.global {
            if let Some(global_var) = configs
                .global
                .iter_mut()
                .find(|v| v.name == global_option.name)
            {
                global_var.input = global_option.input.clone();
                global_var.enable = global_option.enable;
            }
        }
    }
    pub fn simple_clear(&mut self) {
        self.runner = String::new();
        self.name = String::new();
        self.bin = String::new();
        self.cwd = String::new();
        self.args = String::new();
        self.env = String::new();
        self.icon = String::new();
    }
    pub fn clear(&mut self, configs: &mut Config) {
        self.runner = String::new();
        self.name = String::new();
        self.bin = String::new();
        self.cwd = String::new();
        self.args = String::new();
        self.env = String::new();
        self.icon = String::new();
        for i in configs.global.iter_mut() {
            i.enable = false;
            i.input = "".to_string();
        }
        if let Some(runner) = configs.runners.get_mut(&self.runner) {
            for i in runner.options.iter_mut() {
                i.enable = false;
                i.input = "".to_string();
            }
        };
    }
}

impl WinAdd {
    pub fn draw_varwidgets(ui: &mut egui::Ui, option: &mut RunnerVarOption) {
        ui.label(&option.name);
        match option.mode.as_str() {
            "folder" => {
                ui.text_edit_singleline(&mut option.input);
                if ui.button("Selecionar carpeta").clicked() {
                    option.input = dialogos::folderpicker();
                }
            }
            "input" => {
                ui.text_edit_singleline(&mut option.input);
            }
            "file_names" => {
                egui::ComboBox::from_label("")
                    .selected_text(&option.input)
                    .show_ui(ui, |ui| {
                        for v in &option.values {
                            ui.selectable_value(&mut option.input, v.clone(), v);
                        }
                    });
            }
            _ => {}
        }
    }

    pub fn draw_widgets(ui: &mut egui::Ui, option: &mut RunnerOption) {
        let mut enabled = option.enable;
        ui.checkbox(&mut enabled, &option.name);
        option.enable = enabled;
        match option.mode.as_str() {
            "folder" => {
                ui.text_edit_singleline(&mut option.input);
                if ui.button("Selecionar carpeta").clicked() {
                    option.input = dialogos::folderpicker();
                }
            }
            "input" => {
                ui.text_edit_singleline(&mut option.input);
            }
            "file_names" => {
                egui::ComboBox::from_label(&option.name)
                    .selected_text(&option.input)
                    .show_ui(ui, |ui| {
                        for v in &option.values {
                            ui.selectable_value(&mut option.input, v.clone(), v);
                        }
                    });
            }
            _ => {}
        }
    }

    pub fn gen_var_widgets(&mut self, ui: &mut egui::Ui, runners: &mut HashMap<String, Runner>) {
        egui::Grid::new("variables")
            .min_col_width(120.0)
            .show(ui, |ui| {
                if let Some(runner) = runners.get_mut(&self.config.runner) {
                    for var in runner.variables.iter_mut() {
                        WinAdd::draw_varwidgets(ui, var);
                        ui.end_row();
                    }
                }
            });
    }

    pub fn gen_global_widgets(&mut self, ui: &mut egui::Ui, global: &mut Vec<RunnerOption>) {
        egui::Grid::new("OpcionesGlobales")
            .min_col_width(120.0)
            .show(ui, |ui| {
                for option in global.iter_mut() {
                    WinAdd::draw_widgets(ui, option);
                    ui.end_row();
                }
            });
    }
    pub fn gen_options_widgets(
        &mut self,
        ui: &mut egui::Ui,
        runners: &mut HashMap<String, Runner>,
    ) {
        egui::Grid::new("OpcionesGlobales")
            .min_col_width(120.0)
            .show(ui, |ui| {
                for option in runners
                    .get_mut(&self.config.runner)
                    .unwrap()
                    .options
                    .iter_mut()
                {
                    WinAdd::draw_widgets(ui, option);
                    ui.end_row();
                }
            });
    }

    // pub fn show(&mut self, ctx: &egui::Context, root: &mut MyApp) {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        activo: &mut bool,
        config: &mut Config,
        games: &mut Games,
        images: &mut HashMap<String, TextureHandle>,
    ) {
        egui::Window::new(&self.title)
            .scroll(true)
            .open(activo)
            .default_pos(ctx.content_rect().center())
            .show(ctx, |ui| {
                egui::ComboBox::from_label("Ejecutor")
                    .selected_text(&self.config.runner)
                    .show_ui(ui, |ui| {
                        for (i, _) in config.runners.iter() {
                            ui.selectable_value(&mut self.config.runner, i.clone(), i);
                        }
                    });

                egui::Grid::new("DatosJuego")
                    .min_col_width(120.0)
                    .show(ui, |ui| {
                        ui.label("Icono: ");
                        ui.text_edit_singleline(&mut self.config.icon);
                        if ui.button("Selecionar icono").clicked() {
                            self.config.icon = dialogos::iconpicker();
                            println!("icono {}", self.config.icon);
                            self.config.icon = self.iconfinder.resolve_icon(
                                &self.config.icon,
                                &self.config.cwd,
                                &self.config.name,
                            );
                        }
                        ui.end_row();

                        ui.label("Nombre:");
                        ui.text_edit_singleline(&mut self.config.name);
                        ui.end_row();

                        ui.label("Ejecutable:");
                        ui.text_edit_singleline(&mut self.config.bin);

                        if ui.button("Selecionar").clicked() {
                            let bin;
                            if self.config.cwd.is_empty() {
                                bin = dialogos::filepicker("Binario", &["*"]);
                            } else {
                                bin =
                                    dialogos::filepicker_path("Binario", &["*"], &self.config.cwd);
                            }
                            if !bin.is_empty() {
                                self.config.bin = bin;
                                let file = Path::new(&self.config.bin);

                                if self.config.cwd.is_empty() {
                                    if let Some(parent) = file.parent() {
                                        self.config.cwd = parent.to_string_lossy().to_string();
                                    }
                                }

                                if self.config.name.is_empty() {
                                    if let Some(name) = file.file_name() {
                                        self.config.name = name
                                            .to_string_lossy()
                                            .to_string()
                                            .replace(".exe", "")
                                            .to_lowercase();
                                    }
                                }

                                if self.config.bin.ends_with(".exe") && self.config.icon.is_empty()
                                {
                                    println!("Es un exe");
                                    self.config.icon = self.iconfinder.icon_exe(
                                        &self.config.cwd,
                                        &self.config.bin,
                                        &self.config.name,
                                    );
                                }

                                if self.config.icon.is_empty() {
                                    self.config.icon = self
                                        .iconfinder
                                        .locate_icon(&self.config.bin, &self.config.name);
                                }

                                if self.config.icon.is_empty() {
                                    if let Ok(icon) =
                                        lutris::get_icon(&self.config.name.to_lowercase())
                                    {
                                        self.config.icon = icon;
                                    }
                                }
                            }
                        }
                        ui.end_row();

                        ui.label("Carpeta de trabajo:");
                        ui.text_edit_singleline(&mut self.config.cwd);
                        if ui.button("Selecionar").clicked() {
                            self.config.cwd =
                                dialogos::folderpicker_title("Selecione la carpeta del juego");
                        }
                        ui.end_row();

                        ui.label("Argumentos:");
                        ui.text_edit_singleline(&mut self.config.args);
                        ui.end_row();

                        ui.label("Variables:");
                        ui.text_edit_singleline(&mut self.config.env);
                        ui.end_row();
                    });

                if !self.config.runner.is_empty() {
                    self.gen_var_widgets(ui, &mut config.runners);
                }

                self.gen_global_widgets(ui, &mut config.global);

                if !self.config.runner.is_empty() {
                    egui::CollapsingHeader::new("Opciones avanzadas").show(ui, |ui| {
                        self.gen_options_widgets(ui, &mut config.runners);
                    });

                    if ui.button("Guardar").clicked() {
                        let mut id = None;

                        if self.edit {
                            id = Some(games.play.clone());
                            self.edit = false;
                            self.title = "Añadir Juego".to_string();
                        }
                        games.add_game(
                            &format!(
                                "{}/.config/cat_games_launcher/games.json",
                                env::home_dir().unwrap().to_string_lossy()
                            ),
                            &GameConfig::new(
                                &config.clone(),
                                &self.config.runner,
                                &self.config.name,
                                &self.config.bin.replace(&self.config.cwd, "$cwd"),
                                &self.config.cwd,
                                &self.config.args,
                                &self.config.env,
                                &self.config.icon,
                            ),
                            id,
                            images,
                            ctx,
                        );
                        self.cerrar = true;
                    }
                }
            });
        if self.cerrar {
            *activo = false;
            self.cerrar = false;
        }
    }
}
