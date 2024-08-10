use std::collections::HashMap;

use game_board::{GameBoard, GameBoardError};
use multi_counter::MultiCounter;
use rand::{seq::SliceRandom, thread_rng};
use thiserror::Error;

fn main() -> Result<(), String> {
    let mut branches: Vec<GameState> = Vec::new();
    let initial = GameState::new(4).map_err(|e| e.to_string())?;
    let mut victories: Vec<u8> = Vec::new();

    let mut game_state: Option<GameState> = None;
    let mut next_game_state: Option<GameState> = None;
    while victories.is_empty() || !branches.is_empty() {
        match game_state {
            Some(ref state) => match assess_decision(state.to_owned()) {
                Ok(decision) => match decision {
                    Decision::Victory(player) => victories.push(player),
                    _ => next_game_state = Some(process_branches(&mut branches, decision)?),
                },
                Err(e) => return Err(e.to_string()),
            },
            None => match assess_decision(initial.to_owned()) {
                Ok(decision) => match decision {
                    Decision::Victory(player) => victories.push(player),
                    _ => next_game_state = Some(process_branches(&mut branches, decision)?),
                },
                Err(e) => return Err(e.to_string()),
            },
        }

        game_state = next_game_state.take();
    }

    let mut results: HashMap<u8, usize> = HashMap::new();

    for v in victories {
        let _ = match results.get(&v) {
            Some(current) => results.insert(v, current + 1),
            None => results.insert(v, 1),
        };
    }

    Ok(())
}

