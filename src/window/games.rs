use crate::{
    MyApp,
    libs::{importer, utils::notify},
    utils::game::GameConfig,
};
use egui::{Shadow, Vec2};

pub fn draw_game(root: &mut MyApp, game: GameConfig, i: &str, ui: &mut egui::Ui) {
    //                             ^^^^^^^^^^^ por valor (requiere Clone en GameConfig)
    let imagen = root.images.get(i).unwrap_or(&root.default_img);

    let img = egui::Image::new(imagen)
        .sense(egui::Sense::click())
        .fit_to_exact_size(Vec2 { x: 100.0, y: 100.0 })
        .maintain_aspect_ratio(false);

    ui.scope(|ui| {
        if i == root.games.play {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
        } else {
            ui.visuals_mut().window_shadow = Shadow::NONE;
        }

        egui::Frame::window(ui.style()).show(ui, |ui| {
            ui.vertical(|ui| {
                let w = ui.add(img); // sin & — Response no necesita ser referencia
                ui.label(&game.name);

                if w.double_clicked() {
                    root.games.play = i.to_string().clone();
                    root.games.play();
                }

                if w.clicked() {
                    root.games.play = i.to_string().clone();
                } else {
                    w.context_menu(|ui| {
                        root.games.play = i.to_string().clone();
                        if ui.button("Jugar").clicked() {
                            root.games.play();
                        };
                        if ui.button("Editar").clicked() {
                            root.ventanas.add.edit = true;
                            root.ventanas.active.add = true;
                            root.ventanas.add.config.import_game(
                                root.games.configs.get(&root.games.play).unwrap(),
                                &mut root.config,
                            );
                        };
                        if ui.button("Crear atajo").clicked() {
                            root.games.shortcut();
                            notify(format!("Atajo Creado {}", &root.games.play).as_str());
                        };
                        if ui.button("Exportar Config").clicked() {
                            let game = root.games.configs.get(&root.games.play).unwrap();
                            importer::export_game(game, &game.cwd).ok();
                        }
                        if ui.button("Borrar").clicked() {
                            root.games.delete_game(&root.games.play.clone());
                            root.games.play = String::new();
                        };
                    });
                }
            });
        });
    });
}

pub fn draw(root: &mut MyApp, ui: &mut egui::Ui, width: i32) {
    let keys: Vec<String> = root.games.configs.keys().cloned().collect();
    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("Juegos").show(ui, |ui| {
            let mut index = 0;

            for i in &keys {
                let game = root.games.configs[i].clone();
                draw_game(root, game, i, ui);
                index += 1;
                if index >= width {
                    index = 0;
                    ui.end_row();
                }
            }
        });
    });
}
