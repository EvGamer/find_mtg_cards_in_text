use crate::card::Card;

enum CardCondition {
  NearMint,
  SlightlyPlayed,
  ModeratelyPlayed,
  Played,
  Damaged, 
}

pub struct CardListing {
  card: Card,
  quantity: u32,
  price: u32,
  condition: CardCondition,
}