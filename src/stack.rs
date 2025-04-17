use crate::{Card, NumberEnum, SuitEnum};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Stack {
    pub suit: SuitEnum,
    up_card: Option<Card>,
    down_card: Option<Card>,
}

#[derive(Debug, Error)]
pub enum StackError {
    #[error("Invalid stack state")]
    InvalidStackState,

    #[error("Up position contains card lower than Seven")]
    InvalidUpStack,

    #[error("Down position contains card higher than Seven")]
    InvalidDownStack,

    #[error("Attempted to play on a completed stack")]
    CompletedStackPlayedOn,

    #[error("Attempted to play a card with an unplayable number")]
    UnplayableCardNumber,
}

impl Stack {
    pub fn new_empty(suit: SuitEnum) -> Stack {
        return Stack {
            suit: suit,
            up_card: None,
            down_card: None,
        };
    }

    pub fn get_playable_cards(&self) -> Vec<Card> {
        match (self.up_card, self.down_card) {
            (None, None) => {
                // if nothing has been played, then only the seven is playable
                vec![Card {
                    suit: self.suit,
                    number: NumberEnum::Seven,
                }]
            }
            (Some(up_card), Some(down_card)) => {
                use NumberEnum::*;
                // if at least the seven has been played, then return the next playable card on each stack,
                // or None if the direction is complete
                let playable_up: Option<NumberEnum> = match up_card.number {
                    Seven => Some(Eight),
                    Eight => Some(Nine),
                    Nine => Some(Ten),
                    Ten => Some(Jack),
                    Jack => Some(Queen),
                    Queen => Some(King),
                    King => None,
                    _ => panic!("{}", StackError::InvalidUpStack),
                };
                let playable_down: Option<NumberEnum> = match down_card.number {
                    Ace => None,
                    Two => Some(Ace),
                    Three => Some(Two),
                    Four => Some(Three),
                    Five => Some(Four),
                    Six => Some(Five),
                    Seven => Some(Six),
                    _ => panic!("{}", StackError::InvalidDownStack),
                };
                match (playable_up, playable_down) {
                    (Some(up_num), Some(down_num)) => vec![
                        Card {
                            suit: self.suit,
                            number: up_num,
                        },
                        Card {
                            suit: self.suit,
                            number: down_num,
                        },
                    ],
                    (Some(num), None) | (None, Some(num)) => vec![Card {
                        suit: self.suit,
                        number: num,
                    }],
                    (None, None) => vec![],
                }
            }
            // no valid cases where only one of up or down is Some
            _ => panic!("{}", StackError::InvalidStackState),
        }
    }

