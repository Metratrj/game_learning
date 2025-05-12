mod game;
mod input;
mod map;

use std::io::stdout;

use crossterm::{ExecutableCommand, terminal};

fn main() {
    println!("Hello, world!");

    terminal::enable_raw_mode().expect("Failed to enable");
    stdout()
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    let mut game = game::Game::new();

    loop {
        game.draw();
        if !game.update() {
            break;
        }
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode")
}
