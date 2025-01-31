use macroquad::{
    color::Color,
    input::mouse_position,
    math::{IVec2, Rect},
    window::{screen_height, screen_width},
};

use crate::Context;

use super::card::{Card, CreatureCard, DisplayedCard};

/// Beginning of the hand on the screen
const HAND_START: f32 = 20.;

/// Represents a player's hand
pub struct Hand {
    /// Current cards in hand
    pub card_in_hands: Vec<DisplayedCard>,
    /// Selected card by the player, ready to be played. Index between -1 (represents none hovered) and the length of card_in_hands
    pub selected_card: i8,
    /// Hovered card by the cursor. Index between -1 (represents none hovered) and the length of card_in_hands
    pub hovered_card: i8,
    // The scale of hovered cards
    pub hovered_scale: f32,
    /// The dimension of the card (width: f32, heigth: f32)
    pub card_dimensions: (f32, f32),
    /// Screen size (width: f32, heigth: f32)
    pub screen_size: (f32, f32),
}

impl Hand {
    /// Create new hand
    ///
    /// * `card_dimensions` : width and heigth of the cards, (width: f32 , heigth: f32)
    /// * `hovered_scale` : the scale of the cards once hovered
    pub fn new(hovered_scale: f32, card_dimensions: (f32, f32)) -> Hand {
        Hand {
            card_in_hands: Vec::new(),
            selected_card: -1,
            hovered_card: -1,
            hovered_scale,
            card_dimensions,
            // screen_size: (screen_width(), screen_height()),
            screen_size: (0., 0.),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        // Both base_dimensions and neighbour_start are set to 0. because they will be replace adjusted automatically in update_card_to_screen
        let base_dimensions = Rect {
            x: 0.,
            y: 0.,
            w: 0.,
            h: 0.,
        };
        let neighbour_start = 0.;

        let new_card_to_display =
            DisplayedCard::new(card, self.hovered_scale, base_dimensions, neighbour_start);
        self.card_in_hands.push(new_card_to_display);
    }

    /// Return if a card is hovered
    pub fn card_is_hovered(&self) -> bool {
        self.hovered_card != -1
    }

    /// Return if a card is selected
    pub fn card_is_selected(&self) -> bool {
        self.selected_card != -1
    }

    /// Set the currently hovered card as the selected card
    ///
    /// If the card is already selected then it is not the selected card anymore.
    pub fn select_hovered_card(&mut self) {
        self.selected_card = if self.hovered_card == self.selected_card {
            -1
        } else {
            self.hovered_card
        };
    }

    /// Select the target of a given card
    ///
    /// TODO:
    /// Change the effect of the target based on the card type.
    /// For example, a creature card's target would be where the creature is summoned and a effect
    /// card's target would be the target of the spell which must be an entity (creature, player, etc.)
    // fn select_card_target(&self, ctx: &Context) {
    //     let mouse_in_world = ctx.camera.screen_to_world(mouse_position().into());
    // }
    /// Play a card that is hovered
    ///
    /// If no card is hovered, nothing is done without panicking
    pub async fn play_card(&mut self, creatures: &mut Vec<(CreatureCard, IVec2)>, pos: IVec2) {
        // let hovered_card = self.hovered_card;

        assert!(self.selected_card != -1);

        let card = self.card_in_hands.remove(self.selected_card as usize);
        match card.card {
            Card::Creature(mut creature_card) => {
                creature_card.load_texture().await;
                creatures.push((creature_card, pos));
            }
            Card::Effect(effect_card) => todo!(),
        }

        self.selected_card = -1;

        self.update_card_to_screen();
        // match self.hovered_card {
        //     (-1) => {}
        //     (_) => {}
        // }
    }

    /// Update the size and position of the screen to fit the screen size
    fn update_card_to_screen(&mut self) {
        let (w, h) = self.screen_size;
        let current_w = screen_width();
        let current_h = screen_height();

        // Check if the screen size has change to ensure that we do not execute
        // the following logic for nothing
        if w != current_w || h != current_h {
            self.card_dimensions = (current_w * 0.2, current_h * 0.3);
            let (new_w, new_h) = self.card_dimensions;

            let mut i = 1;
            for card in self.card_in_hands.iter_mut() {
                card.base_dimensions.x = (card.base_dimensions.w * 0.3) * i as f32;
                card.base_dimensions.y = current_h * 0.9;
                card.base_dimensions.w = new_w;
                card.base_dimensions.h = new_h;
                card.neighbour_start = (card.base_dimensions.w * 0.3) * (i + 1) as f32;
                i += 1;
            }

            // Update the last card to make it completely hoverable
            let last_card = self.card_in_hands.last_mut();
            match last_card {
                Some(c) => c.neighbour_start = c.base_dimensions.x + c.base_dimensions.w,
                None => (),
            }
        }
    }

    /// Display the card in hand to the player's screen
    ///
    /// `NOTE` Currently update the hovered card here to not have to make another
    /// method that would iterate over the cards again to find the hovered one.
    pub fn display_hand(&mut self, font_size: f32, font_color: Color) {
        let mut hovered_card: i8 = -1;

        self.update_card_to_screen();

        for (i, card) in self.card_in_hands.iter_mut().enumerate() {
            card.display_card(font_size, font_color, self.selected_card == i as i8);

            // Keeps the hovered card updated
            if card.scale > 1. {
                hovered_card = i as i8;
            }
        }
        self.hovered_card = hovered_card;
    }
}
