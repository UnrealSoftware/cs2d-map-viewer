use std::cell::RefCell;
use macroquad::prelude::*;
use crate::assets::assets::Assets;
use crate::audio::AudioPlayer;
use crate::map::map::Map;
use crate::map::reader::read_map_file;
use crate::paths::{PATH_MAPS};

// fake use of image lib to support bmp without marking lib as unused
#[allow(unused_imports)]
use image as _;
use macroquad::hash;
use macroquad::ui::{root_ui, widgets};
use crate::settings::Settings;

mod audio;
mod util;
mod map;
mod assets;
mod materials;
mod paths;
mod settings;

const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");

const TILE_SIZE: f32 = 32.0;
const MAP_WIDTH: i32 = 50;
const MAP_HEIGHT: i32 = 50;
const MOVE_SPEED: f32 = 500.0;

const GAME_WIDTH: f32 = 1920.0;
const GAME_HEIGHT: f32 = 1080.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "CS2d Map Viewer".to_owned(),
        platform: miniquad::conf::Platform {
            // Some(0) disables VSync
            // Some(1) enables VSync
            // Some(-1) enables Adaptive VSync
            swap_interval: Some(1),
            ..Default::default()
        },
        ..Default::default()
    }
}

thread_local! {
    pub static SETTINGS: RefCell<Settings> = RefCell::new(Settings {
        grid: false,
    });
}

