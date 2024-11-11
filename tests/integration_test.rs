use investigating_sevens::card_and_enums::{Card, NumberEnum, SuitEnum};
use investigating_sevens::game_state::GameState;
use investigating_sevens::{assess_decision, Decision};

#[test]
fn test_game_initialization() {
    let game_state = GameState::new(4);
    assert!(game_state.is_ok());
    let game_state = game_state.unwrap();

    assert_eq!(game_state.players.len(), 4);
    assert_eq!(game_state.player_turn, 0);
}

#[test]
fn test_card_shuffling_and_distribution() {
    let game_state = GameState::new(4);
    assert!(game_state.is_ok());
    let game_state = game_state.unwrap();

    let mut total_cards = 0;
    for player in game_state.players {
        total_cards += player.hand.len();
    }

    assert_eq!(total_cards, 52);
}

#[test]
fn test_turn_passing() {
    let game_state = GameState::new(4);
    assert!(game_state.is_ok());
    let mut game_state = game_state.unwrap();

    assert_eq!(game_state.player_turn, 0);
    game_state.pass_turn().unwrap();
    assert_eq!(game_state.player_turn, 1);
    game_state.pass_turn().unwrap();
    assert_eq!(game_state.player_turn, 2);
    game_state.pass_turn().unwrap();
    assert_eq!(game_state.player_turn, 3);
    game_state.pass_turn().unwrap();
    assert_eq!(game_state.player_turn, 0);
}

#[test]
fn test_victory_condition() {
    let mut game_state = GameState::new(4).unwrap();
    game_state.players[0].hand.clear();

    let decision = assess_decision(game_state);
    assert!(decision.is_ok());
    let decision = decision.unwrap();

    match decision {
        Decision::Victory(player) => assert_eq!(player, 0),
        _ => panic!("Expected Victory decision"),
    }
}

#[test]
fn test_game_initialization_with_more_players() {
    let game_state = GameState::new(6);
    assert!(game_state.is_ok());
    let game_state = game_state.unwrap();

    assert_eq!(game_state.players.len(), 6);
    assert_eq!(game_state.player_turn, 0);
}

#[test]
fn test_card_shuffling_and_distribution_with_more_players() {
    let game_state = GameState::new(6);
    assert!(game_state.is_ok());
    let game_state = game_state.unwrap();

    let mut total_cards = 0;
    for player in game_state.players {
        total_cards += player.hand.len();
    }

    assert_eq!(total_cards, 52);
}

#[test]
fn test_turn_passing_with_more_players() {
    let game_state = GameState::new(6);
    assert!(game_state.is_ok());
    let mut game_state = game_state.unwrap();

    assert_eq!(game_state.player_turn, 0);
    game_state.pass_turn().unwrap();
    assert_eq!(game_state.player_turn, 1);
    game_state.pass_turn().unwrap();
    assert_eq!(game_state.player_turn, 2);
    game_state.pass_turn().unwrap();
    assert_eq!(game_state.player_turn, 3);
    game_state.pass_turn().unwrap();
    assert_eq!(game_state.player_turn, 4);
    game_state.pass_turn().unwrap();
    assert_eq!(game_state.player_turn, 5);
    game_state.pass_turn().unwrap();
    assert_eq!(game_state.player_turn, 0);
}

#[test]
fn test_victory_condition_with_more_players() {
    let mut game_state = GameState::new(6).unwrap();
    game_state.players[0].hand.clear();

    let decision = assess_decision(game_state);
    assert!(decision.is_ok());
    let decision = decision.unwrap();

    match decision {
        Decision::Victory(player) => assert_eq!(player, 0),
        _ => panic!("Expected Victory decision"),
    }
}
