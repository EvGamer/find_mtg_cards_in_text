use std::str::Chars;

use crate::{card::Card, card_trie::CardTrieNode};

pub struct FoundCards<'a> {
  root: &'a CardTrieNode,
  char_iter: Chars<'a>,

}

impl<'a> FoundCards<'a> {
  pub fn new(root: &'a CardTrieNode, text: &'a str) -> FoundCards<'a> {
    return FoundCards {
      root: root,
      char_iter: text.chars(),
    }
  }
}

impl<'a> Iterator for FoundCards<'a> {
  type Item = Card;

  fn next(&mut self) -> Option<Self::Item> {
    let mut node = self.root;
    let mut char_iter = self.char_iter.clone();

    let mut current_card: &Option<Card> = &None;
    // let mut i = 0;

    while let Some(letter) = char_iter.next() {
      let low_letter = letter.to_lowercase().next().unwrap();
      // print!("letter: \"{}\", node:{}\n", low_letter, node);
      match node.get_child(low_letter) {
        Some(child_node) => {
          node = &child_node;

          self.char_iter = char_iter.clone();

          if child_node.card.is_some() {
            current_card = &child_node.card;
          }
        },
        None => {
          if current_card.is_some() {
            return current_card.clone();
          }
          node = &self.root;
          self.char_iter.next();
          char_iter = self.char_iter.clone();
        }
      }
      // i += 1;
    }
    return current_card.clone();
  }
}