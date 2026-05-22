use macroquad::prelude::*;
use crate::map::entity::Entity;
use crate::map::entity_type::EntityType;
use crate::map::map::Map;

#[derive(Debug, Default)]
pub struct TileFx {
    pub tile_index: usize,
    pub frames: Vec<Texture2D>,
    pub animation_start: usize,
    pub animation_end: usize,
    pub current_frame: usize,
    pub speed: f32,
    pub update_time: f32,
}

#[derive(Debug, Default)]
pub struct TileFxManager {
    pub effects: Vec<TileFx>,
    pub mapping: Vec<usize>,
}

impl TileFxManager {
    pub fn free_all(&mut self) {
        self.effects.clear();
        for i in 0..self.mapping.len() {
            self.mapping[i] = usize::MAX;
        }
    }

    pub fn init(&mut self, map: &Map) {
        self.free_all();
        self.mapping.clear();

        if map.tile_texture.is_none() {
            error!("can't init tile fx: no tileset");
            return
        }

        let tile_count = map.tile_texture.as_ref().unwrap().frame_count as usize;
        while self.mapping.len() < tile_count {
            self.mapping.push(usize::MAX);
        }

        for e in &map.entities {
            let tile_idx = e.ints[0] as usize;
            if tile_idx >= tile_count {
                error!("tried to create FX on an inexistent tile!");
                continue;
            }

            if self.effects.iter().any(|fx| fx.tile_index == tile_idx) {
                error!("tile already has FX!");
                continue;
            }

            match e.entity_type {
                EntityType::InfoAnimation => self.create_animation_fx(&e, &map),
                EntityType::InfoTileFx => self.create_tile_fx(&e, &map),
                _ => {}
            }
        }
    }

    fn create_animation_fx(&mut self, e: &Entity, mut map: &Map) {
        let tile_idx = e.ints[0] as usize;
        let f_start = tile_idx;
        let mut f_end = e.ints[1] as usize;
        let speed_ms = e.ints[2] as f32;

        let tiles = map.tile_texture.as_ref().unwrap();

        if f_end >= tiles.frame_count as usize { f_end = tiles.frame_count as usize - 1; }
        if f_end < f_start { f_end = f_start; }

        self.effects.push(TileFx {
            tile_index: tile_idx,
            animation_start: f_start,
            animation_end: f_end,
            current_frame: f_start,
            speed: speed_ms / 1000.0,
            ..TileFx::default()
        });
        self.mapping[tile_idx] = self.effects.len() - 1;
    }

