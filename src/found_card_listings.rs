use std::str::Chars;

use crate::{card::Card, card_listing::{CardCondition, CardListing}, card_trie::CardTrieNode};

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
  type Item = CardListing;

  fn next(&mut self) -> Option<Self::Item> {
    let mut node = self.root;
    let mut char_iter = self.char_iter.clone();

    let mut current_card: &Option<Card> = &None;
    // let mut i = 0;

    let mut quantity: Option<u32> = None;
    let mut price: Option<u32> = None;
    let mut quantity_str = String::new();
    let mut price_str = String::new();
    let mut is_bracket_open = false;

    while let Some(letter) = char_iter.next() {
      let low_letter = letter.to_lowercase().next().unwrap();

      // ignoring html tags
      if is_bracket_open {
        if low_letter == '>' {
          is_bracket_open = false;
        }
        continue;
      }
      if low_letter == '<' {
        is_bracket_open = true;
        continue;
      }

      if low_letter.is_numeric() {
        if current_card.is_none() {
          quantity_str.push(letter);
        } else {
          price_str.push(letter)
        }
      } else {
        if quantity_str.len() > 0 {
          quantity = quantity_str.parse::<u32>().ok();
          quantity_str.clear();
        } 
        else if price_str.len() > 0 {
          price = price_str.parse::<u32>().ok();
          price_str.clear();

          let card_listing = CardListing::try_create(quantity, current_card, price);
          if card_listing.is_some() {
            return card_listing;
          }
        }

        continue;
      }

      // print!("letter: \"{}\", node:{}\n", low_letter, node);
      match node.get_child(low_letter) {
        Some(child_node) => {
          // if price is set before card is detected, 
          // it's holdover from previous listing and should be cleared
          if price.is_some() {
            price = None;
          }
          node = &child_node;

          self.char_iter = char_iter.clone();

          if child_node.card.is_some() {
            current_card = &child_node.card;
          }
        },
        None => {
          node = &self.root;
          self.char_iter.next();
          char_iter = self.char_iter.clone();
        }
      }
      // i += 1;
    }

    return CardListing::try_create(quantity, current_card, price);
  }
}