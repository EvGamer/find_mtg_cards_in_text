mod card;
mod card_trie;
mod found_cards;

use std::{fs::File, io::BufWriter, path::Path};

use card_trie::CardTrie;
use serde_json::to_writer_pretty;

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
