use macroquad::{color::Color, math::Rect, window::screen_height};

use super::card::{Card, DisplayedCard};

/// Beginning of the hand on the screen
const HAND_START: f32 = 20.;

/// Represents a player's hand
pub struct Hand {
    /// Current cards in hand
    pub card_in_hands: Vec<DisplayedCard>,
    /// index between -1 (represents none hovered) and the length of card_in_hands
    pub hovered_card: i8,
    // The scale of hovered cards
    pub hovered_scale: f32,
    /// The dimension of the card (width: f32, heigth: f32)
    pub card_dimensions: (f32, f32),
}

impl Hand {
    /// Create new hand
    ///
    /// * `card_dimensions` : width and heigth of the cards, (width: f32 , heigth: f32)
    /// * `hovered_scale` : the scale of the cards once hovered
    pub fn new(card_dimensions: (f32, f32), hovered_scale: f32) -> Hand {
        Hand {
            card_in_hands: Vec::new(),
            hovered_card: -1,
            hovered_scale,
            card_dimensions,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        let nb_card = self.card_in_hands.len();
        let previous_card = self.card_in_hands.last_mut();
        let (w, h) = self.card_dimensions;

        let (base_dimensions, neighbour_start) = match previous_card {
            Some(prev_card) => {
                let dimensions = Rect {
                    x: HAND_START + (HAND_START * 3.) * (nb_card as f32),
                    y: screen_height() * 0.9,
                    w,
                    h,
                };
                prev_card.neighbour_start = dimensions.x;
                (dimensions, dimensions.x + dimensions.w)
            }
            None => {
                // First card added
                let dimensions = Rect {
                    x: HAND_START,
                    y: screen_height() * 0.9,
                    w,
                    h,
                };
                (dimensions, dimensions.x + dimensions.w)
            }
        };

        let new_card_to_display =
            DisplayedCard::new(card, self.hovered_scale, base_dimensions, neighbour_start);
        self.card_in_hands.push(new_card_to_display);
    }

    /// Play a card that is hovered
    ///
    /// If no card is hovered, nothing is done without panicking
    pub fn play_card(&self) {
        !todo!()
    }

    /// Display the card in hand to the player's screen
    ///
    /// `NOTE` Currently update the hovered card here to not have to make another
    /// method that would iterate over the cards again to find the hovered one.
    pub fn display_hand(&mut self, font_size: f32, font_color: Color) {
        let mut hovered_card = -1;
        let mut i = 0;

        for card in self.card_in_hands.iter_mut() {
            card.display_card(font_size, font_color);

            // Keeps the hovered card updated
            if card.scale > 1. {
                hovered_card = i;
            }

            i += 1;
        }
        self.hovered_card = hovered_card;
    }
}
