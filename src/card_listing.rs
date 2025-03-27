use std::fmt::Display;

use crate::card::Card;

pub enum CardCondition {
  NearMint,
  SlightlyPlayed,
  ModeratelyPlayed,
  Played,
  Damaged, 
}

pub struct CardListing {
  pub card: Card,
  pub quantity: u32,
  pub price: u32,
  pub condition: CardCondition,
}

impl CardListing {
  pub fn try_create(quantity: Option<u32>, card: &Option<Card>, price: Option<u32>) -> Option<Self> {
    if let (Some(quantity), Some(current_card), Some(price)) = (quantity, current_card, price) {
      return Some(CardListing {
        quantity,
        price,
        card: current_card.clone(),
        condition: CardCondition::NearMint,
      })
    }
    return None;
  }
}

impl Display for CardListing {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let card_name = self.card.get_printed_name();
    return write!(f, "{} {} {}р", self.quantity, card_name, self.price);
  }
}