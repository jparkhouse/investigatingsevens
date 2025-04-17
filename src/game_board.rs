use std::vec;

use crate::{Card, NumberEnum, SuitEnum};
use crate::{Stack, StackError};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct GameBoard {
    spade_stack: Stack,
    club_stack: Stack,
    heart_stack: Stack,
    diamond_stack: Stack,
}

#[derive(Debug, Error)]
pub enum GameBoardError {
    #[error("'{0}' error in {1} stack")]
    StackError(StackError, String),
}

impl GameBoard {
    pub fn new() -> GameBoard {
        return GameBoard {
            spade_stack: Stack::new_empty(SuitEnum::Spade),
            club_stack: Stack::new_empty(SuitEnum::Club),
            heart_stack: Stack::new_empty(SuitEnum::Heart),
            diamond_stack: Stack::new_empty(SuitEnum::Diamond),
        };
    }

    #[cfg(test)]
    pub fn from(stacks: Vec<Stack>) -> GameBoard {
        let mut output = GameBoard::new();
        for st in stacks {
            match st.suit {
                SuitEnum::Spade => output.spade_stack = st,
                SuitEnum::Club => output.club_stack = st,
                SuitEnum::Heart => output.heart_stack = st,
                SuitEnum::Diamond => output.diamond_stack = st,
            }
        }
        // panics if anything is misconfigured
        output.get_playable_cards();
        output
    }

    pub fn get_playable_cards(&self) -> Vec<Card> {
        // if the seven of diamonds has not been played yet,
        // it is the only playable card
        if self.diamond_stack.is_empty() {
            return vec![Card {
                suit: SuitEnum::Diamond,
                number: NumberEnum::Seven,
            }];
        }
        // otherwise check each stack's playable cards
        [
            &self.spade_stack,
            &self.club_stack,
            &self.heart_stack,
            &self.diamond_stack,
        ]
        .iter()
        .flat_map(|x| x.get_playable_cards())
        .collect()
    }

    pub fn play_card(&mut self, card: Card) -> Result<(), GameBoardError> {
        match card.suit {
            SuitEnum::Spade => self
                .spade_stack
                .play_card(card.number)
                .map_err(|e| GameBoardError::StackError(e, "Spades".to_string())),
            SuitEnum::Club => self
                .club_stack
                .play_card(card.number)
                .map_err(|e| GameBoardError::StackError(e, "Clubs".to_string())),
            SuitEnum::Heart => self
                .heart_stack
                .play_card(card.number)
                .map_err(|e| GameBoardError::StackError(e, "Hearts".to_string())),
            SuitEnum::Diamond => self
                .diamond_stack
                .play_card(card.number)
                .map_err(|e| GameBoardError::StackError(e, "Diamonds".to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::card_and_enums::{NumberEnum, SuitEnum};

    #[test]
    fn initialization() {
        let game_board = GameBoard::new();

        assert_eq!(game_board.club_stack.suit, SuitEnum::Club);
        assert_eq!(game_board.spade_stack.suit, SuitEnum::Spade);
        assert_eq!(game_board.diamond_stack.suit, SuitEnum::Diamond);
        assert_eq!(game_board.heart_stack.suit, SuitEnum::Heart);
    }

    #[test]
    fn fresh_board_can_play_only_seven_of_diamonds() {
        let game_board = GameBoard::new();

        let output = game_board.get_playable_cards();
        assert_eq!(output.len(), 1);
        assert!(output.contains(&Card {
            suit: SuitEnum::Diamond,
            number: NumberEnum::Seven
        }));
    }

    #[test]
    fn played_seven_plays_eight_and_six() {
        let mut game_board = GameBoard::new();

        game_board
            .diamond_stack
            .play_card(NumberEnum::Seven)
            .expect("Failed to play seven");

        let output = game_board.get_playable_cards();

        assert_eq!(output.len(), 5);
        assert!(output.contains(&Card {
            suit: SuitEnum::Diamond,
            number: NumberEnum::Eight
        }));
        assert!(output.contains(&Card {
            suit: SuitEnum::Diamond,
            number: NumberEnum::Six
        }));
        assert!(output.contains(&Card {
            suit: SuitEnum::Spade,
            number: NumberEnum::Seven
        }));
        assert!(output.contains(&Card {
            suit: SuitEnum::Heart,
            number: NumberEnum::Seven
        }));
        assert!(output.contains(&Card {
            suit: SuitEnum::Club,
            number: NumberEnum::Seven
        }));
    }

    #[test]
    fn can_play_card() {
        let mut game_board = GameBoard::new();

        match game_board.play_card(Card {
            suit: SuitEnum::Diamond,
            number: NumberEnum::Seven,
        }) {
            Ok(_) => {}
            Err(e) => panic!("Error playing diamond 7 on empty gameboard: {e}"),
        };
    }

    #[test]
    fn cannot_play_unplayable_card() {
        let mut game_board = GameBoard::new();

        let output = game_board.play_card(Card {
            suit: SuitEnum::Diamond,
            number: NumberEnum::Six,
        });

        assert!(output.is_err());
        assert_eq!(
            output.unwrap_err().to_string(),
            GameBoardError::StackError(StackError::UnplayableCardNumber, "Diamonds".to_string())
                .to_string()
        )
    }
}
