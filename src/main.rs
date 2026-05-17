use macroquad::prelude::*;
use crate::assets::assets::Assets;
use crate::audio::AudioPlayer;
use crate::map::map::Map;
use crate::map::reader::read_map_file;
use crate::paths::{PATH_MAPS};

// fake use of image lib to support bmp without marking lib as unused
#[allow(unused_imports)]
use image as _;

mod audio;
mod util;
mod map;
mod assets;
mod materials;
mod paths;

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

    let mut assets = Assets::init("assets.zip").await;

    let mut map = Map::default();
    let mut map_path = String::from(PATH_MAPS);
    map_path.push_str("de_dust2.map");
    read_map_file(&map_path, &mut map, &mut assets).await.unwrap();

    let render_target = render_target(GAME_WIDTH as u32, GAME_HEIGHT as u32);
    //render_target.texture.set_filter(FilterMode::Nearest);

    //let mut audio = AudioPlayer::new();
    //audio.play_file("unrealsoftware.wav", 0.5, [-50.0, 0.0], true);
    //let mem = load_file("unrealsoftware.wav").await.unwrap();
    //audio.play_memory(&*mem, 0.5, [-50.0, 0.0], true);

    loop {
        let delta = get_frame_time();

        let mut speed = if is_key_down(KeyCode::LeftShift) { 5.0 } else { 1.0 };
        if is_key_down(KeyCode::LeftAlt) { speed *= 0.1; }
        speed *= MOVE_SPEED * delta;
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) { world_target.y -= speed; }
        if is_key_down(KeyCode::Down)  || is_key_down(KeyCode::S) { world_target.y += speed; }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) { world_target.x -= speed; }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) { world_target.x += speed; }

        let screen_scale = (screen_width() / GAME_WIDTH).min(screen_height() / GAME_HEIGHT);
        let mouse_delta = mouse_delta_position();

        if is_mouse_button_down(MouseButton::Left) {
            world_target.x += mouse_delta.x * delta * MOVE_SPEED * 32.0 * screen_scale;
            world_target.y += mouse_delta.y * delta * MOVE_SPEED * 32.0 * screen_scale;
        }

        let cam = Camera2D {
            render_target: Some(render_target.clone()),
            target: vec2(world_target.x.floor(), world_target.y.floor()),
            zoom: vec2(2.0 / GAME_WIDTH, -2.0 / GAME_HEIGHT),
            ..Default::default()
        };
        set_camera(&cam);

        map.background.draw(&cam, delta);
        map.draw(&assets, 0);
        map.draw_shadows(&assets);
        map.draw_entities(&assets);

        // UI

        set_default_camera();
        clear_background(BLACK);

        let scale = (screen_width() / GAME_WIDTH).min(screen_height() / GAME_HEIGHT);
        let w = GAME_WIDTH * scale;
        let h = GAME_HEIGHT * scale;
        let x = (screen_width() - w) / 2.0;
        let y = (screen_height() - h) / 2.0;

        draw_texture_ex(
            &render_target.texture,
            x, y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                flip_y: true,
                ..Default::default()
            },
        );

        draw_text("Use Arrow Keys to Scroll", 10.0, 10.0, 20.0, WHITE);

        let fps_text = format!("FPS: {}", get_fps());
        let text_dimensions = measure_text(&fps_text, None, 20, 1.0);
        draw_text(
            &fps_text,
            screen_width() - text_dimensions.width - 10.0,
            20.0 + text_dimensions.offset_y,
            20.0,
            GREEN
        );

        next_frame().await
    }
}