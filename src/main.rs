use std::collections::HashMap;

// represents the cards, their suits and numbers (enums)
mod card_and_enums;

// represents a suit stack, starting with 7s and playing up and down
mod stack;

// represents the game board, with 4 stacks and their current play state
mod game_board;

// keeps track of the player's hand
mod player;

// represents the game state, including the game board and players
mod game_state;

// a little helper to distribute cards evenly
mod multi_counter;

// republish some of the key objects for convenience
pub use card_and_enums::{Card, NumberEnum, SuitEnum};
pub use game_board::{GameBoard, GameBoardError};
pub use game_state::{GameState, GameStateError};
pub use multi_counter::MultiCounter;
pub use player::Player;
pub use stack::{Stack, StackError};

fn main() -> Result<(), String> {
    let number_of_players = 4;

    let mut branches: Vec<GameState> = Vec::new();
    let initial = GameState::new(number_of_players).map_err(|e| e.to_string())?;
    let mut victories: Vec<usize> = Vec::new();

    let mut game_state: GameState = initial;
    let mut next_game_state: Option<GameState> = None;
    while victories.is_empty() || !branches.is_empty() {
        match assess_decision(game_state) {
            Ok(decision) => match decision {
                Decision::Victory(player) => victories.push(player),
                _ => next_game_state = Some(process_branches(&mut branches, decision)?),
            },
            Err(e) => return Err(e.to_string()),
        }

        game_state = match next_game_state {
            Some(state) => {
                next_game_state = None;
                state
            }
            None => {
                if branches.is_empty() {
                    break;
                }
                branches.pop().unwrap()
            }
        };
    }

    let mut results: Vec<usize> = vec![0; number_of_players];

    for v in victories {
        results[v] += 1;
    }

    println!("Results: {:?}", results);

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

enum Decision {
    Victory(usize),
    NoPlayableCards(GameState),
    OnePlayableCard(GameState),
    MultiplePlayableCards(Vec<GameState>),
}

fn assess_decision(mut game_state: GameState) -> Result<Decision, GameStateError> {
    let current_players_hand = &game_state
        .players
        .get(game_state.player_turn as usize)
        .expect("current player turn should never exceed max players")
        .hand;
    if current_players_hand.is_empty() {
        return Ok(Decision::Victory(game_state.player_turn));
    }
    let playable_cards: Vec<Card> = game_state.get_playable_cards();
    if playable_cards.len() == 1 {
        game_state.play_only_playable_card()?;
        return Ok(Decision::OnePlayableCard(game_state));
    } else {
        let output: Result<Vec<GameState>, GameStateError> = playable_cards
            .into_iter()
            .map(|card| game_state.play_card_and_return_new(card))
            .collect();
        match output {
            Ok(result) => Ok(Decision::MultiplePlayableCards(result)),
            Err(e) => Err(e),
        }
    }
}