#[macroquad::main(window_conf)]
async fn main() {
    let _ = util::params::APP_PARAMS.set(util::params::get_params());
    for (key, value) in util::params::APP_PARAMS.get().unwrap() {
        info!("Param: {}={}", key, value);
    }

    let mut world_target = vec2(
        MAP_WIDTH as f32 * TILE_SIZE / 2.0,
        MAP_HEIGHT as f32 * TILE_SIZE / 2.0
    );

    let assets_path = format!("assets.zip?v={}", BUILD_TIMESTAMP);
    let mut assets = Assets::init(&assets_path).await;
    let mut map = Map::default();
    let mut did_load_map = false;

    // Try to load specified map from UnrealSoftware.de file archive
    let load_file = util::params::get_app_param_string("file", "");
    let load_file_cid = util::params::get_app_param_string("cid", "");
    if load_file.len() > 0 && load_file_cid.len() > 0 {
        let url = "https://www.unrealsoftware.de/get.php?get={f}&p=2&cid={cid}"
            .replace("{f}", &load_file)
            .replace("{cid}", &load_file_cid);
        match assets.loader.load_zip(&url, true).await {
           Ok(loaded_files) => {
               for map_file in loaded_files.iter().filter(|file| file.ends_with(".map")) {
                   did_load_map = read_map_file(map_file, &mut map, &mut assets).await.is_ok();
                   break;
               }
           }
           Err(e) => {
               error!("Failed to load zip '{}': {}", &url, e);
           }
       }
    }

    // Just load de_dust2 if no map could be loaded from zip
    if !did_load_map {
        let mut map_path = String::from(PATH_MAPS);
        map_path.push_str("de_dust2.map");
        read_map_file(&map_path, &mut map, &mut assets).await.unwrap();
    }

    let rt: Option<RenderTarget> = if cfg!(feature = "r2tex") {
        Some(render_target(GAME_WIDTH as u32, GAME_HEIGHT as u32))
    } else {
        None
    };
    let render_y_base = if cfg!(feature = "r2tex") { -2.0 } else { 2.0 };

    //let mut audio = AudioPlayer::new();
    //audio.play_file("unrealsoftware.wav", 0.5, [-50.0, 0.0], true);
    //let mem = load_file("unrealsoftware.wav").await.unwrap();
    //audio.play_memory(&*mem, 0.5, [-50.0, 0.0], true);

    let mut last_pointer_pos = (0.0, 0.0);
    let mut was_pointer_down = false;

    loop {
        let delta = get_frame_time();

        let mut is_pointer_over_ui = false;

        /*
        egui_macroquad::ui(|egui_ctx| {
            let scale = (screen_width() / GAME_WIDTH).min(screen_height() / GAME_HEIGHT);
            egui_ctx.set_pixels_per_point(scale);

            egui::Window::new("CS2D Map Viewer")
                .show(egui_ctx, |ui| {
                    ui.label("Test");
                });

            is_pointer_over_ui = egui_ctx.wants_pointer_input() || egui_ctx.is_pointer_over_area();
        });
         */

        if !is_pointer_over_ui {
            let mut speed = if is_key_down(KeyCode::LeftShift) { 5.0 } else { 1.0 };
            if is_key_down(KeyCode::LeftAlt) { speed *= 0.1; }
            speed *= MOVE_SPEED * delta;
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) { world_target.y -= speed; }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) { world_target.y += speed; }
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) { world_target.x -= speed; }
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) { world_target.x += speed; }

            if is_key_released(KeyCode::G) {
                SETTINGS.with(|s| {
                    let mut settings = s.borrow_mut();
                    settings.grid = !settings.grid;
                });
            }

            let current_pos: (f32, f32);
            let pointer_down: bool;
            let active_touches = touches();

            if !active_touches.is_empty() {
                let touch = &active_touches[0];
                current_pos = (touch.position.x, touch.position.y);
                pointer_down = touch.phase != TouchPhase::Ended && touch.phase != TouchPhase::Cancelled;
            } else {
                current_pos = mouse_position();
                pointer_down = is_mouse_button_down(MouseButton::Left);
            }

            let delta_x = current_pos.0 - last_pointer_pos.0;
            let delta_y = current_pos.1 - last_pointer_pos.1;

            if pointer_down && was_pointer_down {
                let screen_scale = (screen_width() / GAME_WIDTH).min(screen_height() / GAME_HEIGHT);
                world_target.x -= delta_x / screen_scale;
                world_target.y -= delta_y / screen_scale;
            }

            last_pointer_pos = current_pos;
            was_pointer_down = pointer_down;
        }

        assets.materials.use_default();

        let cam = Camera2D {
            render_target: rt.clone(),
            target: vec2(world_target.x.floor(), world_target.y.floor()),
            zoom: vec2(2.0 / GAME_WIDTH, render_y_base / GAME_HEIGHT),
            ..Default::default()
        };
        set_camera(&cam);

        let top_left = cam.screen_to_world(vec2(0.0, 0.0));
        let rect = Rect::new(top_left.x, top_left.y, GAME_WIDTH, GAME_HEIGHT);

        map.tile_fx.update(delta);

        // Draw Level 0 - Background
        map.background.draw(delta, rect);
        // todo: particles level 0
        // todo: Tdo.draw_reset & Tdo.draw_background

        // Draw Level 1 - Water
        map.draw(rect, 1);
        // todo: particles level 1

        // Draw Level 2 - Ground
        map.draw(rect, 2);
        map.draw_entities(delta, rect, &assets, 0);
        // todo: particles level 2
        // todo: Tdo.draw_ground
        // todo: Tpro.draw_ground(0)

        // Draw Level 3 - Items / Obstacles / Shadows
        // todo: Titem.draw
        map.draw(rect, 3);
        // todo: Tdo.draw_obstacle
        // todo: Tpro.draw_ground(1)
        map.draw_entities(delta, rect, &assets, 1);
        map.draw_shadows(rect, &assets);
        // todo: particles level 3

        // Draw Level 4 - Hostages / Players / Flying Projectiles
        // todo: hostages
        // todo: players
        // todo: Tpro.draw_flying
        // todo: muzzles
        // todo: tsparkle
        // todo: particles level 4

        // Draw Level 5 - Walls / Entities
        map.draw(rect, 4);
        // todo: Tdo.draw_wall
        map.draw_entities(delta, rect, &assets, 2);
        // todo: Tdo.draw_overwall
        // todo: particles level 5
        // todo: particles level 7 (?!?!)
        // todo: particles level 6
        // todo: smart light layer 2
        // todo: particles level 8

        // todo: fow
        // todo: night vision overlay


        if SETTINGS.with(|s| s.borrow().grid) {
            map.draw_grid(rect, &assets);
        }

        // UI

        set_default_camera();

        if cfg!(feature = "r2tex") {
            if let Some(target) = &rt {
                clear_background(BLACK);

                let scale = (screen_width() / GAME_WIDTH).min(screen_height() / GAME_HEIGHT);
                let w = GAME_WIDTH * scale;
                let h = GAME_HEIGHT * scale;
                let x = (screen_width() - w) / 2.0;
                let y = (screen_height() - h) / 2.0;

                draw_texture_ex(
                    &target.texture,
                    x, y, WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(w, h)),
                        flip_y: true,
                        ..Default::default()
                    },
                );
            }
        }

        let fps_text = format!("{}", get_fps());
        let text_dimensions = measure_text(&fps_text, None, 20, 1.0);
        draw_text(
            &fps_text,
            screen_width() - text_dimensions.width - 10.0,
            10.0 + text_dimensions.offset_y,
            20.0,
            GREEN
        );

        egui_macroquad::draw();

        next_frame().await
    }
}