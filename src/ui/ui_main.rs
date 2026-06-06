use std::collections::HashMap;
use egui::Context;
use crate::assets::assets::Assets;
use crate::map::map::Map;
use crate::map::entity_type::EntityType;
use crate::map::entity::Entity;
use crate::{SETTINGS, TILE_SIZE};
use crate::util::path::{get_filename, get_filename_without_ext};
use macroquad::math::Vec2;

#[derive(Debug, Default)]
pub struct MainUI {
    show_resources: bool,
    resource_users: HashMap<String, Vec<usize>>,
    res_tab: i32,
    show_users_of: Option<String>,
    show_users: bool,
    show_entities: bool,
    cached_loaded: Vec<String>,
    cached_missing: Vec<String>,
    entity_filter: Option<EntityType>,
    entity_types_cache: Vec<(String, Option<EntityType>, usize)>
}

impl MainUI {
    pub fn draw(&mut self, egui_ctx: &Context, map: &Map, assets: &Assets, world_target: &mut Vec2) {
        if self.cached_loaded.len() != assets.lookup.len() {
            self.cached_loaded = assets.lookup.keys().cloned().collect();
            self.cached_loaded.sort();
        }
        if self.cached_missing.len() != assets.failed.len() {
            self.cached_missing = assets.failed.iter().cloned().collect();
            self.cached_missing.sort();
        }

        SETTINGS.with(|s| {
            let mut settings = s.borrow_mut();

            // Main UI Window
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
                    ui.horizontal(|ui| {
                        if ui.button("Resources").clicked() {
                            self.show_resources = !self.show_resources;
                            self.resource_users.clear();
                            self.cached_loaded.clear();
                            self.cached_missing.clear();
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
                        if ui.button("Entities").clicked() {
                            self.show_entities = !self.show_entities;
                            self.entity_types_cache.clear();
                        }
                    });
                    ui.separator();
                    ui.label(format!("Map: {}", get_filename_without_ext(&map.path)));
                    ui.label(format!("Size: {}x{}", &map.size.x, &map.size.y));
                    ui.label(format!("Tiles: {}", get_filename(&map.tile_texture_filename)));
                    if map.header.author_name.len() > 0 && map.header.author_name != "Player" {
                        ui.label(format!("Author: {}", &map.header.author_name));
                    }
                });

            // Resources List
            if self.show_resources {
                let mut show_res = self.show_resources;
                egui::Window::new("Resources")
                    .open(&mut show_res)
                    .resizable(true)
                    .show(egui_ctx, |ui| {

                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.res_tab, 0, format!("Loaded ({})", &assets.lookup.len()));
                            ui.selectable_value(&mut self.res_tab, 1, format!("Missing ({})", &assets.failed.len()));
                        });

                        ui.separator();

                        if self.res_tab == 0 {
                            let row_height = ui.spacing().interact_size.y;
                            let num_rows = self.cached_loaded.len();
                            egui::ScrollArea::vertical()
                                .auto_shrink([false, false])
                                .show_rows(ui, row_height, num_rows, |ui, row_range| {
                                    for idx in row_range {
                                        let path = self.cached_loaded[idx].clone();
                                        self.draw_resource_row(ui, &path);
                                    }
                                });
                        } else {
                            let row_height = ui.spacing().interact_size.y;
                            let num_rows = self.cached_missing.len();
                            egui::ScrollArea::vertical()
                                .auto_shrink([false, false])
                                .show_rows(ui, row_height, num_rows, |ui, row_range| {
                                    for idx in row_range {
                                        let path = self.cached_missing[idx].clone();
                                        self.draw_resource_row(ui, &path);
                                    }
                                });
                        }
                    });
                self.show_resources = show_res;
            }

            // Entities
            if self.show_entities {
                if self.entity_types_cache.is_empty() && !map.entities.is_empty() {
                    let mut counts: HashMap<u8, usize> = HashMap::new();
                    for entity in &map.entities {
                        *counts.entry(entity.entity_type.clone() as u8).or_insert(0) += 1;
                    }
                    let mut list: Vec<(String, Option<EntityType>, usize)> = Vec::new();
                    list.push((format!("All ({})", map.entities.len()), None, map.entities.len()));

                    let mut typed_entries: Vec<(String, EntityType, usize)> = Vec::new();
                    for (etype_u8, count) in counts {
                        let etype = EntityType::from(etype_u8);
                        typed_entries.push((etype.get_name().to_string(), etype, count));
                    }
                    typed_entries.sort_by(|a, b| a.0.cmp(&b.0));

                    for (name, etype, count) in typed_entries {
                        list.push((format!("{} ({})", name, count), Some(etype), count));
                    }
                    self.entity_types_cache = list;
                }

                let mut show_entities = self.show_entities;
                egui::Window::new("Entities")
                    .open(&mut show_entities)
                    .resizable(true)
                    .show(egui_ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("Total Entities: {}", map.entities.len()));

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let selected_label = if let Some(ref entry) = self.entity_types_cache.iter().find(|x| x.1 == self.entity_filter) {
                                    entry.0.clone()
                                } else {
                                    "All".to_string()
                                };
                                egui::ComboBox::from_id_salt("entity_filter_combo")
                                    .selected_text(&selected_label)
                                    .show_ui(ui, |ui| {
                                        for entry in &self.entity_types_cache {
                                            ui.selectable_value(&mut self.entity_filter, entry.1.clone(), &entry.0);
                                        }
                                    });
                            });
                        });
                        ui.separator();

                        let filtered_indices: Vec<usize> = if let Some(ref filter_type) = self.entity_filter {
                            map.entities.iter().enumerate()
                                .filter(|(_, e)| e.entity_type == *filter_type)
                                .map(|(i, _)| i)
                                .collect()
                        } else {
                            (0..map.entities.len()).collect()
                        };

                        let row_height = ui.spacing().interact_size.y;
                        let num_rows = filtered_indices.len();
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show_rows(ui, row_height, num_rows, |ui, row_range| {
                                for idx in row_range {
                                    let i = filtered_indices[idx];
                                    let entity = &map.entities[i];
                                    self.draw_entity_button(ui, i, entity, world_target);
                                }
                            });
                    });
                self.show_entities = show_entities;
            }

            // Resource Users
            if self.show_users && self.show_users_of.is_some() {
                let mut show_users = self.show_users;
                egui::Window::new("Resource Users")
                    .open(&mut show_users)
                    .resizable(true)
                    .show(egui_ctx, |ui| {
                        ui.label(format!("'{}' is used by:", self.show_users_of.as_ref().unwrap()));
                        ui.separator();
                        let lower_path = self.show_users_of.as_ref().unwrap().to_lowercase();
                        let users = self.resource_users.get(&lower_path).cloned();
                        if let Some(user_list) = users {
                            let row_height = ui.spacing().interact_size.y;
                            let num_rows = user_list.len();
                            egui::ScrollArea::vertical()
                                .auto_shrink([false, false])
                                .show_rows(ui, row_height, num_rows, |ui, row_range| {
                                    for idx in row_range {
                                        let user = user_list[idx];
                                        let entity = &map.entities[user];
                                        self.draw_entity_button(ui, user, entity, world_target);
                                    }
                                });
                        }
                    });
                self.show_users = show_users;
            }
        });
    }

    fn draw_resource_row(&mut self, ui: &mut egui::Ui, path: &str) {
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let lower_path = path.to_lowercase();
                let users = self.resource_users.get(&lower_path);
                let user_count = if users.is_some() { users.unwrap().len() } else { 0 };
                let btn = egui::Button::new(format!("{}", user_count));
                if ui.add_sized([45.0, ui.spacing().interact_size.y], btn).clicked() {
                    self.show_users_of = Some(lower_path.clone());
                    self.show_users = true;
                }
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add(egui::Label::new(path));
                });
            });
        });
    }

    fn draw_entity_button(&mut self, ui: &mut egui::Ui, index: usize, entity: &Entity, world_target: &mut Vec2) {
        let text = format!("[{}] {} @ {}|{}", index, entity.entity_type.get_name(), entity.position.x, entity.position.y);
        let btn = egui::Button::new(text);
        if ui.add_sized([ui.available_width(), ui.spacing().interact_size.y], btn).clicked() {
            world_target.x = entity.position.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
            world_target.y = entity.position.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;
        }
    }
}