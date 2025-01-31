use macroquad::{
    camera::Camera2D,
    color::{GREEN, WHITE},
    input::mouse_position,
    math::{ivec2, mat2, vec2, IVec2, Vec2},
    texture::{draw_texture, Texture2D},
};
use macroquad_tiled::{Layer, Map};
// use macroquad_tiled::tiled::Map;

use crate::Context;

const TILE_SIZE: IVec2 = ivec2(32, 32);

pub struct World {
    // pub layer: Layer,
    // pub map_size: (u32, u32),
    // map: Map,
    // tiles_textures: Vec<Texture2D>,
    pub width: u32,
    pub heigth: u32,
    // pub data: Vec<Option<Tile>>,
}

impl World {
    pub fn new(/*map: Map, tiles_textures: Vec<Texture2D>*/ width: u32, heigth: u32) -> World {
        // let raw = map.raw_tiled_map;
        World {
            // map,
            // tiles_textures, // layer: raw.layers[0],
            width,
            heigth,
            // data,
        }
    }

    pub fn cursor_within_map(&self, camera: &Camera2D) -> bool {
        let mouse_in_world = camera.screen_to_world(mouse_position().into());
        let IVec2 { x, y } = world_to_map(mouse_in_world);

        x >= 0 && x <= self.width as i32 && y >= 0 && y <= self.heigth as i32
    }

    pub fn cursor_position(&self, camera: &Camera2D) -> IVec2 {
        let mouse_in_world = camera.screen_to_world(mouse_position().into());
        world_to_map(mouse_in_world)
    }

    // pub fn display_world(&self, ctx: Context) {
    //     // let layer = self.map.raw_tiled_map

    //     let layer = &self.map.layers["main layer"];
    //     let (w, h) = (layer.width, layer.height);
    //     let mouse_in_world = ctx.camera.screen_to_world(mouse_position().into());
    //     for y in 0..h {
    //         for x in 0..w {
    //             let texture_id = layer.data[y as usize * h as usize + x as usize]
    //                 .as_ref()
    //                 .unwrap()
    //                 .id as usize;
    //             let texture = &self.tiles_textures[texture_id];

    //             let world_pos = World::map_to_world(ivec2(x as i32, y as i32));

    //             // When hovering tile
    //             if ivec2(x as i32, y as i32) == World::world_to_map(mouse_in_world)
    //                 && ctx.hand.card_is_selected()
    //             {
    //                 draw_texture(&texture, world_pos.x - 0.8, world_pos.y - 0.8, GREEN);
    //             } else {
    //                 draw_texture(&texture, world_pos.x, world_pos.y, WHITE);
    //             };
    //         }
    //     }
    // }
}

// My thanks to the following PR https://github.com/not-fl3/macroquad/pull/598/commits/903333bea9747d490c360d9a1a91aa21f37ba379

// Transform world position to map position.
// Reference: https://youtu.be/04oQ2jOUjkU
pub fn world_to_map(world_pos: Vec2) -> IVec2 {
    let ihat = vec2(0.5, 0.25) * TILE_SIZE.as_vec2();
    let jhat = vec2(-0.5, 0.25) * TILE_SIZE.as_vec2();
    let inverse = mat2(ihat, jhat).inverse();

    inverse.mul_vec2(world_pos).as_ivec2()
}

// Transform map position to world position.
// Reference: https://youtu.be/04oQ2jOUjkU
pub fn map_to_world(map_pos: IVec2) -> Vec2 {
    let ihat = vec2(0.5, 0.25) * TILE_SIZE.as_vec2();
    let jhat = vec2(-0.5, 0.25) * TILE_SIZE.as_vec2();

    let transform = mat2(ihat, jhat);
    let offset = ivec2(-TILE_SIZE.x / 2, 0);

    transform.mul_vec2(map_pos.as_vec2()) + offset.as_vec2()
}
