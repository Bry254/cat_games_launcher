use std::collections::HashMap;
use std::{env, fs, path::Path};

use eframe::{App, egui};
use egui::{IconData, TextureHandle};

use crate::libs::{cli::args_parse, vars::Variables};
use crate::utils::{
    config::Config,
    game::{GameConfig, Games},
};
mod libs;
mod utils;
mod window;

#[derive(Default)]
struct Actives {
    add: bool,
    config: bool,
    import: bool,
}

struct Ventanas {
    add: window::add::WinAdd,
    config: window::config::ConfigWin,
    import: window::import::WinImp,
    active: Actives,
}

struct MyApp {
    ventanas: Ventanas,
    config: Config,
    games: Games,
    default_img: egui::TextureHandle,
    images: HashMap<String, TextureHandle>,
}

impl MyApp {
    fn new(ctx: &eframe::CreationContext<'_>) -> Self {
        let variables = Variables::default();
        let data = libs::utils::read_file(
            format!(
                "{}/.config/cat_games_launcher/options.json",
                &variables.HOME
            )
            .as_str(),
        );
        let mut config: utils::config::Config = serde_json::from_str(data.as_str()).unwrap();
        for (_, runner) in config.runners.iter_mut() {
            for i in runner.variables.iter_mut() {
                if i.mode == "file_names" {
                    let path = i.cmd.replace("$HOME", &variables.HOME);
                    let dir = Path::new(&path);
                    for entry in fs::read_dir(dir).unwrap().flatten() {
                        i.values
                            .push(entry.file_name().to_string_lossy().to_string());
                    }
                }
            }
        }
        let games = libs::utils::read_file(
            format!("{}/.config/cat_games_launcher/games.json", variables.HOME).as_str(),
        );
        let games: HashMap<String, GameConfig> = serde_json::from_str(games.as_str()).unwrap();
        let mut images = HashMap::default();
        for (i, v) in &games {
            if !v.icon.is_empty() {
                if Path::new(&v.icon).exists() {
                    if let Ok(imagen) =
                        libs::imagen::load_image(&ctx.egui_ctx, &v.icon, &format!("{i}_icon"))
                    {
                        images.insert(i.to_owned(), imagen);
                    }
                }
            }
        }
        Self {
            games: Games {
                configs: games,
                play: "".to_string(),
                child: None,
            },
            default_img: libs::imagen::load_embedded_texture(
                ctx,
                "default",
                include_bytes!("logo.png"),
            )
            .unwrap(),
            ventanas: Ventanas {
                add: window::add::WinAdd::default(),
                config: window::config::ConfigWin::default(),
                import: window::import::WinImp::default(),
                active: Actives::default(),
            },
            images: images,
            config: config,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        window::panel::panel(self, ctx);
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            let size = ui.available_width();
            let w = (size / 120.0).floor() as i32;
            window::games::draw(self, ui, w);
            if self.ventanas.active.add {
                self.ventanas.add.show(
                    ctx,
                    &mut self.ventanas.active.add,
                    &mut self.config,
                    &mut self.games,
                    &mut self.images,
                );
            };
            if self.ventanas.active.config {
                self.ventanas
                    .config
                    .draw(ctx, &mut self.ventanas.active.config, &self.games);
            };
            if self.ventanas.active.import {
                self.ventanas.import.draw(
                    ctx,
                    &mut self.ventanas.active.import,
                    &mut self.ventanas.add,
                    &mut self.games,
                    &mut self.images,
                    &mut self.ventanas.config,
                    &mut self.config,
                );
            };
        });
    }
}

fn load_icon() -> IconData {
    let bytes = include_bytes!("icon.png");

    let image = image::load_from_memory(bytes)
        .expect("No se pudo cargar el icono")
        .into_rgba8();

    let (width, height) = image.dimensions();

    IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

fn create_file_all(file: String, data: &str) {
    if !Path::new(&file).exists() {
        fs::write(&file, data).ok();
    }
}

fn main() {
    let vars = Variables::default();
    Variables::gen_path().unwrap();
    let argumentos: Vec<String> = env::args().skip(1).collect();
    if argumentos.len() > 0 {
        args_parse(argumentos).unwrap();
        return;
    }
    if Path::new(&format!("{}/cat_games_launcher.png", vars.ICONS)).exists() {
        fs::write(
            &format!("{}/cat_games_launcher.png", vars.ICONS),
            include_bytes!("icon.png"),
        )
        .expect("No se pudo copiar el icono del launcher.")
    };
    create_file_all(
        format!("{}/options.json", vars.CONFIG),
        include_str!("options.json"),
    );
    create_file_all(format!("{}/games.json", vars.CONFIG), "{}");

    let mut options = eframe::NativeOptions::default();
    options.viewport = egui::ViewportBuilder::default()
        .with_app_id("cat_games_launcher")
        .with_icon(load_icon());
    let _ = eframe::run_native(
        "Cat Games Launcher",
        options,
        Box::new(|ctx| {
            egui_extras::install_image_loaders(&ctx.egui_ctx);
            ctx.egui_ctx.set_pixels_per_point(1.5);
            Ok(Box::new(MyApp::new(ctx)))
        }),
    );
}
