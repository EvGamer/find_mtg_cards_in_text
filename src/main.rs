use std::{collections::HashMap, fmt::Display, fs::File, io::BufReader};
// use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Card {
  id: String,
  name: String,
}

type CardTrieChildren = HashMap<char, CardTrieNode>;

#[derive(Default, Debug)]
struct CardTrieNode {
  children: Option<CardTrieChildren>,
  card: Option<Card>,
}

impl CardTrieNode {
  pub fn get_child(&self, letter: char) -> Option<&CardTrieNode> {
    return match &self.children {
      Some(children) => children.get(&letter),
      None => None
    }
  }
}

fn format_child_nodes(child_nodes: &Option<CardTrieChildren>) -> String {
  return match child_nodes {
    Some(children) => children.keys().map(|key| format!("'{}", key)).collect::<Vec<_>>().join(", ").to_string(),
    None => "None".to_string(),
  }
}

impl Display for CardTrieNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let card_name = self.card.as_ref().map_or("None".to_string(), |card|card.name.clone());
    return write!(f, "( card: {}, children: ({}) )", &card_name, format_child_nodes(&self.children));
  }
}

#[derive(Default, Debug)]
struct CardTrie {
  root: CardTrieNode,
}

impl CardTrie {
  pub fn insert(&mut self, card: Card) {
    let mut node = &mut self.root;
    for letter in card.name.chars() {
      let low_letter = letter.to_ascii_lowercase();
      let children = node.children.get_or_insert_with(||HashMap::new());
      node = children.entry(low_letter).or_insert_with(||CardTrieNode{ card: None, children: None });
    }
    node.card = Some(card);
  }

  pub fn find(&mut self, text: &str) -> Vec<Card> {
    let mut node = &self.root;
    let mut depth = 0;

    let mut current_card: &Option<Card> = &None;
    let mut cards = Vec::new();
    let mut i = 0;
    print!("{}\n", text);

    let mut char_iter = text.chars();
    while let Some(letter) = char_iter.next() {
      let low_letter = letter.to_ascii_lowercase();
      print!("{}) letter: \"{}\", len: {}, node:{}\n", i, low_letter, depth, node);
      match node.get_child(low_letter) {
        Some(child_node) => {
          node = &child_node;
          depth += 1;
          if child_node.card.is_some() {
            current_card = &child_node.card;
          }
        },
        None => {
          match &current_card {
            Some(card) => {
              cards.push(card.clone());
              current_card = &None;
            },
            None => {
              if depth > 1 {
                char_iter.nth_back(depth);
                i -= depth;
              }
            }
          }
          node = &self.root;
          depth = 0;
        }
      }
      i += 1;
    }
    return cards;
  }
}

fn main() {
  let cards_file = File::open(".\\assets\\cards.json").expect("couldn't open file");
  let cards_file_reader = BufReader::new(cards_file);
  let cards: Vec<Card> = serde_json::from_reader(cards_file_reader).expect("malformed json");
  
  let mut card_trie = CardTrie::default();

  for card in cards {
    card_trie.insert(card);
  }

  let found_cards = card_trie.find("Clock of omens, Plains, World Hello, Demonic Tutor, Defab, Fabricate");
  for card in &found_cards {
    print!("{}\n", card.name)
  }
}
