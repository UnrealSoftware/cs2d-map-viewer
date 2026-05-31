use egui::Context;
use crate::assets::assets::Assets;
use crate::map::map::Map;
use crate::SETTINGS;
use crate::util::path::{get_filename, get_filename_without_ext};

#[derive(Debug, Default)]
pub struct MainUI {
    show_resources: bool,
    res_tab: i32,
}

impl MainUI {
    pub fn draw(&mut self, egui_ctx: &Context, map: &Map, assets: &Assets) {
        SETTINGS.with(|s| {
            let mut settings = s.borrow_mut();

            egui::Window::new("Settings / Info")
                .default_open(false)
                .resizable(false)
                .show(egui_ctx, |ui| {
                    ui.checkbox(&mut settings.grid, "Grid");
                    ui.checkbox(&mut settings.shadows, "Shadows");
                    ui.checkbox(&mut settings.decals, "Decals");
                    ui.checkbox(&mut settings.entities, "Entities");
                    ui.checkbox(&mut settings.entity_fx, "Entity Graphics/FX");
                    ui.separator();
                    if ui.button("Resources").clicked() {
                        self.show_resources = !self.show_resources;
                    }
                    ui.separator();
                    ui.label(format!("Map: {}", get_filename_without_ext(&map.path)));
                    ui.label(format!("Size: {}x{}", &map.size.x, &map.size.y));
                    ui.label(format!("Tiles: {}", get_filename(&map.tile_texture_filename)));
                    if map.header.author_name.len() > 0 && map.header.author_name != "Player" {
                        ui.label(format!("Author: {}", &map.header.author_name));
                    }
                });

            if self.show_resources {
                egui::Window::new("Resources")
                    .open(&mut self.show_resources)
                    .resizable(true)
                    .show(egui_ctx, |ui| {

                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.res_tab, 0, format!("Loaded ({})", &assets.lookup.len()));
                            ui.selectable_value(&mut self.res_tab, 1, format!("Missing ({})", &assets.failed.len()));
                        });

                        ui.separator();

                        if self.res_tab == 0 {
                            egui::ScrollArea::vertical()
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    for (path, _) in &assets.lookup {
                                        ui.label(path);
                                    }
                                });
                        } else {
                            egui::ScrollArea::vertical()
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    for path in &assets.failed {
                                        ui.label(path);
                                    }
                                });
                        }

                    });
            }
        });
    }
}