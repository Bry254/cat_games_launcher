use anyhow::Ok;

use crate::{
    libs::{dialogos, extractor, importer::import_game, installer, utils::notify},
    utils::{config::Config, game::Games},
    window::{add::WinAdd, config::ConfigWin},
};
use std::{fs::create_dir_all, path::Path};

#[derive(Default)]
pub struct WinImp {}

impl WinImp {
    pub fn draw(
        &mut self,
        ctx: &egui::Context,
        activo: &mut bool,
        winadd: &mut WinAdd,
        games: &mut Games,
        images: &mut std::collections::HashMap<String, egui::TextureHandle>,
        config: &mut ConfigWin,
        mut configfile: &mut Config,
    ) {
        egui::Window::new("Importar")
            .default_size(egui::vec2(200.0, 200.0))
            .scroll(true)
            .open(activo)
            .show(ctx, |ui| {
                ui.heading("Importar Juegos");
                if ui.button("Importar juego (.cat_game)").clicked() {
                    let path = dialogos::filepicker("Juego", &["cat_game"]);
                    if !path.is_empty() {
                        import_game(&path, games, images, ctx).ok();
                    };
                }
                if ui.button("Importar juego (.zip").clicked() {
                    if WinImp::import_game_zip(games, images, ctx, winadd, config).is_ok() {
                        notify("Juego importado correctamente.");
                    } else {
                        notify("Ocurrio un error al importar.");
                    };
                }
                ui.heading("Importar Runners");
                if ui.button("Importar Runner (.cat_runner)").clicked() {
                    let path = dialogos::filepicker("instalador .cat_runner", &["cat_runner"]);
                    if !path.is_empty() {
                        println!("[1/3] instalando {path}");
                        let installer = installer::Runner_installer::new(&path).ok();
                        println!("{:?}", installer);
                        if let Some(installer) = installer {
                            println!("[2/3] instalando {path}");
                            if installer.install().is_ok() {
                                println!("[3/3] instalando {path}");
                                installer.import(&mut configfile);
                            };
                        }
                    };
                }
                // if ui.button("Importar Runner (.zip)").clicked() {}
            });
    }
    // pub fn import_game_file() {}
    pub fn import_game_zip(
        games: &mut Games,
        images: &mut std::collections::HashMap<String, egui::TextureHandle>,
        ctx: &egui::Context,
        winadd: &mut WinAdd,
        winconfig: &mut ConfigWin,
    ) -> anyhow::Result<()> {
        let path = dialogos::filepicker("archivo .zip", &["zip"]);
        if !path.is_empty() {
            if let Some(name) = Path::new(&path).file_stem() {
                let name = name.to_string_lossy().to_string();
                let folder;
                if winconfig.gamespath.is_empty() {
                    let dest = dialogos::folderpicker_title("Selecione carpeta de destino");
                    if dest.is_empty() {
                        return Err(anyhow::anyhow!("Error al selecionar carpeta"));
                    } else {
                        folder = Path::new(&dest).join(name);
                    }
                } else {
                    folder = Path::new(&winconfig.gamespath).join(name);
                }
                create_dir_all(&folder)?;
                notify("Extrayendo el juego...");
                extractor::unzip(&path, &folder.to_string_lossy().to_string())?;
                let cat_game = folder.join("config.cat_game");
                if cat_game.exists() {
                    import_game(&cat_game.to_string_lossy().to_string(), games, images, &ctx)?
                } else {
                    notify("El archivo config.cat_game no existe.");
                    winadd.config.simple_clear();
                    winadd.config.cwd = folder.to_string_lossy().to_string();
                    // ventanas.import_active = false;
                }
            }
        }
        return Ok(());
    }
}
