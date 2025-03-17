use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Card {
  id: String,
  name: String,
}

#[derive(Default, Debug)]
struct CardTrieNode {
  children: Option<HashMap<char, CardTrieNode>>,
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
    let mut char_iter = text.chars();
    let mut depth = 0;
    let mut match_length = 0;
    let node = &self.root;
    let mut cards = Vec::new();
    let mut current_card: &Option<Card> = &None;

    while let Some(letter) = char_iter.next() {
      depth += 1;
      match node.get_child(letter) {
        Some(child_node) => {
          if child_node.card.is_some() {
            current_card = &child_node.card;
            match_length = depth;
          }
        },
        None => {
          match &current_card {
            Some(current_card) => {
              cards.push(current_card.clone());
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
    return cards;
  }
}

fn main() {
  println!("Hello, world!");
}
