use crate::{Card, GameBoard, GameBoardError, MultiCounter, NumberEnum, Player, SuitEnum};

use rand::{seq::SliceRandom, thread_rng};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct GameState {
    game_board: GameBoard,
    pub players: Vec<Player>,
    pub player_turn: usize,
}

#[derive(Debug, Error)]
pub enum GameStateError {
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
    pub fn new(number_of_players: usize) -> Result<GameState, GameStateError> {
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

    #[cfg(test)]
    pub fn with_specific_hands(hands: Vec<Vec<Card>>) -> Result<GameState, GameStateError> {
        let number_of_players = hands.len();
        if number_of_players > 26 {
            return Err(GameStateError::TooManyPlayers);
        }
        let mut players = vec![Player::new(); number_of_players];
        for (i, hand) in hands.into_iter().enumerate() {
            players[i].hand = hand;
        }
        return Ok(GameState {
            game_board: GameBoard::new(),
            players: players,
            player_turn: 0,
        });
    }

    pub fn pass_turn(&mut self) {
        self.player_turn = (self.player_turn + 1) % self.players.len();
    }

    pub fn play_only_playable_card(&mut self) -> Result<(), GameStateError> {
        let playable_cards = self.get_playable_cards();
        let playable = match playable_cards.len() {
            0 => {
                return Err(GameStateError::NoPlayableCard(
                    "play_only_playable_card".to_string(),
                ))
            }
            1 => playable_cards[0].to_owned(),
            _ => {
                return Err(GameStateError::MoreThanOnePlayableCard(
                    "play_only_playable_card".to_string(),
                ))
            }
        };
        self
            .play_card(playable)?;
        self.pass_turn();
        return Ok(());
    }

    pub fn play_card_and_return_new(&self, card: Card) -> Result<GameState, GameStateError> {
        let playable_cards = self.game_board.get_playable_cards();
        if !playable_cards.contains(&card) {
            return Err(GameStateError::UnplayableCard);
        } else {
            let mut output = self.clone();
            output
                .play_card(card)?;
            output.pass_turn();
            return Ok(output);
        }
    }

    pub fn get_playable_cards(&self) -> Vec<Card> {
        let all_playable = self.game_board.get_playable_cards();
        let current_players_hand = &self
            .players
            .get(self.player_turn as usize)
            .expect("current player turn should never exceed max players")
            .hand;
        all_playable
            .into_iter()
            .filter(|x| current_players_hand.contains(x))
            .collect()
    }

    pub fn play_card(&mut self, card: Card) -> Result<(), GameStateError> {
        let playable_cards = self.game_board.get_playable_cards();
        if !playable_cards.contains(&card) {
            return Err(GameStateError::UnplayableCard);
        } else {
            self.game_board
                .play_card(card)
                .map_err(|e| GameStateError::GameBoardError(e))?;
            self.players[self.player_turn]
                .remove_card(&card)
                .map_err(|_| GameStateError::UnplayableCard)?;
            return Ok(());
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
    let mut players: Vec<Player> = Vec::with_capacity(number_of_players);
    for _i in 0..number_of_players {
        players.push(Player::new())
    }
    let counter = MultiCounter::new(vec![number_of_players, 52], false);
    for v in counter {
        players[v[0]].hand.push(deck[v[1]].clone())
    }
    players
}

#[cfg(test)]
mod tests {

    use crate::{
        card_and_enums::{NumberEnum, SuitEnum},
        stack::Stack,
    };

    use super::*;

    #[test]
    fn initialization_with_valid_player_count() {
        let game_state = GameState::new(4);

        assert!(game_state.is_ok());
        let game_state = game_state.unwrap();

        assert_eq!(game_state.players.len(), 4);
        assert_eq!(game_state.player_turn, 0);
    }

    #[test]
    fn initialization_with_invalid_player_count() {
        let game_state = GameState::new(30);

        assert!(game_state.is_err());
        let game_state = game_state.unwrap_err();

        assert_eq!(
            game_state.to_string(),
            GameStateError::TooManyPlayers.to_string()
        );
    }

    #[test]
    fn pass_turn_advances_player_turn() {
        let game_state = GameState::new(4);
        assert!(game_state.is_ok());
        let mut game_state = game_state.unwrap();

        assert_eq!(game_state.player_turn, 0);
        game_state.pass_turn();
        assert_eq!(game_state.player_turn, 1);
    }

    #[test]
    fn pass_turn_resets_to_0_after_last_player_turn() {
        let game_state = GameState::new(3);
        assert!(game_state.is_ok());
        let mut game_state = game_state.unwrap();
        game_state.player_turn = 2;

        game_state.pass_turn();
        assert_eq!(game_state.player_turn, 0);
    }

    #[test]
    fn play_only_playable_card_errors_with_multiple_playable_cards() {
        let hands = vec![
            vec![Card {
                suit: SuitEnum::Diamond,
                number: NumberEnum::Seven,
            }],
            vec![
                Card {
                    suit: SuitEnum::Diamond,
                    number: NumberEnum::Six,
                },
                Card {
                    suit: SuitEnum::Diamond,
                    number: NumberEnum::Eight,
                },
            ],
            vec![],
        ];
        let mut game_state = GameState::with_specific_hands(hands).unwrap();
        let first = game_state.play_only_playable_card();
        assert!(first.is_ok());
        let real_test = game_state.play_only_playable_card();
        assert!(real_test.is_err());
        let err = real_test.unwrap_err();
        assert_eq!(
            err.to_string(),
            GameStateError::MoreThanOnePlayableCard("play_only_playable_card".to_string())
                .to_string()
        );
    }

    #[test]
    fn play_only_playable_card_plays_with_one_playable_card() {
        let hands = vec![
            vec![Card {
                suit: SuitEnum::Diamond,
                number: NumberEnum::Seven,
            }],
            vec![
                Card {
                    suit: SuitEnum::Diamond,
                    number: NumberEnum::Six,
                },
                Card {
                    suit: SuitEnum::Diamond,
                    number: NumberEnum::Eight,
                },
            ],
            vec![],
        ];
        let mut game_state = GameState::with_specific_hands(hands).unwrap();
        let game_board = GameBoard::from(vec![
            Stack::get_completed_stack(SuitEnum::Club),
            Stack::get_completed_stack(SuitEnum::Spade),
            Stack::get_completed_stack(SuitEnum::Heart),
        ]);
        game_state.game_board = game_board;
        let output = game_state.play_only_playable_card();
        assert!(output.is_ok());

        assert_eq!(game_state.player_turn, 1); // turn was passed

        let playables = game_state.get_playable_cards();

        // since we have played the seven of diamonds and all other stacks are completed
        // the next playable cards will be the six and the eight of diamonds
        assert_eq!(playables.len(), 2);
        assert!(playables.contains(&Card {
            suit: SuitEnum::Diamond,
            number: NumberEnum::Eight,
        }));
        assert!(playables.contains(&Card {
            suit: SuitEnum::Diamond,
            number: NumberEnum::Six,
        }));
    }

    #[test]
    fn play_only_playable_card_errors_with_no_playable_card() {
        let mut game_state = GameState::new(3).unwrap();
        let game_board = GameBoard::from(vec![
            Stack::get_completed_stack(SuitEnum::Club),
            Stack::get_completed_stack(SuitEnum::Spade),
            Stack::get_completed_stack(SuitEnum::Heart),
            Stack::get_completed_stack(SuitEnum::Diamond),
        ]); // get a completed board
        game_state.game_board = game_board; // use it
        let output = game_state.play_only_playable_card();
        assert!(output.is_err());

        let output = output.unwrap_err();
        assert_eq!(
            output.to_string(),
            GameStateError::NoPlayableCard("play_only_playable_card".to_string()).to_string()
        )
    }

    #[test]
    fn play_card_and_return_new_succeeds() {
        let hands = vec![
            vec![Card {
                suit: SuitEnum::Diamond,
                number: NumberEnum::Seven,
            }],
            vec![
                Card {
                    suit: SuitEnum::Diamond,
                    number: NumberEnum::Six,
                },
                Card {
                    suit: SuitEnum::Diamond,
                    number: NumberEnum::Eight,
                },
            ],
            vec![],
            vec![],
        ];
        let game_state = GameState::with_specific_hands(hands);
        assert!(game_state.is_ok());
        let game_state = game_state.unwrap();
        let output = game_state.play_card_and_return_new(Card {
            suit: SuitEnum::Diamond,
            number: NumberEnum::Seven,
        });
        assert!(output.is_ok());
    }
}
