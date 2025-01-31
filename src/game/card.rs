use macroquad::{
    color::{Color, BLACK, BLUE, GREEN, LIME, WHITE},
    input::mouse_position,
    math::{vec2, Rect, Vec2},
    shapes::{draw_poly, draw_rectangle, draw_rectangle_lines},
    text::{draw_multiline_text, draw_text},
    texture::{draw_texture, draw_texture_ex, load_texture, DrawTextureParams, Texture2D},
    time::get_time,
};

use super::{effect::Effect, life::HasLife};

pub enum Card {
    Creature(CreatureCard),
    Effect(EffectCard),
}

pub struct CardBasicInfo {
    pub name: String, // name of the card
    // image_front: Path,   // Path to the image on the front of the card
    // image_back: Path,    // Path to the image on the back of the card
    pub description: String, // description of the card
    pub cost: u32,           // cost of the card
    // pub counter: Option<u32>, // counter before card take effects
    pub card_color: Color,
    // pub position: Vec2,
    // pub size: Vec2,
}

/// Creature Card
pub struct CreatureCard {
    pub basic_info: CardBasicInfo,
    pub hp_current: u32,
    pub hp_max: u32,
    /// Total movement the creature can do
    pub movement: u32,
    /// TODO the following will need to have its own struct for animations
    img_path: &'static str,
    nb_animation_frame: usize,
    current_animation_frame: usize,
    picture: Option<Texture2D>,
    animation: Vec<Texture2D>,
    animation_time_per_frame: f64,
    time_elapsed: f64,
}

impl CreatureCard {
    pub fn new(
        basic_info: CardBasicInfo,
        hp_max: u32,
        movement: u32,
        img_path: &'static str,
        nb_animation_frame: usize, // TODO will added automaticly based on the number of file with the name in creature assets
        animation_time_per_frame: f64,
    ) -> Self {
        CreatureCard {
            basic_info,
            hp_current: hp_max,
            hp_max,
            movement,
            img_path,
            nb_animation_frame,
            current_animation_frame: 0,
            picture: None,
            animation: Vec::new(),
            animation_time_per_frame,
            time_elapsed: 0.,
        }
    }

    pub async fn load_texture(&mut self) {
        for i in 0..self.nb_animation_frame {
            self.animation.push(
                load_texture(&format!("{}-{}.png", &self.img_path, i))
                    .await
                    .unwrap(),
            );
        }
        match load_texture(&format!("{}-cover.png", &self.img_path)).await {
            Ok(picture_texture) => self.picture = Some(picture_texture),
            Err(_) => {
                self.picture = Some(
                    load_texture("creatures/default/default-cover.png")
                        .await
                        .unwrap(),
                )
            }
        }
    }

    fn draw_picture(&self, x: f32, y: f32, size: Vec2, color: Color) {
        draw_texture_ex(
            &self.picture.clone().unwrap(),
            x,
            y,
            color,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
    }
    pub fn draw_creature(&mut self, x: f32, y: f32, color: Color) {
        let time = get_time();
        let diff = time - self.time_elapsed;

        if diff >= self.animation_time_per_frame {
            self.current_animation_frame =
                (self.current_animation_frame + 1) % self.nb_animation_frame;
            self.time_elapsed = get_time();
        }

        draw_texture(
            &self.animation.get(self.current_animation_frame).unwrap(),
            x,
            y,
            color,
        );
    }
}

impl HasLife for CreatureCard {
    fn is_alive(&self) -> bool {
        self.hp_current > 0
    }

    fn get_life_current(&self) -> u32 {
        self.hp_current
    }

    fn get_life_max(&self) -> u32 {
        self.hp_max
    }