    fn create_tile_fx(&mut self, e: &Entity, map: &Map) {
        let tile_idx = e.ints[0] as usize;
        let tiles = map.tile_texture.as_ref().unwrap();
        let original_texture = tiles.extract_frame_texture(tile_idx as u16);

        let src_img = original_texture.get_texture_data();
        let w = src_img.width as i32;
        let h = src_img.height as i32;
        let scale = w / 32;

        let fx_type = e.ints[1];
        let mut frames: Vec<Texture2D> = Vec::new();

        match fx_type {
            // FX 0: Scrolling
            0 => {
                let mut xs = e.ints[2];
                let mut ys = e.ints[3];

                if xs > w / 2 { xs = w / 2; }
                if xs < -(w / 2) { xs = -(w / 2); }
                if ys > h / 2 { ys = h / 2; }
                if ys < -(h / 2) { ys = -(h / 2); }

                let scale_speed = |val: i32| -> i32 {
                    match val {
                        3 => 2 * scale,
                        5 => 4 * scale,
                        7 | 9 | 10 | 11 | 12 => 8 * scale,
                        13 | 14 | 15 => 16 * scale,
                        -3 => -2 * scale,
                        -5 => -4 * scale,
                        -7 | -9 | -10 | -11 | -12 => -8 * scale,
                        -13 | -14 | -15 => -16 * scale,
                        _ => val,
                    }
                };

                xs = scale_speed(xs);
                ys = scale_speed(ys);

                if xs.abs() != 0 || ys.abs() != 0 {
                    let mut c = if xs.abs() != 0 { (w / xs.abs()) - 1 } else { 0 };
                    let cy = if ys.abs() != 0 { (h / ys.abs()) - 1 } else { 0 };

                    if cy > c { c = cy; }
                    if c > w - 1 { c = w - 1; }
                    if c < 0 { c = 0; }

                    let mut scrx = 0;
                    let mut scry = 0;

                    for _ in 0..=c {
                        let mut new_img = Image::gen_image_color(w as u16, h as u16, BLANK);

                        for x in 0..w {
                            for y in 0..h {
                                let col = src_img.get_pixel(x as u32, y as u32);

                                let wx = (x + scrx).rem_euclid(w);
                                let wy = (y + scry).rem_euclid(h);

                                new_img.set_pixel(wx as u32, wy as u32, col);
                            }
                        }

                        frames.push(Texture2D::from_image(&new_img));

                        scrx = (scrx + xs).rem_euclid(w);
                        scry = (scry + ys).rem_euclid(h);
                    }

                    self.effects.push(TileFx {
                        tile_index: tile_idx,
                        frames,
                        speed: 60.0 / 1000.0,
                        ..TileFx::default()
                    });
                    self.mapping[tile_idx] = self.effects.len() - 1;
                }
            }

            // FX 1, 2, 3: Distortion
            1 | 2 | 3 => {
                let intensity = match fx_type {
                    2 => 100,
                    3 => 500,
                    _ => 50,
                };

                for _ in 0..16 {
                    let mut new_img = src_img.clone();

                    for _ in 0..=intensity {
                        let mut x = rand::gen_range(0, w);
                        let mut y = rand::gen_range(0, h);

                        let col = src_img.get_pixel(x as u32, y as u32);

                        x += rand::gen_range(-scale, scale + 1);
                        y += rand::gen_range(-scale, scale + 1);

                        if scale < 2 {
                            x = x.rem_euclid(w);
                            y = y.rem_euclid(h);
                            new_img.set_pixel(x as u32, y as u32, col);
                        } else {
                            x = x.rem_euclid(w - 1);
                            y = y.rem_euclid(h - 1);

                            new_img.set_pixel(x as u32, y as u32, col);
                            new_img.set_pixel((x + 1) as u32, y as u32, col);
                            new_img.set_pixel((x + 1) as u32, (y + 1) as u32, col);
                            new_img.set_pixel(x as u32, (y + 1) as u32, col);
                        }
                    }

                    frames.push(Texture2D::from_image(&new_img));
                }

                self.effects.push(TileFx {
                    tile_index: tile_idx,
                    frames,
                    speed: 50.0 / 1000.0,
                    ..TileFx::default()
                });
                self.mapping[tile_idx] = self.effects.len() - 1;
            }

            // FX 4, 5, 6: Waves
            4 | 5 | 6 => {
                let intensity = match fx_type {
                    4 => 1,
                    5 => 2,
                    _ => 3,
                } * scale;

                for i in 0..30 {
                    let mut new_img = Image::gen_image_color(w as u16, h as u16, BLANK);

                    for x in 0..w {
                        for y in 0..h {
                            let deg_to_rad = std::f32::consts::PI / 180.0;

                            let y_offset = ((i * 12 + x * 11 / scale) as f32 * deg_to_rad).sin() * intensity as f32;
                            let x_offset = ((i * 12 + y * 11 / scale) as f32 * deg_to_rad).sin() * intensity as f32;

                            let src_x = (x as f32 + x_offset) as i32;
                            let src_y = (y as f32 + y_offset) as i32;

                            let wrap_x = src_x.rem_euclid(w);
                            let wrap_y = src_y.rem_euclid(h);

                            let col = src_img.get_pixel(wrap_x as u32, wrap_y as u32);
                            new_img.set_pixel(x as u32, y as u32, col);
                        }
                    }
                    frames.push(Texture2D::from_image(&new_img));
                }

                self.effects.push(TileFx {
                    tile_index: tile_idx,
                    frames,
                    speed: 80.0 / 1000.0,
                    ..TileFx::default()
                });
                self.mapping[tile_idx] = self.effects.len() - 1;
            }

            _ => {}
        }
    }

    pub fn update(&mut self, delta:f32) {
        for fx in &mut self.effects {
            fx.update_time += delta;
            if fx.update_time < fx.speed { continue; }
            fx.update_time -= fx.speed;
            if fx.frames.len() > 0 {
                fx.current_frame = (fx.current_frame + 1) % fx.frames.len();
            } else {
                fx.current_frame += 1;
                if fx.current_frame >= fx.animation_end {
                    fx.current_frame = fx.animation_start;
                }
            }
        }
    }
}