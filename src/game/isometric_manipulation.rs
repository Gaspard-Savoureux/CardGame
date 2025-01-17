// My thanks to the following PR https://github.com/not-fl3/macroquad/pull/598/commits/903333bea9747d490c360d9a1a91aa21f37ba379

use macroquad::math::{ivec2, mat2, vec2, IVec2, Vec2};

use crate::TILE_SIZE;

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
