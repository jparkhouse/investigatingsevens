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

use std::collections::HashSet;

// republish some of the key objects for convenience
pub use card_and_enums::{Card, NumberEnum, SuitEnum};
pub use game_board::{GameBoard, GameBoardError};
pub use game_state::{GameState, GameStateError};
pub use multi_counter::MultiCounter;
pub use player::Player;
pub use stack::{Stack, StackError};

#[derive(Debug, PartialEq, PartialOrd)]
enum DebugLevel {
    None = 0,
    Basic = 1,
    Full = 2,
}

const DEBUG_LEVEL: DebugLevel = DebugLevel::None;
const NUMBER_OF_PLAYERS: usize = 4;

fn main() -> Result<(), String> {
    let mut branches: Vec<GameState> = Vec::new();
    let mut prev_game_states: HashSet<GameState> = HashSet::new();
    let initial = GameState::new(NUMBER_OF_PLAYERS).map_err(|e| e.to_string())?;
    let mut results: Vec<usize> = vec![0; NUMBER_OF_PLAYERS];
    // since initially there won't be any branches,
    // we need a lock to do the first few iterations
    let mut no_results_yet = true;

    let mut game_state: GameState = initial;
    let mut next_game_state: Option<GameState> = None;
    while no_results_yet || !branches.is_empty() {
        if DEBUG_LEVEL >= DebugLevel::Full {
            println!("Player {} takes a turn", game_state.player_turn);
        }

        // first we must log that we are processing this gamestate
        prev_game_states.insert(game_state.clone());

        // then we process it
        match assess_decision(game_state) {
            Ok(decision) => match decision {
                Decision::Victory(player) => {
                    // we have at least one result, so we can release the lock
                    no_results_yet = false;
                    results[player] += 1;
                    if DEBUG_LEVEL >= DebugLevel::Basic {
                        println!("The {}th victory for player {}", results[player], player);
                        println!("{:?}", results);
                    }
                }
                Decision::NoPlayableCards(gs) | Decision::OnePlayableCard(gs) => {
                    next_game_state = Some(gs)
                }
                Decision::MultiplePlayableCards(possible_gs) => {
                    next_game_state =
                        Some(add_new_branches_and_return_one(&mut branches, possible_gs)?)
                }
            },
            Err(e) => return Err(e.to_string()),
        }

        // check that the next game state is worth computing
        while need_new_game_state(&next_game_state, &prev_game_states) {
            if DEBUG_LEVEL >= DebugLevel::Full {
                println!("Skipping previously seen state");
            }
            next_game_state = Some(get_next_branch(&mut branches)?);
        }

        // replace game_state with next
        game_state = match next_game_state {
            Some(next) => {
                next_game_state = None;
                next
            }
            None => {
                if DEBUG_LEVEL >= DebugLevel::Basic {
                    println!("No more branches");
                }
                break;
            }
        }
    }

    println!("Results: {:?}", results);

    Ok(())
}

fn add_new_branches_and_return_one(
    branches: &mut Vec<GameState>,
    new_branches: Vec<GameState>,
) -> Result<GameState, String> {
    for state in new_branches {
        branches.push(state);
    }
    if DEBUG_LEVEL >= DebugLevel::Basic {
        println!("Now up to {} parallel universes", branches.len());
    }
    match branches.pop() {
        Some(state) => Ok(state),
        None => Err("No states in branches".to_string()),
    }
}

fn get_next_branch(branches: &mut Vec<GameState>) -> Result<GameState, String> {
    match branches.pop() {
        Some(state) => Ok(state),
        None => Err("No states in branches".to_string()),
    }
}

fn need_new_game_state(
    possible_state: &Option<GameState>,
    previous_states: &HashSet<GameState>,
) -> bool {
    // if there is a possible next state,
    if let Some(state) = possible_state {
        // and we have not seen it before
        if !previous_states.contains(state) {
            // we can keep this one
            return false;
        }
    }
    // otherwise we need to get a new one
    true
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
        .get(game_state.player_turn)
        .expect("current player turn should never exceed max players")
        .hand;
    if current_players_hand.is_empty() {
        return Ok(Decision::Victory(game_state.player_turn));
    }
    let playable_cards: Vec<Card> = game_state.get_playable_cards();
    match playable_cards.len() {
        0 => {
            if DEBUG_LEVEL >= DebugLevel::Full {
                println!("No playable cards");
            }

            game_state.pass_turn();
            return Ok(Decision::NoPlayableCards(game_state));
        }
        1 => {
            if DEBUG_LEVEL >= DebugLevel::Full {
                println!("One playable card: {:?}", playable_cards[0]);
            }
            game_state.play_only_playable_card()?;
            return Ok(Decision::OnePlayableCard(game_state));
        }
        _ => {
            let output: Result<Vec<GameState>, GameStateError> = playable_cards
                .into_iter()
                .enumerate()
                .map(|(universe_no, card)| {
                    if DEBUG_LEVEL >= DebugLevel::Full {
                        println!(
                            "Creating parallel universe {} with card {:?}",
                            universe_no, card
                        );
                    }

                    game_state.play_card_and_return_new(card)
                })
                .collect();
            match output {
                Ok(result) => Ok(Decision::MultiplePlayableCards(result)),
                Err(e) => Err(e),
            }
        }
    }
}
