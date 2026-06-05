use std::collections::HashMap;
use egui::Context;
use crate::assets::assets::Assets;
use crate::map::map::Map;
use crate::map::entity_type::EntityType;
use crate::SETTINGS;
use crate::util::path::{get_filename, get_filename_without_ext};

#[derive(Debug, Default)]
pub struct MainUI {
    show_resources: bool,
    resource_users: HashMap<String, Vec<usize>>,
    res_tab: i32,
    show_users_of: Option<String>,
    show_users: bool
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
                        self.resource_users.clear();
                        for i in 0..map.entities.len() {
                            match map.entities[i].entity_type {
                                EntityType::EnvSprite | EntityType::EnvSound | EntityType::EnvImage => {
                                    let path = map.entities[i].strings[0].to_lowercase();
                                    if path.is_empty() {
                                       continue;
                                    }
                                    let users = self.resource_users.get_mut(&path);
                                    if users.is_some() {
                                        users.unwrap().push(i);
                                    } else {
                                        let mut new_users = Vec::new();
                                        new_users.push(i);
                                        self.resource_users.insert(path.clone(), new_users);
                                    }
                                }
                                _ => {}
                            }
                        }
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
                                        ui.horizontal(|ui| {
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                let lower_path = path.to_lowercase();
                                                let users = self.resource_users.get(&lower_path);
                                                let user_count = if users.is_some() { users.unwrap().len() } else { 0 };
                                                if ui.button(format!("{}", user_count)).clicked() {
                                                    self.show_users_of = Some(lower_path.clone());
                                                    self.show_users = true;
                                                }
                                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                                    ui.add(egui::Label::new(path));
                                                });
                                            });
                                        });
                                    }
                                });
                        } else {
                            egui::ScrollArea::vertical()
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    for path in &assets.failed {
                                        ui.horizontal(|ui| {
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                let lower_path = path.to_lowercase();
                                                let users = self.resource_users.get(&lower_path);
                                                let user_count = if users.is_some() { users.unwrap().len() } else { 0 };
                                                if ui.button(format!("{}", user_count)).clicked() {
                                                    self.show_users_of = Some(lower_path.clone());
                                                    self.show_users = true;
                                                }
                                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                                    ui.add(egui::Label::new(path));
                                                });
                                            });
                                        });
                                    }
                                });
                        }
                    });
            }

            if self.show_users && self.show_users_of.is_some() {
                egui::Window::new("Resource Users")
                    .open(&mut self.show_users)
                    .resizable(true)
                    .show(egui_ctx, |ui| {
                        ui.label(format!("'{}' is used by:", self.show_users_of.as_ref().unwrap()));
                        let lower_path = self.show_users_of.as_ref().unwrap().to_lowercase();
                        let users = self.resource_users.get(&lower_path);
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                for user in users.unwrap().clone() {
                                    let entity = &map.entities[user];
                                    ui.label(format!("{} @ {}x{}", entity.entity_type.get_name(), entity.position.x, entity.position.y));
                                }
                            });
                    });
            }
        });
    }
}