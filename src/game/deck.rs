pub struct Deck {
    deck_id: u32,
    deck_size: u32,
    // NOTE: Possiblement remplacer vec par hashmap (simplifie la recherche de carte et l'edit des decks)
    cards_unplayed: vec<Card>,
    cards_played: vec<Card>,
}
