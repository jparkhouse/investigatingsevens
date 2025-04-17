
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

#[derive(Debug, PartialEq, PartialOrd)]
enum DebugLevel {
    None = 0,
    Basic = 1,
    Full = 2,
}

const DEBUG_LEVEL: DebugLevel = DebugLevel::Basic;

fn main() -> Result<(), String> {
    const NUMBER_OF_PLAYERS: usize = 4;
    

    let mut branches: Vec<GameState> = Vec::new();
    let initial = GameState::new(NUMBER_OF_PLAYERS).map_err(|e| e.to_string())?;
    let mut victories: Vec<usize> = Vec::new();

    let mut game_state: GameState = initial;
    let mut next_game_state: Option<GameState> = None;
    while victories.is_empty() || !branches.is_empty() {
        if DEBUG_LEVEL >= DebugLevel::Full {
            println!("Player {} takes a turn", game_state.player_turn);
        }
        match assess_decision(game_state) {
            Ok(decision) => match decision {
                Decision::Victory(player) => {
                    victories.push(player);
                    if DEBUG_LEVEL >= DebugLevel::Basic {
                        println!("A victory for player {}", player);
                    }
                },
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
                    if DEBUG_LEVEL >= DebugLevel::Basic {
                        println!("No more branches, game over");
                    }
                    break;
                }
                if DEBUG_LEVEL >= DebugLevel::Basic {
                    println!("Loading the next parallel universe");
                }
                branches.pop().unwrap()
            }
        };
    }

    let mut results: Vec<usize> = vec![0; NUMBER_OF_PLAYERS];

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
            if DEBUG_LEVEL >= DebugLevel::Basic {
                println!("Now up to {} parallel universes", branches.len());
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
            return Ok(Decision::NoPlayableCards(game_state))},
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
                        println!("Creating parallel universe {} with card {:?}", universe_no, card);
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

