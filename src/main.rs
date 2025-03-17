use std::{collections::HashMap, fs::File, io::BufReader};

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

#[derive(Default, Debug)]
struct CardTrie {
  root: CardTrieNode,
}

fn format_children(children: &Option<CardTrieChildren>) -> String {
  return match children {
    Some(children) => children.keys().map(|key| format!("{}", key)).collect::<Vec<_>>().join(", ").to_string(),
    None => "".to_string(),
  };
}

impl CardTrie {
  pub fn insert(&mut self, card: Card) {
    let mut node = &mut self.root;
    for letter in card.name.to_lowercase().chars() {
      let children = node.children.get_or_insert_default();
      node = children.entry(letter).or_default();
    }
    node.card = Some(card);
  }

  pub fn find(&mut self, text: &str) -> Vec<Card> {
    let mut node = &self.root;
    let mut depth = 0;

    let mut match_length = 0;
    let mut current_card: &Option<Card> = &None;
    let mut cards = Vec::new();
    let mut i = 0;

    let mut char_iter = text.chars();
    while let Some(letter) = char_iter.next() {
      depth += 1;
      match node.get_child(letter) {
        Some(child_node) => {
          print!("{}\n", format_children(&node.children));
          if child_node.card.is_some() {
            current_card = &child_node.card;
            match_length = depth;
            node = child_node;
          }
        },
        None => {
          match &current_card {
            Some(current_card) => {
              cards.push(current_card.clone());
            },
            None => {
              print!("{}", format_children(&node.children));
              char_iter.nth_back(match_length);
              i -= match_length;
            }
          }
          node = &self.root;
          depth = 0;
          match_length = 0;
        }
      }
      i += 1;
      print!("i: {i}, depth: {depth} letter: {letter}\n")
    }
    return cards;
  }
}

fn main() {
  let cards_file = File::open("C:\\Users\\dunice\\projects\\find_mtg_cards_in_text\\assets\\cards.json").expect("couldn't open file");
  let cards_file_reader = BufReader::new(cards_file);
  let cards: Vec<Card> = serde_json::from_reader(cards_file_reader).expect("malformed json");
  
  let mut card_trie = CardTrie::default();

  for card in cards {
    card_trie.insert(card);
  }

  let found_cards = card_trie.find("Clock of omens, Plains, Hello world, Demonic Tutor, Defab");
  for card in &found_cards {
    print!("{}", card.name)
  }
}
