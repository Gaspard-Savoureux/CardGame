struct Card {
    name: String,        // name of the card
    image_front: Path,   // Path to the image on the front of the card
    back_front: Path,    // Path to the image on the back of the card
    description: String, // description of the card
    cost: u32,           // cost of the card
}

impl Card {
    fn new(
        name: String,
        image_front: &str,
        image_back: &str,
        description: String,
        cost: u32,
    ) -> Self {
        Card {
            name,
            image_front: Path::new(image_front).to_path_buf(),
            image_back: Path::new(image_back).to_path_buf(),
            description,
            cost,
        }
    }
}
