use macroquad::{
    camera::Camera2D,
    input::mouse_position,
    math::{ivec2, mat2, vec2, IVec2, Vec2},
};

const TILE_SIZE: IVec2 = ivec2(32, 32);

/// Represent the game world
///
/// TODO add map data inside
pub struct World {
    pub width: u32,
    pub heigth: u32,
}

impl World {
    pub fn new(width: u32, heigth: u32) -> World {
        World { width, heigth }
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