    pub fn play_card(&mut self, card_number: NumberEnum) -> Result<(), StackError> {
        let playable_cards = self.get_playable_cards();
        if playable_cards.is_empty() {
            // if there are no playable cards, then the stack is complete
            return Err(StackError::CompletedStackPlayedOn);
        }
        // get playable card(s), if none, then stack is complete
        if playable_cards.contains(&Card {
            suit: self.suit,
            number: card_number,
        }) {
            // if the card is playable
            match card_number {
                NumberEnum::Ace
                | NumberEnum::Two
                | NumberEnum::Three
                | NumberEnum::Four
                | NumberEnum::Five
                | NumberEnum::Six => {
                    // if the card is below seven, it plays on the downwards stack
                    self.down_card = Some(Card {
                        suit: self.suit,
                        number: card_number,
                    });
                    return Ok(());
                }
                NumberEnum::Seven => {
                    // handle the special case for seven, which plays on both stacks
                    self.up_card = Some(Card {
                        suit: self.suit,
                        number: card_number,
                    });
                    self.down_card = Some(Card {
                        suit: self.suit,
                        number: card_number,
                    });
                    return Ok(());
                }
                NumberEnum::Eight
                | NumberEnum::Nine
                | NumberEnum::Ten
                | NumberEnum::Jack
                | NumberEnum::Queen
                | NumberEnum::King => {
                    // if the card is greater than seven, it plays on the up stack
                    self.up_card = Some(Card {
                        suit: self.suit,
                        number: card_number,
                    });
                    return Ok(());
                }
            }
        } else {
            // not contained in the playable cards, therefore an unplayable number
            return Err(StackError::UnplayableCardNumber);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.up_card.is_none() && self.down_card.is_none()
    }

    #[cfg(test)]
    pub fn from(suit: SuitEnum, up_num: Option<NumberEnum>, down_num: Option<NumberEnum>) -> Stack {
        let (up_card, down_card) = match (up_num, down_num) {
            (Some(up), Some(down)) => (
                Some(Card {
                    suit: suit,
                    number: up,
                }),
                Some(Card {
                    suit: suit,
                    number: down,
                }),
            ),
            (Some(_), None) | (None, Some(_)) => {
                panic!("{}", StackError::InvalidStackState);
            }
            (None, None) => (None, None),
        };
        Stack {
            suit: suit,
            up_card: up_card,
            down_card: down_card,
        }
    }

    #[cfg(test)]
    pub fn get_completed_stack(suit: SuitEnum) -> Stack {
        return Stack {
            suit: suit,
            up_card: Some(Card {
                suit: suit,
                number: NumberEnum::King,
            }),
            down_card: Some(Card {
                suit: suit,
                number: NumberEnum::Ace,
            }),
        };
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn initialization() {
        let stack = Stack::new_empty(SuitEnum::Spade);
        assert!(stack.up_card.is_none());
        assert!(stack.down_card.is_none());
        assert_eq!(stack.suit, SuitEnum::Spade);
    }

    mod test_get_playable_cards {

        use super::*;

        #[test]
        fn only_seven_plays_on_new_stack() {
            let stack = Stack::new_empty(SuitEnum::Club);
            let playable_cards = stack.get_playable_cards();
            assert_eq!(playable_cards.len(), 1);
            assert!(playable_cards.contains(&Card {
                suit: SuitEnum::Club,
                number: NumberEnum::Seven
            }));
        }

        #[test]
        fn eight_and_six_play_on_seven() {
            let mut stack = Stack::new_empty(SuitEnum::Diamond);
            let seven = Card {
                suit: SuitEnum::Diamond,
                number: NumberEnum::Seven,
            };
            stack.up_card = Some(seven.clone());
            stack.down_card = Some(seven);

            let playable_cards = stack.get_playable_cards();

            assert_eq!(playable_cards.len(), 2);
            assert!(playable_cards.contains(&Card {
                suit: SuitEnum::Diamond,
                number: NumberEnum::Six
            }));
            assert!(playable_cards.contains(&Card {
                suit: SuitEnum::Diamond,
                number: NumberEnum::Eight
            }));
        }

        #[test]
        fn finished_up_stack_does_not_return_playable() {
            let mut stack = Stack::new_empty(SuitEnum::Heart);
            let seven = Card {
                suit: SuitEnum::Heart,
                number: NumberEnum::Seven,
            };
            let king = Card {
                suit: SuitEnum::Heart,
                number: NumberEnum::King,
            };
            stack.up_card = Some(king);
            stack.down_card = Some(seven);

            let playable_cards = stack.get_playable_cards();

            assert_eq!(playable_cards.len(), 1);
            assert!(playable_cards.contains(&Card {
                suit: SuitEnum::Heart,
                number: NumberEnum::Six
            }));
        }

        #[test]
        fn finished_down_stack_does_not_return_playable() {
            let mut stack = Stack::new_empty(SuitEnum::Heart);
            let seven = Card {
                suit: SuitEnum::Heart,
                number: NumberEnum::Seven,
            };
            let ace = Card {
                suit: SuitEnum::Heart,
                number: NumberEnum::Ace,
            };
            stack.up_card = Some(seven);
            stack.down_card = Some(ace);

            let playable_cards = stack.get_playable_cards();

            assert_eq!(playable_cards.len(), 1);
            assert!(playable_cards.contains(&Card {
                suit: SuitEnum::Heart,
                number: NumberEnum::Eight
            }));
        }

        #[test]
        fn completed_stack_does_not_return_playable() {
            let mut stack = Stack::new_empty(SuitEnum::Heart);
            let king = Card {
                suit: SuitEnum::Heart,
                number: NumberEnum::King,
            };
            let ace = Card {
                suit: SuitEnum::Heart,
                number: NumberEnum::Ace,
            };
            stack.up_card = Some(king);
            stack.down_card = Some(ace);

            let playable_cards = stack.get_playable_cards();

            assert!(playable_cards.is_empty());
        }
    }

    mod test_play_card {

        use super::*;

        #[test]
        fn seven_plays_on_new_stack() {
            let mut stack = Stack::new_empty(SuitEnum::Club);
            let seven = Card {
                suit: SuitEnum::Club,
                number: NumberEnum::Seven,
            };

            let output = stack.play_card(NumberEnum::Seven);

            assert!(output.is_ok());
            assert_eq!(stack.up_card.unwrap(), seven);
            assert_eq!(stack.down_card.unwrap(), seven);
        }

        #[test]
        fn eight_plays_on_up_stack_with_seven() {
            let mut stack = Stack::new_empty(SuitEnum::Club);
            let seven = Card {
                suit: SuitEnum::Club,
                number: NumberEnum::Seven,
            };
            let eight = Card {
                suit: SuitEnum::Club,
                number: NumberEnum::Eight,
            };
            stack.up_card = Some(seven.clone());
            stack.down_card = Some(seven.clone());

            let output = stack.play_card(NumberEnum::Eight);

            assert!(output.is_ok());
            assert_eq!(stack.up_card.unwrap(), eight);
            assert_eq!(stack.down_card.unwrap(), seven);
        }

        #[test]
        fn six_plays_on_down_stack_with_seven() {
            let mut stack = Stack::new_empty(SuitEnum::Club);
            let seven = Card {
                suit: SuitEnum::Club,
                number: NumberEnum::Seven,
            };
            let six = Card {
                suit: SuitEnum::Club,
                number: NumberEnum::Six,
            };
            stack.up_card = Some(seven.clone());
            stack.down_card = Some(seven.clone());

            let output = stack.play_card(NumberEnum::Six);

            assert!(output.is_ok());
            assert_eq!(stack.up_card.unwrap(), seven);
            assert_eq!(stack.down_card.unwrap(), six);
        }

        #[test]
        fn unplayable_card_returns_unplayablecard() {
            let mut stack = Stack::new_empty(SuitEnum::Club);
            let seven = Card {
                suit: SuitEnum::Club,
                number: NumberEnum::Seven,
            };
            stack.up_card = Some(seven.clone());
            stack.down_card = Some(seven.clone());

            let output = stack.play_card(NumberEnum::King);

            assert!(output.is_err());
            assert_eq!(
                output.unwrap_err().to_string(),
                StackError::UnplayableCardNumber.to_string()
            );
            assert_eq!(stack.up_card.unwrap(), seven);
            assert_eq!(stack.down_card.unwrap(), seven);
        }

        #[test]
        fn playing_on_completed_stack_returns_completedstackplayedon() {
            let mut stack = Stack::new_empty(SuitEnum::Club);
            let ace = Card {
                suit: SuitEnum::Club,
                number: NumberEnum::Ace,
            };
            let king = Card {
                suit: SuitEnum::Club,
                number: NumberEnum::King,
            };
            stack.up_card = Some(king.clone());
            stack.down_card = Some(ace.clone());

            let output = stack.play_card(NumberEnum::King);

            assert!(output.is_err());
            assert_eq!(
                output.unwrap_err().to_string(),
                StackError::CompletedStackPlayedOn.to_string()
            );
            assert_eq!(stack.up_card.unwrap(), king);
            assert_eq!(stack.down_card.unwrap(), ace);
        }
    }
}
