mod game;
use std::collections::HashMap;

use game::card::{Card, CardBasicInfo, EffectCard};
use game::effect::{Effect, EffectType};
use game::hand::Hand;
use game::keymapping::apply_input;

use game::card::CreatureCard;
use game::ui::*;
use game::world::{map_to_world, world_to_map, World};
use macroquad::{prelude::*, ui::root_ui};
use macroquad_tiled::{self as tiled};

// NOTE susceptible to change
// const TILE_SIZE: IVec2 = ivec2(32, 32);
const MAP_SIZE: IVec2 = ivec2(16, 16);
const NB_TILE_TYPE: usize = 115;

const CAM_SPEED: f32 = 10.;

// enum Entity {
//     Creature(CreatureCard),
//     Player(Player),
// }

struct Context {
    /// map_dimensions -> (width: u32, width: u32)
    pub world: World,
    pub camera: Camera2D,
    pub last_mouse_position: Vec2,
    pub hand: Hand,
    /// Vec of creatures and their positions
    pub creatures: Vec<(CreatureCard, IVec2)>,
}

fn format_digit(mut digit: usize, nb_displayed_digit: usize) -> String {
    if digit == 0 {
        return "000".to_string();
    }

    let default_digit = digit;
    let mut count = 0;
    while digit != 0 {
        digit = digit / 10;
        count += 1;
    }

    let mut zeros = "".to_string();
    for _ in 0..nb_displayed_digit - count {
        zeros.push('0');
    }

    return format!("{}{}", zeros, default_digit).to_string();
}
#[macroquad::main("CardGame")]
async fn main() {
    let game_name = "Funny Game";
    let mut text_color: Color;

    let mut settings = Settings::builder()
        .skin(HashMap::from([
            ("Default".to_string(), default_skin().await),
            ("Keymapping".to_string(), keymappings_skin().await),
        ]))
        .build()
        .await;

    set_pc_assets_folder("assets");

    let tileset = load_texture("spritesheet.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("map1.json").await.unwrap();
    let tiled_map = tiled::load_map(&tiled_map_json, &[("spritesheet.png", tileset)], &[]).unwrap();

    let mut tiles_textures: Vec<Texture2D> = Vec::with_capacity(NB_TILE_TYPE);

    // CAN BE OPTIMIZED
    for i in 0..NB_TILE_TYPE {
        let texture_name = format!("isometric_tileset/tile_{}.png", format_digit(i, 3));
        tiles_textures.push(load_texture(&texture_name).await.unwrap());
    }

    let layer = &tiled_map.layers["main layer"];
    // let layer = tiled_map.raw_tiled_map;

    let cam_area = vec2(32. * 24., 32. * 18.);
    // Assumption here is the world origin is 0, 0.
    let cam_pos = vec2(-cam_area.x / 2., -cam_area.y / 2.);
    let camera =
        Camera2D::from_display_rect(Rect::new(cam_pos.x, -cam_pos.y, cam_area.x, -cam_area.y));

    let mut ctx: Context = Context {
        world: World::new(layer.width, layer.height),
        camera: camera,
        last_mouse_position: mouse_position().into(),
        hand: Hand::new(1.4, (screen_width() * 0.2, screen_height() * 0.3)),
        creatures: Vec::new(),
    };

    let creature_card: Card = game::card::Card::Creature(CreatureCard::new(
        CardBasicInfo {
            name:"Goblin".to_string(), 
            description:"Vilest of creatures.\nHostile to all and detesable to it's very core.\nNo guilt must be felt when killing one.".to_string(),
            cost:1,
            // counter:None,
            card_color:BEIGE,
            },
        4,
        4,
        "creatures/goblin/goblin",
        2,
    ));

    let creature_card2: Card = game::card::Card::Creature(CreatureCard::new(
        CardBasicInfo {
            name: "Monkey Knight".to_string(),
            description: "Likes banana".to_string(),
            cost: 1,
            // counter: None,
            card_color: BEIGE,
        },
        4,
        4,
        "/creatures/monkey_knight/monkey_knight",
        4,
    ));

    let effect_card: Card = game::card::Card::Effect(EffectCard::new(
        CardBasicInfo {
            name: "Fire Ball".to_string(),
            description: "One of the most simple spell, yet a spell to be feared".to_string(),
            cost: 2,
            // counter: Some(0),
            card_color: RED,
        },
        Effect {
            effect_type: EffectType::Damage,
            nb: 4,
        },
    ));

    ctx.hand.add_card(creature_card);
    ctx.hand.add_card(effect_card);
    ctx.hand.add_card(creature_card2);

    loop {
        clear_background(GRAY);
        if settings.dark_theme {
            clear_background(BLACK);
            text_color = WHITE;
        } else {
            clear_background(LIGHTGRAY);
            text_color = BLACK;
        }

        // User input
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        apply_input(&mut ctx, &mut settings).await;

        // Draw tiles in camera perspective
        set_camera(&ctx.camera);
        draw_rectangle_lines(cam_pos.x, -cam_pos.y, cam_area.x, -cam_area.y, 2., RED);
        draw_text("Isometric map here", 0.0, 0.0, 30.0, text_color);

        let mouse_in_world = ctx.camera.screen_to_world(mouse_position().into());
        for y in 0..MAP_SIZE.y {
            for x in 0..MAP_SIZE.x {
                let texture_id = layer.data[y as usize * MAP_SIZE.y as usize + x as usize]
                    .as_ref()
                    .unwrap()
                    .id as usize;
                let texture = &tiles_textures[texture_id];

                let world_pos = map_to_world(ivec2(x, y));

                // When hovering tile
                if ivec2(x, y) == world_to_map(mouse_in_world) && ctx.hand.card_is_selected() {
                    draw_texture(&texture, world_pos.x - 0.8, world_pos.y - 0.8, GREEN);
                } else {
                    draw_texture(&texture, world_pos.x, world_pos.y, WHITE);
                };
            }
        }

        // Draw creatures
        for (creature, pos) in &mut ctx.creatures {
            let Vec2 { x, y } = map_to_world(*pos);
            // -16 because the tile heigth are 16 and those of the creature img are 32
            creature.draw_creature(x, y - 16., WHITE);
        }

        // 2D context
        set_default_camera();
        draw_text(game_name, 10.0, 20.0, 30.0, text_color);

        // Hand
        ctx.hand.display_hand(16., text_color);

        // Buttons
        let (_, skin) = settings.skin.get_key_value(&"Default".to_string()).unwrap();
        root_ui().push_skin(skin);
        if root_ui().button(vec2(screen_width() - 80., 20.), "Settings  ") {
            settings.toggle_display_settings();
        }

        if root_ui().button(vec2(screen_width() - 80., 40.), "Keymapping") {
            settings.toggle_display_keymapping();
        }

        root_ui().pop_skin();

        #[cfg_attr(any(), rustfmt::skip)]
        { // Display settings related informations
        if settings.display_settings   { show_settings(&mut settings); }
        if settings.display_keymapping { show_keymapping(&mut settings); }
        if settings.debug { show_debug_info(&ctx, &settings, text_color); }
        }

        next_frame().await
    }
}
