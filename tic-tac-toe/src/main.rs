// 1. Create a Player
// 2. Switch player method
// 3. Print Player

// Trait - PartialEq
use board::Board;
use game::Game;
use player::Player;

#![allow(unused)]
mod board;

mod player;

fn main() {
    println!("Hello, world!");
    let mut player = Player::X;
    player.switch_player();
    player.switch_player();

    println!("{:?}", player);
}

#[derive(PartialEq, Debug)]
enum Player {
    X, O
}

impl Player {
    fn switch_player(&mut self) {
        if *self == Player::X {
            *self = Player::O;
        } else {
            *self = Player::X;
        }
    }
}