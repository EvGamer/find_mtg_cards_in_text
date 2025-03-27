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