    fn set_life(&mut self, delta: u32) {
        self.hp_current = delta;
    }
}

/// Card with effect
pub struct EffectCard {
    pub basic_info: CardBasicInfo,
    pub effect: Effect,
}

impl EffectCard {
    pub fn new(basic_info: CardBasicInfo, effect: Effect) -> Self {
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

    /// Load the textures of a card
    pub async fn load_texture(&mut self) {
        match self {
            Card::Creature(creature_card) => creature_card.load_texture().await,
            Card::Effect(effect_card) => {}
        }
    }

    fn get_basic_info(&self) -> &CardBasicInfo {
        match self {
            Card::Creature(creature) => &creature.basic_info,
            Card::Effect(effect) => &effect.basic_info,
        }
    }

    pub fn is_hovered(&self, dimensions: Rect) -> bool {
        let Rect { x, y, w, h } = dimensions;
        let (mouse_pos_x, mouse_pos_y) = mouse_position();

        return (mouse_pos_x >= x && mouse_pos_x <= x + w)
            && (mouse_pos_y >= y && mouse_pos_y <= y + h);
    }

    pub fn draw_card(&mut self, card: Rect, font_size: f32, font_color: Color) -> Rect {
        let basic_info = self.get_basic_info();
        let Rect { x, y, w, h } = card;

        // Name
        draw_text(
            &basic_info.name,
            x + font_size * 0.5,
            y + font_size,
            font_size,
            font_color,
        );

        // Cost
        draw_poly(x + w - 17., y + font_size - 3., 8, 10., 0., BLUE);
        draw_text(
            &format!("{}", basic_info.cost),
            x + w - 20.,
            y + font_size,
            font_size,
            font_color,
        );

        match self {
            Card::Creature(creature) => {
                // Cover Picture
                match creature.picture {
                    Some(_) => {
                        creature.draw_picture(x + 6., y + 24., vec2(w - 12., h / 3.), WHITE);
                        draw_rectangle_lines(x + 6., y + 24., w - 12., h / 3., 2., BLACK);
                    }
                    None => draw_rectangle(x + 6., y + 24., w - 12., h / 3., GREEN),
                };

                // HP
                draw_text(
                    &format!("HP: {}/{}", creature.hp_current, creature.hp_max),
                    x + font_size * 0.5,
                    y + h * 0.5,
                    font_size,
                    font_color,
                );

                // Range
                draw_text(
                    &format!("Mobility: {} steps", creature.movement),
                    x + font_size * 0.5,
                    y + h * 0.6,
                    font_size,
                    font_color,
                );

                // Description
                draw_multiline_text(
                    &creature.basic_info.description,
                    x + font_size * 0.5,
                    y + h * 0.7,
                    font_size * 0.6,
                    Some(2.),
                    BLACK,
                );
            }
            Card::Effect(effect) => {
                // Cover Picture
                // TODO

                // Effect
                draw_text(
                    "Effect: TODO",
                    x + font_size * 0.5,
                    y + h * 0.5,
                    font_size,
                    font_color,
                );

                // Description
                draw_multiline_text(
                    &effect.basic_info.description,
                    x + font_size * 0.5,
                    y + h * 0.6,
                    font_size * 0.6,
                    Some(2.),
                    BLACK,
                );
            }
        }

        return Rect { x, y, w, h };
    }
}

/// Struct containing a card and its information allowing us to display it.
pub struct DisplayedCard {
    pub card: Card,
    pub scale: f32, // Default 1.
    pub hovered_scale: f32,
    pub base_dimensions: Rect,    // x, y, w, h
    pub current_dimensions: Rect, // x, y, w, h
    pub neighbour_start: f32, // Start of the next card, will allow us to not hover several card at once
                              // pub hovered: bool (could allow us to know which card is hovered and therefore playable later on)
                              // pub selected: bool
}

impl DisplayedCard {
    pub fn new(
        card: Card,
        hovered_scale: f32,
        base_dimensions: Rect,
        neighbour_start: f32,
    ) -> DisplayedCard {
        DisplayedCard {
            card,
            scale: 1.,
            hovered_scale,
            base_dimensions,
            current_dimensions: base_dimensions,
            neighbour_start,
        }
    }

    // TODO move to utils
    #[cfg_attr(any(), rustfmt::skip)]
    fn min (a: f32, b: f32) -> f32 { if a > b { b } else { a }}
    #[cfg_attr(any(), rustfmt::skip)]
    fn max (a: f32, b: f32) -> f32 { if a > b { a } else { b }}

    fn increase_scale(&mut self, speed: f32) {
        self.scale += DisplayedCard::min(
            (self.hovered_scale - self.scale) * speed, // Ease in effect
            self.hovered_scale,
        )
    }

    fn decrease_scale(&mut self, speed: f32) {
        self.scale = DisplayedCard::max(self.scale - speed, 1.);
    }

    pub fn display_card(&mut self, font_size: f32, font_color: Color, is_selected: bool) {
        let scale = self.scale;
        let card = &self.card;

        let x = self.base_dimensions.x;
        let mut y = self.base_dimensions.y - (self.current_dimensions.h - self.base_dimensions.h);
        let w = self.base_dimensions.w * scale;
        let h = self.base_dimensions.h * scale;
        self.current_dimensions.w = w;
        self.current_dimensions.h = h;

        let mut hoverable_surface = self.base_dimensions;
        hoverable_surface.w = self.neighbour_start - self.base_dimensions.x;

        let card_color = card.get_basic_info().card_color;

        // Draw the zone in which the cursor must be in order to highligth the card
        draw_rectangle_lines(
            hoverable_surface.x,
            hoverable_surface.y,
            hoverable_surface.w,
            hoverable_surface.h,
            2.,
            card_color,
        );

        if self.card.is_hovered(hoverable_surface) {
            y = y * 0.6; // Move the card upward to not overlap other cards
            draw_rectangle(x, y, w, h, card.get_basic_info().card_color); // background
                                                                          // draw_rectangle_lines(x, y, w, h, 8., if is_selected { GREEN } else { BLUE }); // outline

            // outline
            // draw_rectangle_lines(x, y, w, h, 8., BLUE);
            if is_selected {
                draw_rectangle_lines(x, y, w, h, 8., LIME);
            } else {
                draw_rectangle_lines(x, y, w, h, 8., BLUE);
            }
            self.increase_scale(0.2);
        } else {
            draw_rectangle(x, y, w, h, card.get_basic_info().card_color); // background

            // outline
            if is_selected {
                draw_rectangle_lines(x, y, w, h, 8., LIME);
            } else {
                draw_rectangle_lines(x, y, w, h, 2., BLACK);
            }

            self.decrease_scale(0.5);
        }

        self.card
            .draw_card(Rect { x, y, w, h }, font_size, font_color);
    }
}
