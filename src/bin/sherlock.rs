use std::ops::Deref;

use simulations::riffle_shuffle;

#[derive(Debug)]
struct Deck(Vec<u8>);

impl Deck {
    fn new() -> Deck {
        Deck ((1..53).collect())
    }

    fn riffle_shuffle(&mut self) {
        let v = riffle_shuffle(&self.0);
        self.0 = v;
    }
}

impl Deref for Deck {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn follow_strings() {
    
}

fn main() {
    let mut deck = Deck::new();

    deck.riffle_shuffle();
    deck.riffle_shuffle();
    println!("{:?}", deck);
}
