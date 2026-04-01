// use std::process::Command;
use crate::MyApp;

pub fn panel(root: &mut MyApp, ctx: &egui::Context) {
    egui::SidePanel::right("mi_panel")
        // egui::TopBottomPanel::top("mi_panel")
        .resizable(false)
        .default_width(150.0)
        .show(ctx, |ui| {
            // ui.horizontal_centered(|ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Cat Games\n Launcher");
                ui.label(&root.games.play);
                if !root.games.child.is_none() {
                    // if ui.button("Detener").clicked() {
                    //     root.games.child.as_mut().unwrap().kill().ok();
                    //     root.games.child = None;
                    // }
                    // if ui.button("Matar").clicked() {
                    //     if let Some(child) = &root.games.child {
                    //         println!("{}", child.id());
                    //         let _ = Command::new("kill")
                    //             .arg("-9")
                    //             .arg(child.id().to_string())
                    //             .spawn();
                    //     }
                    // }
                }
                if !root.games.play.is_empty() {
                    if ui.button("Jugar").clicked() {
                        root.games.play();
                    };
                    if ui.button("Editar").clicked() {
                        root.ventanas.add.edit = true;
                        root.ventanas.active.add = true;
                        root.ventanas.add.title = format!("Editar {}", root.games.play);
                        root.ventanas.add.config.clear(&mut root.config);
                        root.ventanas.add.config.import_game(
                            root.games.configs.get(&root.games.play).unwrap(),
                            &mut root.config,
                        );
                    };
                    if ui.button("Eliminar").clicked() {
                        root.games.delete_game(&root.games.play.clone());
                        root.games.play = String::new();
                    };
                };
                if ui.button("Añadir").clicked() {
                    root.ventanas.add.title = "Añadir juego".to_string();
                    root.ventanas.add.config.clear(&mut root.config);
                    root.ventanas.active.add = true;
                };
                if ui.button("Importar").clicked() {
                    root.ventanas.active.import = true;
                };
                if ui.button("Configuración").clicked() {
                    root.ventanas.active.config = true;
                };
            });
        });
}
