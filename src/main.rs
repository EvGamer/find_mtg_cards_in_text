use serde::{Deserialize, Serialize};
use serde_json::Result

#[derive(Deserialize, Serialize)]
struct Card {
  id: String,
  name: String,
}

#[derive(Default, Debug)]
struct CardTrieNode {
  children: Option<HashMap<char, CardTrieNode>>,
  card: Option<Card>,
}

#[derive(Default, Debug)]
struct CardTrie {
  root: CardTrieNode,
}

impl CardTrie {
  pub fn insert(&mut self, mut card: Card) {
    let mut node = &mut self.root;
    for letter in card.name.to_lowercase().chars() {
      if node.children == None {
        node.children = Some(HashMap<char, CardTrieNode>::new());
      }
      if let Some(children) = node.children {
        node = node.children.entry(letter).or_default();
      }
    }
    node.card = card;
  }

  pub fn find(&mut self, &str text) -> Vec<Card> {
    let char_iter = text.chars();
    let mut depth = 0;
    let mut match_length = 0;
    let &node = self.root;
    let cards = Vec<Card>::new()
    let mut current_card: Option<Card> = None;

    for letter in char_iter {
      depth++;
      match node.children.get(&letter) {
        Some(child_node) => {
          if (child_node.card.is_some()) {
            current_card = child_node.card;
            match_length = depth;
          }
        },
        None => {
          match current_card {
            Some(current_card) => {
              cards.push(current_card);
            },
            None => {
              char_iter.nth_back(match_length - 1);
            }
          }
          depth = 0;
          match_length = 0;
        }
      }
    }
  }
}

fn main() {
  println!("Hello, world!");
}
