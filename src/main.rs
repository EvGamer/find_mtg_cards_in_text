use std::{collections::HashMap, fmt::Display, fs::File, io::{BufRead, BufReader, BufWriter}, path::Path};
// use std::collections::HashMap;

use bincode::{decode_from_std_read, encode_into_std_write, error::DecodeError, Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_json::{self, to_writer_pretty};

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
struct Card {
  id: String,
  name: String,
  printed_name: Option<String>,
  lang: String,

}

impl Card {
  fn get_printed_name(&self) -> &String {
    return self.printed_name.as_ref().unwrap_or(&self.name);
  }
}

type CardTrieChildren = HashMap<char, CardTrieNode>;

#[derive(Encode, Decode, Default, Debug)]
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
    let card_name = self.card.as_ref().map_or("None".to_string(), |card|card.get_printed_name().clone());
    return write!(f, "( card: {}, children: ({}) )", &card_name, format_child_nodes(&self.children));
  }
}

#[derive(Encode, Decode, Debug, Default)]
struct CardTrie {
  root: CardTrieNode,
}

impl CardTrie {
  pub fn insert(&mut self, card: Card) {
    if card.lang != "en" && card.lang != "ru" || card.get_printed_name().len() < 3 {
      return;
    }
    let mut node = &mut self.root;
    for letter in card.get_printed_name().chars() {
      let low_letter = letter.to_lowercase().next().unwrap();
      let children = node.children.get_or_insert_with(||HashMap::new());
      node = children.entry(low_letter).or_insert_with(||CardTrieNode{ card: None, children: None });
    }
    node.card = Some(card);
  }

  pub fn find_in_file(&self, path: &Path) -> Result<Vec<Card>, &str> {
    if !path.exists() { return Err("no file to scan") };
    let file = File::open(path);
    if file.is_err() { return Err("failed to load a file ")}
    let reader = BufReader::new(file.unwrap());
  
    let mut cards: Vec<Card> = Vec::new();
    for line in reader.lines() {
      if line.is_err() { continue; };
      cards.extend(self.find(&line.unwrap().to_string()));
    }
    return Ok(cards);
  }

  pub fn find(&self, text: &str) -> Vec<Card> {
    let mut node = &self.root;
    let mut depth = 0;

    let mut current_card: &Option<Card> = &None;
    let mut cards = Vec::new();
    // let mut i = 0;

    let mut char_iter = text.chars();
    let mut char_iter_before_match = char_iter.clone();
    while let Some(letter) = char_iter.next() {
      let low_letter = letter.to_lowercase().next().unwrap();
      // print!("{}) letter: \"{}\", len: {}, node:{}\n", i, low_letter, depth, node);
      match node.get_child(low_letter) {
        Some(child_node) => {
          node = &child_node;
          depth += 1;
          char_iter_before_match = char_iter.clone();
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
                char_iter = char_iter_before_match.clone();
                // i -= depth;
              }
            }
          }
          node = &self.root;
          depth = 0;
        }
      }
      // i += 1;
    }
    if current_card.is_some() {
      cards.push(current_card.as_ref().unwrap().clone());
    }
    return cards;
  }

  fn save(&self, path: &Path) -> Result<(), &str> {
    let file = File::create(path);
    if file.is_err() {
      return Err("failed to save");
    }
    let write_buffer = &mut BufWriter::new(file.unwrap());
    let result = encode_into_std_write(self, write_buffer, bincode::config::standard());
    if result.is_err() {
      return Err("failed to save")
    };
    return Ok(())
  }

  fn load_from_card_list(cards_path: &Path) -> Result<Self, &str> {
    if !cards_path.exists() {
      return Err("cards list doesnt exist");
    }
    let mut cards_trie = CardTrie::default();
    let cards_file = File::open(cards_path);
    if cards_file.is_err() {
      return Err("failed to read file");
    }

    let cards_file_reader = BufReader::new(cards_file.unwrap());
    let cards = serde_json::from_reader::<BufReader<File>, Vec<Card>>(cards_file_reader);
    if cards.is_err() {
      return Err("malformed json");
    }
    
    for card in cards.unwrap() {
      cards_trie.insert(card);
    }
    return Ok(cards_trie);
  }

  fn load(path: &Path) -> Result<Self, &str> {
    if !path.exists() {
      return Err("file dont exist");
    }
    let file = File::open(path);
    if file.is_err() {
      return Err("failed to load card trie file");
    }
    let read_buffer = &mut BufReader::new(file.unwrap());
    let result: Result<Self, DecodeError> = decode_from_std_read(read_buffer, bincode::config::standard());
    if result.is_err() {
      return Err("failed to decode")
    }
    return Ok(result.unwrap())
  }
}

fn main() {
  let cards_path = Path::new("./assets/cards.json");
  let card_trie_path = Path::new("./assets/card_tree.mct");

  print!("Loading...\n");
  let card_trie= if card_trie_path.exists() { 
    CardTrie::load(card_trie_path).unwrap()
  } else {
    CardTrie::load_from_card_list(cards_path).unwrap()
  };

  if !card_trie_path.exists() {
    card_trie.save(card_trie_path).expect("failed_to_save");
  }

  let found_cards = card_trie.find_in_file(Path::new("./assets/page.html"));

  let output_file = File::create("./assets/found_cards.json").expect("couldn't create output file");
  let output_file_writer = BufWriter::new(output_file);
  to_writer_pretty(output_file_writer, &found_cards).expect("failed to write output file");
}
