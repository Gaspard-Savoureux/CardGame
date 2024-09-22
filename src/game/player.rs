use super::deck::Deck;

/**
NOTE:
 - On devrait probablement pas toujours garder tous les decks du player pour ne pas que pendant qu'il joue une partie, il garde tout ses decks en mémoires.
 -
 **/
struct Player {
    id_player: u32,     // unique identifier
    name: String,       // in game name
    decks: vec<Deck>,   // all of the decks of a player
    deck_current: Deck, // the current deck of a player
    hp_max: u32,        // health points at start of a game
    hp_current: u32,    // health during game
    ap_max: u32,        // action points at start of game
    ap_current: u32,    // action points during game
}

impl Player {
    fn new(id_player: u32, name: String) -> Self {
        // NOTE: les hp et ap vont possiblement varier selon le mode de jeux. Possiblement les add en paramètres ou ajouter un autre constructeur plus tard
        Player {
            id_player,
            name,
            // ADD DECK Constructor
            hp_max: 100,
            hp_current: 100,
            ap_max: 10,
            ap_current: 10,
        }
    }

    fn playCard(&mut self, index: u32) {
        // Check if the index is within bounds
        if index >= self.deck_current.cards_unplayed.len() {
            println("Invalid card index!");
            return;
        }

        let card_chosen = &self.deck_current.cards_unplayed[index];

        // Check if there are enough action points
        if self.ap_current >= card_chosen.cost {
            // Update action points
            self.ap_current -= card_chosen.cost;

            // Move the card from unplayed to played
            let played_card = self.deck_current.cards_unplayed.remove(index);
            self.deck_current.cards_played.push(played_card);
        } else {
            println("Insufficient action points!");
        }
    }

    fn updateHp(&mut self, amount: u32) {
        self.hp_current += amount;
    }

    // before each game, player current stats are reset to max stats
    fn reset_player() {}
}
