mod battle_ground;
mod field;
mod ship;
mod render;
mod game;

use crate::game::Game;

use std::io::{stdin, Write};

const FIELD_SIZE: u8 = 10;

fn main() {
    let game = Game::new(FIELD_SIZE);

    println!();

    loop {
        let mut s = String::new();

        print!("Please enter some text: ");

        stdin().read_line(&mut s).expect("Did not enter a correct string");

        println!("You typed: {}", s);

        game.render();
    }
}
