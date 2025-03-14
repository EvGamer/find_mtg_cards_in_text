use serde::{Deserialize, Serialize};
use serde_json::Result

struct Card {
    id: String,
    name: String,
}

struct CardTrie {
    children: HashMap<char, CardTrie>,
    card: Option<Card>,
}

fn main() {
    println!("Hello, world!");
}