fn process_branches(
    branches: &mut Vec<GameState>,
    decision: Decision,
) -> Result<GameState, String> {
    match decision {
        Decision::Victory(_) => Err("Victory decision leak".to_string()),
        Decision::NoPlayableCards(state) => Ok(state),
        Decision::OnePlayableCard(state) => Ok(state),
        Decision::MultiplePlayableCards(states) => {
            for state in states {
                branches.push(state);
            }
            match branches.pop() {
                Some(state) => Ok(state),
                None => Err("No states in branches".to_string()),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Card {
    suit: SuitEnum,
    number: NumberEnum,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SuitEnum {
    Spade,
    Club,
    Heart,
    Diamond,
}

impl SuitEnum {
    fn iterator() -> impl Iterator<Item = SuitEnum> {
        [
            SuitEnum::Spade,
            SuitEnum::Club,
            SuitEnum::Heart,
            SuitEnum::Diamond,
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NumberEnum {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl NumberEnum {
    fn iterator() -> impl Iterator<Item = NumberEnum> {
        [
            NumberEnum::Ace,
            NumberEnum::Two,
            NumberEnum::Three,
            NumberEnum::Four,
            NumberEnum::Five,
            NumberEnum::Six,
            NumberEnum::Seven,
            NumberEnum::Eight,
            NumberEnum::Nine,
            NumberEnum::Ten,
            NumberEnum::Jack,
            NumberEnum::Queen,
            NumberEnum::King,
        ]
        .into_iter()
    }
}

mod stack {
    use super::{Card, NumberEnum, SuitEnum};
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
        pub fn new(suit: SuitEnum) -> Stack {
            return Stack {
                suit: suit,
                up_card: None,
                down_card: None,
            };
        }

        pub fn get_playable_cards(&self) -> Result<Option<Vec<Card>>, StackError> {
            match (self.up_card.is_some(), self.down_card.is_some()) {
                (false, false) => {
                    // if nothing has been played, then only the seven is playable
                    return Ok(Some(vec![Card {
                        suit: self.suit,
                        number: NumberEnum::Seven,
                    }]));
                }
                (true, true) => {
                    // if at least the seven has been played, then return the next playable card on each stack,
                    // or None if the direction is complete
                    let playable_up: Option<Card> = match self.up_card.clone().unwrap().number {
                        NumberEnum::Seven => Some(Card {
                            suit: self.suit,
                            number: NumberEnum::Eight,
                        }),
                        NumberEnum::Eight => Some(Card {
                            suit: self.suit,
                            number: NumberEnum::Nine,
                        }),
                        NumberEnum::Nine => Some(Card {
                            suit: self.suit,
                            number: NumberEnum::Ten,
                        }),
                        NumberEnum::Ten => Some(Card {
                            suit: self.suit,
                            number: NumberEnum::Jack,
                        }),
                        NumberEnum::Jack => Some(Card {
                            suit: self.suit,
                            number: NumberEnum::Queen,
                        }),
                        NumberEnum::Queen => Some(Card {
                            suit: self.suit,
                            number: NumberEnum::King,
                        }),
                        NumberEnum::King => None,
                        _ => return Err(StackError::InvalidUpStack),
                    };
                    let playable_down: Option<Card> = match self.down_card.clone().unwrap().number {
                        NumberEnum::Ace => None,
                        NumberEnum::Two => Some(Card {
                            suit: self.suit,
                            number: NumberEnum::Ace,
                        }),
                        NumberEnum::Three => Some(Card {
                            suit: self.suit,
                            number: NumberEnum::Two,
                        }),
                        NumberEnum::Four => Some(Card {
                            suit: self.suit,
                            number: NumberEnum::Three,
                        }),
                        NumberEnum::Five => Some(Card {
                            suit: self.suit,
                            number: NumberEnum::Four,
                        }),
                        NumberEnum::Six => Some(Card {
                            suit: self.suit,
                            number: NumberEnum::Five,
                        }),
                        NumberEnum::Seven => Some(Card {
                            suit: self.suit,
                            number: NumberEnum::Six,
                        }),
                        _ => return Err(StackError::InvalidDownStack),
                    };
                    match (playable_up.is_some(), playable_down.is_some()) {
                        (true, true) => {
                            return Ok(Some(vec![playable_up.unwrap(), playable_down.unwrap()]))
                        }
                        (true, false) => return Ok(Some(vec![playable_up.unwrap()])),
                        (false, true) => return Ok(Some(vec![playable_down.unwrap()])),
                        (false, false) => return Ok(None),
                    }
                }
                _ => return Err(StackError::InvalidStackState),
            }
        }

        pub fn play_card(&mut self, card_number: NumberEnum) -> Result<(), StackError> {
            let playable_cards = match self.get_playable_cards()? {
                Some(cards) => cards,
                None => return Err(StackError::CompletedStackPlayedOn),
            }; // get playable card(s), if none, then stack is complete
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
    }

    #[cfg(test)]
    mod test {

        use super::*;

        #[test]
        fn initialization() {
            let stack = Stack::new(SuitEnum::Spade);
            assert!(stack.up_card.is_none());
            assert!(stack.down_card.is_none());
            assert_eq!(stack.suit, SuitEnum::Spade);
        }

        mod test_get_playable_cards {

            use super::*;

            #[test]
            fn only_seven_plays_on_new_stack() {
                let stack = Stack::new(SuitEnum::Club);
                let playable_cards = stack
                    .get_playable_cards()
                    .expect("Error in Stack::get_playable_cards");
                assert!(playable_cards.is_some());

                let playable_cards = playable_cards.unwrap();
                assert_eq!(playable_cards.len(), 1);
                assert!(playable_cards.contains(&Card {
                    suit: SuitEnum::Club,
                    number: NumberEnum::Seven
                }));
            }

            #[test]
            fn eight_and_six_play_on_seven() {
                let mut stack = Stack::new(SuitEnum::Diamond);
                let seven = Card {
                    suit: SuitEnum::Diamond,
                    number: NumberEnum::Seven,
                };
                stack.up_card = Some(seven.clone());
                stack.down_card = Some(seven);

                let playable_cards = stack
                    .get_playable_cards()
                    .expect("Error in Stack::get_playable_cards")
                    .unwrap();

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
                let mut stack = Stack::new(SuitEnum::Heart);
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

                let playable_cards = stack
                    .get_playable_cards()
                    .expect("Error in Stack::get_playable_cards")
                    .unwrap();

                assert_eq!(playable_cards.len(), 1);
                assert!(playable_cards.contains(&Card {
                    suit: SuitEnum::Heart,
                    number: NumberEnum::Six
                }));
            }

            #[test]
            fn finished_down_stack_does_not_return_playable() {
                let mut stack = Stack::new(SuitEnum::Heart);
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

                let playable_cards = stack
                    .get_playable_cards()
                    .expect("Error in Stack::get_playable_cards")
                    .unwrap();

                assert_eq!(playable_cards.len(), 1);
                assert!(playable_cards.contains(&Card {
                    suit: SuitEnum::Heart,
                    number: NumberEnum::Eight
                }));
            }

            #[test]
            fn completed_stack_does_not_return_playable() {
                let mut stack = Stack::new(SuitEnum::Heart);
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

                let playable_cards = stack
                    .get_playable_cards()
                    .expect("Error in Stack::get_playable_cards");

                assert!(playable_cards.is_none());
            }

            #[test]
            fn invalid_stack_state() {
                let mut stack = Stack::new(SuitEnum::Heart);
                let ace = Card {
                    suit: SuitEnum::Heart,
                    number: NumberEnum::Ace,
                };
                stack.down_card = Some(ace);

                let output = stack.get_playable_cards();

                assert!(output.is_err());
                assert_eq!(
                    output.unwrap_err().to_string(),
                    StackError::InvalidStackState.to_string()
                )
            }

            #[test]
            fn invalid_up_state() {
                let mut stack = Stack::new(SuitEnum::Heart);
                let ace = Card {
                    suit: SuitEnum::Heart,
                    number: NumberEnum::Ace,
                };
                stack.up_card = Some(ace.clone());
                stack.down_card = Some(ace);

                let output = stack.get_playable_cards();

                assert!(output.is_err());
                assert_eq!(
                    output.unwrap_err().to_string(),
                    StackError::InvalidUpStack.to_string()
                )
            }

            #[test]
            fn invalid_down_state() {
                let mut stack = Stack::new(SuitEnum::Heart);
                let king = Card {
                    suit: SuitEnum::Heart,
                    number: NumberEnum::King,
                };
                stack.down_card = Some(king.clone());
                stack.up_card = Some(king);

                let output = stack.get_playable_cards();

                assert!(output.is_err());
                assert_eq!(
                    output.unwrap_err().to_string(),
                    StackError::InvalidDownStack.to_string()
                )
            }
        }

        mod test_play_card {

            use super::*;

            #[test]
            fn seven_plays_on_new_stack() {
                let mut stack = Stack::new(SuitEnum::Club);
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
                let mut stack = Stack::new(SuitEnum::Club);
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
                let mut stack = Stack::new(SuitEnum::Club);
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
                let mut stack = Stack::new(SuitEnum::Club);
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
                let mut stack = Stack::new(SuitEnum::Club);
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
}

mod game_board {

    use super::{Card, SuitEnum};
    use crate::stack::{Stack, StackError};
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
                spade_stack: Stack::new(SuitEnum::Spade),
                club_stack: Stack::new(SuitEnum::Club),
                heart_stack: Stack::new(SuitEnum::Heart),
                diamond_stack: Stack::new(SuitEnum::Diamond),
            };
        }

        pub fn get_playable_cards(&self) -> Result<Option<Vec<Card>>, GameBoardError> {
            let mut playable_spades: Vec<Card> = match self.spade_stack.get_playable_cards() {
                Ok(vec_cards) => match vec_cards {
                    Some(cards) => cards,
                    None => vec![],
                },
                Err(e) => return Err(GameBoardError::StackError(e, "Spades".to_string())),
            };
            let mut playable_clubs: Vec<Card> = match self.club_stack.get_playable_cards() {
                Ok(vec_cards) => match vec_cards {
                    Some(cards) => cards,
                    None => vec![],
                },
                Err(e) => return Err(GameBoardError::StackError(e, "Clubs".to_string())),
            };
            let mut playable_hearts: Vec<Card> = match self.heart_stack.get_playable_cards() {
                Ok(vec_cards) => match vec_cards {
                    Some(cards) => cards,
                    None => vec![],
                },
                Err(e) => return Err(GameBoardError::StackError(e, "Hearts".to_string())),
            };
            let mut playable_diamonds: Vec<Card> = match self.diamond_stack.get_playable_cards() {
                Ok(vec_cards) => match vec_cards {
                    Some(cards) => cards,
                    None => vec![],
                },
                Err(e) => return Err(GameBoardError::StackError(e, "Diamonds".to_string())),
            };
            let mut output: Vec<Card> = Vec::new();
            output.append(&mut playable_spades);
            output.append(&mut playable_clubs);
            output.append(&mut playable_hearts);
            output.append(&mut playable_diamonds);
            if output.len() > 0 {
                return Ok(Some(output));
            }
            return Ok(None);
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

        #[test]
        fn initialization() {
            let game_board = GameBoard::new();

            assert_eq!(game_board.club_stack.suit, SuitEnum::Club);
            assert_eq!(game_board.spade_stack.suit, SuitEnum::Spade);
            assert_eq!(game_board.diamond_stack.suit, SuitEnum::Diamond);
            assert_eq!(game_board.heart_stack.suit, SuitEnum::Heart);
        }

        #[test]
        fn fresh_board_can_play_all_sevens() {
            let game_board = GameBoard::new();

            let output = game_board.get_playable_cards();
            assert!(output.is_ok());

            let output = output.unwrap();
            assert!(output.is_some());

            let output = output.unwrap();

            assert_eq!(output.len(), 4);
            assert!(output.contains(&Card {
                suit: SuitEnum::Club,
                number: crate::NumberEnum::Seven
            }));
            assert!(output.contains(&Card {
                suit: SuitEnum::Spade,
                number: crate::NumberEnum::Seven
            }));
            assert!(output.contains(&Card {
                suit: SuitEnum::Heart,
                number: crate::NumberEnum::Seven
            }));
            assert!(output.contains(&Card {
                suit: SuitEnum::Diamond,
                number: crate::NumberEnum::Seven
            }));
        }

        #[test]
        fn played_seven_plays_eight_and_six() {
            let mut game_board = GameBoard::new();

            game_board
                .club_stack
                .play_card(crate::NumberEnum::Seven)
                .expect("Failed to play seven");

            let output = game_board.get_playable_cards();
            assert!(output.is_ok());

            let output = output.unwrap();
            assert!(output.is_some());

            let output = output.unwrap();

            assert_eq!(output.len(), 5);
            assert!(output.contains(&Card {
                suit: SuitEnum::Club,
                number: crate::NumberEnum::Eight
            }));
            assert!(output.contains(&Card {
                suit: SuitEnum::Club,
                number: crate::NumberEnum::Six
            }));
            assert!(output.contains(&Card {
                suit: SuitEnum::Spade,
                number: crate::NumberEnum::Seven
            }));
            assert!(output.contains(&Card {
                suit: SuitEnum::Heart,
                number: crate::NumberEnum::Seven
            }));
            assert!(output.contains(&Card {
                suit: SuitEnum::Diamond,
                number: crate::NumberEnum::Seven
            }));
        }

        #[test]
        fn can_play_card() {
            let mut game_board = GameBoard::new();

            match game_board.play_card(Card {
                suit: SuitEnum::Diamond,
                number: crate::NumberEnum::Seven,
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
                number: crate::NumberEnum::Six,
            });

            assert!(output.is_err());
            assert_eq!(output.unwrap_err().to_string(), GameBoardError::StackError(StackError::UnplayableCardNumber, "Diamonds".to_string()).to_string())
        }
    }
}

#[derive(Debug, Clone)]
struct Player {
    hand: Vec<Card>,
}

impl Player {
    fn new() -> Player {
        Player { hand: Vec::new() }
    }
}

#[derive(Debug, Clone)]
struct GameState {
    game_board: GameBoard,
    players: Vec<Player>,
    player_turn: u8,
}

#[derive(Debug, Error)]
enum GameStateError {
    #[error("Players exceeded 26 player limit")]
    TooManyPlayers,

    #[error("u8 overflow error")]
    OverflowError,

    #[error("GameBoard Error: {0}")]
    GameBoardError(#[from] GameBoardError),

    #[error("Called {0} on a state with more than one playable card")]
    MoreThanOnePlayableCard(String),

    #[error("Called {0} on a state with no playable card")]
    NoPlayableCard(String),

    #[error("Called play_card_and_return on a state with only one playable card, consider using play_only_playable_card")]
    OnlyOnePlayableCard,

    #[error("Attempted to play an unplayable card in play_card_and_return")]
    UnplayableCard,
}

impl GameState {
    fn new(number_of_players: usize) -> Result<GameState, GameStateError> {
        if number_of_players > 26 {
            return Err(GameStateError::TooManyPlayers);
        }
        let deck = generate_new_shuffle();
        let players = distribute_cards(number_of_players, deck);
        return Ok(GameState {
            game_board: GameBoard::new(),
            players: players,
            player_turn: 0,
        });
    }

    fn pass_turn(&mut self) -> Result<(), GameStateError> {
        if self.players.len() > u8::MAX.into() {
            return Err(GameStateError::OverflowError);
        }
        if self.player_turn < self.players.len() as u8 - 1 {
            self.player_turn += 1;
            return Ok(());
        } else {
            self.player_turn = 0;
            return Ok(());
        }
    }

    fn play_only_playable_card(&mut self) -> Result<(), GameStateError> {
        let playable = match self.game_board.get_playable_cards() {
            Ok(result) => result,
            Err(e) => return Err(GameStateError::GameBoardError(e)),
        };
        let card = match playable {
            Some(card) => {
                if card.len() > 1 {
                    return Err(GameStateError::MoreThanOnePlayableCard(
                        "play_only_playable_card".to_string(),
                    ));
                } else {
                    card[0].to_owned()
                }
            }
            None => {
                return Err(GameStateError::NoPlayableCard(
                    "play_only_playable_card".to_string(),
                ))
            }
        };
        self.game_board
            .play_card(card)
            .map_err(|e| GameStateError::GameBoardError(e))?;
        self.pass_turn()?;
        return Ok(());
    }

    fn play_card_and_return(&self, card: Card) -> Result<GameState, GameStateError> {
        let get_playable = match self.game_board.get_playable_cards() {
            Ok(result) => result,
            Err(e) => return Err(GameStateError::GameBoardError(e)),
        };
        let playable_cards = match get_playable {
            Some(result) => match result.len() {
                0 => {
                    return Err(GameStateError::NoPlayableCard(
                        "play_card_and_return".to_string(),
                    ))
                }
                1 => return Err(GameStateError::OnlyOnePlayableCard),
                _ => result,
            },
            None => {
                return Err(GameStateError::NoPlayableCard(
                    "play_card_and_return".to_string(),
                ))
            }
        };
        if !playable_cards.contains(&card) {
            return Err(GameStateError::UnplayableCard);
        } else {
            let mut output = self.clone();
            output
                .game_board
                .play_card(card)
                .map_err(|e| GameStateError::GameBoardError(e))?;
            output.pass_turn()?;
            return Ok(output);
        }
    }
}

fn generate_new_shuffle() -> Vec<Card> {
    let mut deck = Vec::new();
    let mut rng = thread_rng();
    for suit in SuitEnum::iterator() {
        for number in NumberEnum::iterator() {
            deck.push(Card {
                suit: suit,
                number: number,
            })
        }
    }
    deck.shuffle(&mut rng);
    return deck;
}

fn distribute_cards(number_of_players: usize, deck: Vec<Card>) -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    for _i in 0..number_of_players {
        players.push(Player::new())
    }
    let counter = MultiCounter::new(vec![number_of_players, 52], false);
    for v in counter {
        players[v[0]].hand.push(deck[v[1]].clone())
    }
    players
}

mod multi_counter {
    pub struct MultiCounter {
        counter_maxes: Vec<usize>,
        require_simultaneous_completion: bool,
        _counter_values: Vec<usize>,
        _counter_complete: Vec<bool>,
    }

    impl MultiCounter {
        pub fn new(
            counter_maxes: Vec<usize>,
            require_simultaneous_completion: bool,
        ) -> MultiCounter {
            return MultiCounter {
                counter_maxes: counter_maxes.clone(),
                require_simultaneous_completion: require_simultaneous_completion,
                _counter_values: counter_maxes.iter().map(|_i| 0).collect(),
                _counter_complete: counter_maxes.iter().map(|_i| false).collect(),
            };
        }

        /// Returns the current values of the counters.
        pub fn get_values(&self) -> Vec<usize> {
            self._counter_values.clone()
        }

        /// Checks if all counters are complete based on the mode.
        pub fn check_complete(&self) -> bool {
            match self.require_simultaneous_completion {
                true => {
                    self._counter_values.iter().all(|&value| value == 0)
                        && self._counter_complete.iter().all(|&complete| complete)
                }
                false => self._counter_complete.iter().all(|&complete| complete),
            }
        }

        /// Increments the counter values and returns the new state if not complete, otherwise None.
        pub fn increment(&mut self) {
            let values: Vec<usize> = self
                .get_values()
                .into_iter()
                .enumerate()
                .map(|(index, value)| {
                    if value == self.counter_maxes[index] - 1 {
                        self._counter_complete[index] = true;
                        return 0;
                    } else {
                        return value + 1;
                    }
                })
                .collect();
            self._counter_values = values;
        }
    }

    impl Iterator for MultiCounter {
        type Item = Vec<usize>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.check_complete() {
                true => return None,
                false => {
                    let output = Some(self.get_values());
                    self.increment();
                    return output;
                }
            }
        }
    }

    #[cfg(test)]
    mod tests_for_multicounter {
        use super::*;

        #[test]
        fn test_initialization() {
            let counter = MultiCounter::new(vec![3, 5], true);
            assert_eq!(counter.counter_maxes, vec![3, 5]);
            assert_eq!(counter.require_simultaneous_completion, true);
            assert_eq!(counter._counter_values, vec![0, 0]);
            assert_eq!(counter._counter_complete, vec![false, false]);

            let counter = MultiCounter::new(vec![2, 4, 6], false);
            assert_eq!(counter.counter_maxes, vec![2, 4, 6]);
            assert_eq!(counter.require_simultaneous_completion, false);
            assert_eq!(counter._counter_values, vec![0, 0, 0]);
            assert_eq!(counter._counter_complete, vec![false, false, false]);
        }

        #[test]
        fn test_increment() {
            let mut counter = MultiCounter::new(vec![2, 3], false);

            // check initial values
            assert_eq!(counter._counter_values, vec![0, 0]);
            assert_eq!(counter._counter_complete, vec![false, false]);
            // increment and then check values and completes
            counter.increment();
            assert_eq!(counter._counter_values, vec![1, 1]);
            assert_eq!(counter._counter_complete, vec![false, false]);
            counter.increment();
            assert_eq!(counter._counter_values, vec![0, 2]);
            assert_eq!(counter._counter_complete, vec![true, false]);
            counter.increment();
            assert_eq!(counter._counter_values, vec![1, 0]);
            assert_eq!(counter._counter_complete, vec![true, true]);
            counter.increment();
            assert_eq!(counter._counter_values, vec![0, 1]);
            assert_eq!(counter._counter_complete, vec![true, true]);
            counter.increment();
            assert_eq!(counter._counter_values, vec![1, 2]);
            assert_eq!(counter._counter_complete, vec![true, true]);
            counter.increment();
            assert_eq!(counter._counter_values, vec![0, 0]);
            assert_eq!(counter._counter_complete, vec![true, true]);
        }

        #[test]
        fn test_get_values() {
            let counter = MultiCounter::new(vec![4, 5], false);
            assert_eq!(counter.get_values(), vec![0, 0])
        }

        #[test]
        fn test_check_complete_when_requires_simultaneous_is_true() {
            let mut counter = MultiCounter::new(vec![2, 3], true);
            assert_eq!(counter.check_complete(), false);

            // [1 , 1] [false, false]
            counter.increment();
            assert_eq!(counter.check_complete(), false);

            // [0 , 2] [true, false]
            counter.increment();
            assert_eq!(counter.check_complete(), false);

            // [1 , 0] [true, true]
            counter.increment();
            assert_eq!(counter.check_complete(), false);

            // [0 , 1] [true, true]
            counter.increment();
            assert_eq!(counter.check_complete(), false);

            // [1 , 2] [true, true]
            counter.increment();
            assert_eq!(counter.check_complete(), false);

            // [0 , 0] [true, true]
            counter.increment();
            assert_eq!(counter.check_complete(), true);

            // [1 , 1] [true, true]
            counter.increment();
            assert_eq!(counter.check_complete(), false);
        }

        #[test]
        fn test_check_complete_when_requires_simultaneous_is_false() {
            let mut counter = MultiCounter::new(vec![2, 3], false);
            assert_eq!(counter.check_complete(), false);

            // [1 , 1] [false, false]
            counter.increment();
            assert_eq!(counter.check_complete(), false);

            // [0 , 2] [true, false]
            counter.increment();
            assert_eq!(counter.check_complete(), false);

            // [1 , 0] [true, true]
            counter.increment();
            assert_eq!(counter.check_complete(), true);

            // [0 , 1] [true, true]
            counter.increment();
            assert_eq!(counter.check_complete(), true);

            // [1 , 2] [true, true]
            counter.increment();
            assert_eq!(counter.check_complete(), true);

            // [0 , 0] [true, true]
            counter.increment();
            assert_eq!(counter.check_complete(), true);

            // [1 , 1] [true, true]
            counter.increment();
            assert_eq!(counter.check_complete(), true);
        }

        #[test]
        fn test_iterator_when_requires_simultaneous_is_false() {
            let counter = MultiCounter::new(vec![3, 5], false);
            let output: Vec<_> = counter.into_iter().collect();

            assert_eq!(output.len(), 5);
            assert_eq!(output[0], vec![0, 0]);
            assert_eq!(output[1], vec![1, 1]);
            assert_eq!(output[2], vec![2, 2]);
            assert_eq!(output[3], vec![0, 3]);
            assert_eq!(output[4], vec![1, 4]);
        }

        #[test]
        fn test_iterator_when_requires_simultaneous_is_true() {
            let counter = MultiCounter::new(vec![3, 5], true);
            let output: Vec<_> = counter.into_iter().collect();

            assert_eq!(output.len(), 15);
            assert_eq!(output[0], vec![0, 0]);
            assert_eq!(output[1], vec![1, 1]);
            assert_eq!(output[2], vec![2, 2]);
            assert_eq!(output[3], vec![0, 3]);
            assert_eq!(output[4], vec![1, 4]);
            assert_eq!(output[5], vec![2, 0]);
            assert_eq!(output[6], vec![0, 1]);
            assert_eq!(output[7], vec![1, 2]);
            assert_eq!(output[8], vec![2, 3]);
            assert_eq!(output[9], vec![0, 4]);
            assert_eq!(output[10], vec![1, 0]);
            assert_eq!(output[11], vec![2, 1]);
            assert_eq!(output[12], vec![0, 2]);
            assert_eq!(output[13], vec![1, 3]);
            assert_eq!(output[14], vec![2, 4]);
        }
    }
}

mod decisions {}
enum Decision {
    Victory(u8),
    NoPlayableCards(GameState),
    OnePlayableCard(GameState),
    MultiplePlayableCards(Vec<GameState>),
}

fn assess_decision(mut game_state: GameState) -> Result<Decision, GameStateError> {
    if game_state.players[game_state.player_turn as usize]
        .hand
        .is_empty()
    {
        return Ok(Decision::Victory(game_state.player_turn));
    }
    let playable_cards = match game_state.game_board.get_playable_cards() {
        Ok(playable) => match playable {
            Some(cards) => cards,
            None => {
                game_state.pass_turn()?;
                return Ok(Decision::NoPlayableCards(game_state));
            }
        },
        Err(e) => return Err(GameStateError::GameBoardError(e)),
    };
    if playable_cards.len() == 1 {
        game_state.play_only_playable_card()?;
        return Ok(Decision::OnePlayableCard(game_state));
    } else {
        let output: Result<Vec<GameState>, GameStateError> = playable_cards
            .into_iter()
            .map(|card| game_state.play_card_and_return(card))
            .collect();
        match output {
            Ok(result) => Ok(Decision::MultiplePlayableCards(result)),
            Err(e) => Err(e),
        }
    }
}
