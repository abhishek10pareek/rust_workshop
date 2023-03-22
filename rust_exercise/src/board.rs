extern crate termcolor;

use std::io::Write;

use board::BoardPosition::{NotOccupied, Occupied};

use crate::player::Player;

use self::termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Clone, Copy, PartialEq)]
pub enum BoardPosition {
    Occupied(Player),
    NotOccupied(u8),
}

#[derive(Clone, Copy)]
pub struct Board {
    state: [BoardPosition; 9],
}

impl Board {
    pub fn new() -> Self {
        Board {
            state: [
                NotOccupied(1),
                NotOccupied(2),
                NotOccupied(3),
                NotOccupied(4),
                NotOccupied(5),
                NotOccupied(6),
                NotOccupied(7),
                NotOccupied(8),
                NotOccupied(9),
            ],
        }
    }

    pub fn state(&self) -> [BoardPosition; 9] {
        self.state
    }

    pub fn update_state(&mut self, state: [BoardPosition; 9]) {
        self.state = state;
    }

    pub fn draw(&self) {
        println!("\n");

        for i in (0..3).rev() {
            let offset = i * 3;

            print!("-------------\n| ");
            self.print(&self.state[offset]);
            print!(" | ");
            self.print(&self.state[offset + 1]);
            print!(" | ");
            self.print(&self.state[offset + 2]);
            println!(" |");
        }

        println!("-------------");
    }

    pub fn print(&self, state: &BoardPosition) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        match state {
            NotOccupied(_) => write!(&mut stdout, "-").unwrap(),
            Occupied(player) => {
                let state = player.to_char();
                if state == 'X' {
                    stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
                        .unwrap();
                } else if state == 'O' {
                    stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                        .unwrap();
                }

                write!(&mut stdout, "{}", state).unwrap();
            }
        }

        stdout.reset().unwrap();
    }
}
