use macroquad::{
    color::{Color, BLACK, BLUE, GREEN},
    input::mouse_position,
    shapes::{draw_poly, draw_rectangle, draw_rectangle_lines},
    text::{draw_multiline_text, draw_text},
};

use super::effect::Effect;

// TODO: complete overtime

pub enum Card {
    Creature(CreatureCard),
    Effect(EffectCard),
}

pub struct CardBasicInfo {
    pub name: String, // name of the card
    // image_front: Path,   // Path to the image on the front of the card
    // image_back: Path,    // Path to the image on the back of the card
    pub description: String,  // description of the card
    pub cost: u32,            // cost of the card
    pub counter: Option<u32>, // counter before card take effects
    pub card_color: Color,
}

// Creature Card
pub struct CreatureCard {
    pub basic_info: CardBasicInfo,
    pub current_hp: u32,
    pub hp_max: u32,
    pub movement: u32, // Total movement the creature can do
}

impl CreatureCard {
    pub fn new(basic_info: CardBasicInfo, hp_max: u32, movement: u32) -> Self {
        CreatureCard {
            basic_info,
            current_hp: hp_max,
            hp_max,
            movement,
        }
    }
}

// Card with effect
pub struct EffectCard {
    pub basic_info: CardBasicInfo,
    pub effect: Effect,
}

impl EffectCard {
    fn new(basic_info: CardBasicInfo, effect: Effect) -> Self {
        EffectCard { basic_info, effect }
    }
}

impl Card {
    pub fn get_name(&self) -> &str {
        match self {
            Card::Creature(creature) => &creature.basic_info.name,
            Card::Effect(effect) => &effect.basic_info.name,
        }
    }

    fn card_is_hovered(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        let (mouse_pos_x, mouse_pos_y) = mouse_position();
        return (mouse_pos_x >= x && mouse_pos_x <= x + w)
            && (mouse_pos_y >= y && mouse_pos_y <= y + h);
    }

    pub fn draw_card(
        &self,
        mut x: f32,
        mut y: f32,
        mut w: f32,
        mut h: f32,
        font_size: f32,
        font_color: Color,
    ) {
        match self {
            Card::Creature(creature) => {
                if self.card_is_hovered(x, y, w, h) {
                    // Card size
                    x = x * 0.7;
                    y = y * 0.7;

                    w = w * 1.3;
                    h = h * 1.3;

                    // background
                    draw_rectangle(x, y, w, h, creature.basic_info.card_color);
                    // outline
                    draw_rectangle_lines(x, y, w, h, 8., BLUE);
                } else {
                    // background
                    draw_rectangle(x, y, w, h, creature.basic_info.card_color);
                    // outline
                    draw_rectangle_lines(x, y, w, h, 2., BLACK);
                }

                // Name
                draw_text(
                    &creature.basic_info.name,
                    x + font_size * 0.5,
                    y + font_size,
                    font_size,
                    font_color,
                );

                // Cost
                draw_poly(x + w - 17., y + font_size - 3., 8, 10., 0., BLUE);
                draw_text(
                    &format!("{}", creature.basic_info.cost),
                    x + w - 20.,
                    y + font_size,
                    font_size,
                    font_color,
                );

                // Picture TODO complete
                draw_rectangle(x + 6., y + 24., w - 12., h / 3., GREEN);

                // HP
                draw_text(
                    &format!("HP: {}/{}", creature.current_hp, creature.hp_max),
                    x + font_size * 0.5,
                    y + h * 0.5,
                    font_size,
                    font_color,
                );

                // Description
                draw_multiline_text(
                    &creature.basic_info.description,
                    x + font_size * 0.5,
                    y + h * 0.6,
                    font_size * 0.6,
                    Some(2.),
                    BLACK,
                );
            }
            Card::Effect(effect) => {}
        }
    }
}
