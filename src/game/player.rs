use super::{deck::Deck, life::HasLife};

/**
NOTE:
 - On devrait probablement pas toujours garder tous les decks du player pour ne pas que pendant qu'il joue une partie, il garde tout ses decks en mémoires.
 -
 **/
pub struct Player {
    id_player: u32,   // unique identifier
    name: String,     // in game name
    decks: Vec<Deck>, // all of the decks of a player
    // deck_current: Deck, // the current deck of a player
    hp_max: u32,     // maximum health of a player
    hp_current: u32, // health during game
    ap_max: u32,     // action points at start of game
    ap_current: u32, // action points during game
}

impl Player {
    fn new(id_player: u32, name: String) -> Self {
        // NOTE: les hp et ap vont possiblement varier selon le mode de jeux. Possiblement les add en paramètres ou ajouter un autre constructeur plus tard
        Player {
            id_player,
            name,
            // ADD DECK Constructor
            decks: vec![],
            // deck_current: Deck {},
            hp_max: 100,
            hp_current: 100,
            ap_max: 10,
            ap_current: 10,
        }
    }
}

impl HasLife for Player {
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
