struct Player {
    id_player: u32,            // unique identifier
    name: String,       // in game name
    decks: vec<Deck>,   // all of the decks of a player
    deck_current: Deck, // the current deck of a player
    hp_max: u32,        // health points at start of a game
    hp_current: u32,    // health during game
    ap_max: u32,        // action points at start of game
    ap_current: u32,    // action points during game
}

impl Player {

    fn new(id: u32, name: string) -> Self {
        Player{
            id,
            name,
            // ADD DECK Constructor
            100,
            100,
            10,
            10
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


    fn updateHp(&mut self, amount: u32){
        self.hp_current += amount;
    }

    // before each game, player current stats are reset to max stats
    fn reset_player(){

    }


}
