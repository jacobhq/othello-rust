use crate::mcts::mcts_search;
use crate::othello::{Color, Game};

mod othello;
mod mcts;

fn main() {
    let mut game = Game::new();

    while !game.board.game_over() {
        println!("It is {:?}'s turn.", game.current_turn);
        println!("{}", game.board);

        let legal = game.board.legal_moves(game.current_turn);
        if legal.is_empty() {
            println!("{:?} has no legal moves, skipping turn.", game.current_turn);
            game.current_turn = match game.current_turn {
                Color::WHITE => Color::BLACK,
                Color::BLACK => Color::WHITE,
            };
            continue;
        }

        if let Some(pos) = mcts_search(game.board.clone(), game.current_turn, 500) {
            println!("{:?} plays {:?}", game.current_turn, pos);
            game.play_turn(pos).unwrap();
        } else {
            println!("{:?} has no legal moves, skipping turn.", game.current_turn);
        }

        game.current_turn = match game.current_turn {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        };
    }

    let (white, black) = game.board.score();
    println!("Game over! Final score: White = {}, Black = {}", white, black);
}