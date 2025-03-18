use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct Card {
  pub id: String,
  pub name: String,
  pub printed_name: Option<String>,
  pub lang: String,

}

impl Card {
  pub fn get_printed_name(&self) -> &String {
    return self.printed_name.as_ref().unwrap_or(&self.name);
  }
}