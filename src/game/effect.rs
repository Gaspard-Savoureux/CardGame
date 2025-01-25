use super::{card::CreatureCard, life::HasLife, player::Player};

pub enum EffectType {
    Heal,
    Damage,
    Move,
    Summon,
}

pub enum Target {
    Creature(CreatureCard),
    Player(Player),
}

pub struct TargettedCardEffect {
    target: Target,
    nb: u32, // num of damage, heal, summon, etc.
}

pub struct Effect {
    pub effect_type: EffectType,
    pub nb: u32, // num of damage, heal, summon, etc.
}

impl Effect {
    fn new(effect_type: EffectType, nb: u32) -> Self {
        Effect { effect_type, nb }
    }

    pub fn affect_target<T: HasLife>(&self, target: &mut T) {
        match self.effect_type {
            EffectType::Heal => target.heal(self.nb),
            EffectType::Damage => target.damage(self.nb),
            EffectType::Move => todo!(),
            EffectType::Summon => todo!(),
        }
    }
}
