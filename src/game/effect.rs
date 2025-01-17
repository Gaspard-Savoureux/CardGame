use super::{card::CreatureCard, player::Player};

// NOTE: pas encore sur de l'implémentation
// enum Effect {
//     TargettedCardEffect(TargettedCardEffect),
//     CardEffect(CardEffect),
//     // Targets entities with life
//     Damage(TargettedCardEffect),
//     Heal(TargettedCardEffect),

//     // Target entities on the case
//     Summon(TargettedCardEffect),

//     // Time
//     ChangeRound(CardEffect),
// }

enum EffectType {
    Heal,
    Damage,
    Move,
    Summon,
}

enum Target {
    Creature(Box<CreatureCard>),
    Player(Box<Player>),
}

pub struct TargettedCardEffect {
    target: Target,
    nb: u32, // num of damage, heal, summon, etc.
}

pub struct Effect {
    effect_type: EffectType,
    target: Target,
    nb: u32, // num of damage, heal, summon, etc.
}

impl Effect {
    fn new(effect_type: EffectType, target: Target, nb: u32) -> Self {
        Effect {
            effect_type,
            target,
            nb,
        }
    }
}
