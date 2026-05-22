use macroquad::color::Color;
use macroquad::prelude::{Image, Texture2D, Vec2};

/// Merges a base tilemap and individual tile textures into a single Power-of-Two (POT) square Texture2D.
///
/// # Arguments
/// * `base_texture` - The starting texture containing an existing grid of tiles.
/// * `tile_size` - The dimensions (width, height) of a single tile.
/// * `extra_textures` - A slice of additional textures to append. Must match `tile_size`.
pub fn merge_textures_to_atlas(
    base_texture: &Texture2D,
    tile_size: Vec2,
    extra_textures: &[Texture2D],
) -> Texture2D {
    let tile_w = tile_size.x as u32;
    let tile_h = tile_size.y as u32;

    let base_image = base_texture.get_texture_data();
    let base_cols = base_image.width as u32 / tile_w;
    let base_rows = base_image.height as u32 / tile_h;
    let base_tiles_count = base_cols * base_rows;

    let total_tiles = base_tiles_count + extra_textures.len() as u32;

    let mut atlas_size = tile_w.max(tile_h).next_power_of_two();
    loop {
        let cols = atlas_size / tile_w;
        let rows = atlas_size / tile_h;
        if cols * rows >= total_tiles {
            break;
        }
        atlas_size *= 2;
    }

    let mut atlas_image = Image::gen_image_color(
        atlas_size as u16,
        atlas_size as u16,
        Color::new(0.0, 0.0, 0.0, 0.0),
    );

    let atlas_cols = atlas_size / tile_w;
    let mut current_tile_idx = 0;

    let mut copy_tile = |src: &Image, src_col: u32, src_row: u32| {
        let dest_col = current_tile_idx % atlas_cols;
        let dest_row = current_tile_idx / atlas_cols;

        let dest_x_offset = dest_col * tile_w;
        let dest_y_offset = dest_row * tile_h;
        let src_x_offset = src_col * tile_w;
        let src_y_offset = src_row * tile_h;

        for y in 0..tile_h {
            for x in 0..tile_w {
                let color = src.get_pixel(src_x_offset + x, src_y_offset + y);
                atlas_image.set_pixel(dest_x_offset + x, dest_y_offset + y, color);
            }
        }
        current_tile_idx += 1;
    };

    for row in 0..base_rows {
        for col in 0..base_cols {
            copy_tile(&base_image, col, row);
        }
    }

    for tex in extra_textures {
        let img = tex.get_texture_data();

        debug_assert_eq!(img.width as u32, tile_w, "Extra texture width mismatch");
        debug_assert_eq!(img.height as u32, tile_h, "Extra texture height mismatch");

        copy_tile(&img, 0, 0);
    }

    Texture2D::from_image(&atlas_image)
}