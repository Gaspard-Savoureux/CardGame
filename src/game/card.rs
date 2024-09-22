use super::effect::Effect;

// TODO: complete overtime

enum Card {
    Creature(CreatureCard),
    Effect(EffectCard),
}

struct CardBasicInfo {
    name: String, // name of the card
    // image_front: Path,   // Path to the image on the front of the card
    // image_back: Path,    // Path to the image on the back of the card
    description: String, // description of the card
    cost: u32,           // cost of the card
    counter: u32,        // counter before card take effects
}

// Creature Card
pub struct CreatureCard {
    basic_info: CardBasicInfo,
    hp_max: u32,
    movement: u32, // Total movement the creature can do
}

impl CreatureCard {
    fn new(basic_info: CardBasicInfo, hp_max: u32, movement: u32) -> Self {
        CreatureCard {
            basic_info,
            hp_max,
            movement,
        }
    }
}

// Card with effect
pub struct EffectCard {
    basic_info: CardBasicInfo,
    effect: Effect,
}

impl EffectCard {
    fn new(basic_info: CardBasicInfo, effect: Effect) -> Self {
        EffectCard { basic_info, effect }
    }
}

impl Card {
    fn get_name(&self) -> &str {
        match self {
            Card::Creature(creature) => &creature.basic_info.name,
            Card::Effect(effect) => &effect.basic_info.name,
        }
    }
}
