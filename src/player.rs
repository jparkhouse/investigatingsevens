use crate::Card;

#[derive(Debug, Clone)]
pub struct Player {
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new() -> Player {
        Player { hand: Vec::new() }
    }

    pub fn remove_card(&mut self, card: &Card) -> Result<(), String> {
        if let Some(pos) = self.hand.iter().position(|x| x == card) {
            self.hand.remove(pos);
            Ok(())
        } else {
            Err(format!("Card {:?} not found in hand", card))
        }
    }
}
