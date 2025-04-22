use std::collections::HashSet;

use crate::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub hand: HashSet<Card>,
}

impl std::hash::Hash for Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut contents: Vec<&Card> = self.hand.iter().collect();
        contents.sort();
        contents.hash(state);
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            hand: HashSet::new(),
        }
    }

    pub fn remove_card(&mut self, card: &Card) -> Result<(), String> {
        let success = self.hand.remove(card);
        if success {
            Ok(())
        } else {
            Err(format!("Card {:?} not found in hand", card))
        }
    }
}
