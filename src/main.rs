mod game;
mod input;
mod map;

use std::{io::stdout, time::Duration};

use crossterm::{
    cursor,
    event::{Event, KeyCode, poll, read},
    execute,
    style::{SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    terminal::enable_raw_mode().expect("Failed to enable");
    let mut stdout = stdout();
    execute!(stdout, Clear(terminal::ClearType::All), cursor::Hide)?;
    execute!(
        stdout,
        SetBackgroundColor(crossterm::style::Color::Black),
        SetForegroundColor(crossterm::style::Color::White)
    )?;

    // Default 40 x 20
    let mut game = game::Game::new(180, 40);

    game.draw();

    loop {
        if poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = read()? {
                match key_event.code {
                    KeyCode::Char('w') => game.move_player(0, -1),
                    KeyCode::Char('s') => game.move_player(0, 1),
                    KeyCode::Char('a') => game.move_player(-1, 0),
                    KeyCode::Char('d') => game.move_player(1, 0),
                    KeyCode::Char('q') => break,
                    _ => {}
                }

                game.draw();
            }
        }
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
    execute!(stdout, cursor::Show)?;
    Ok(())
    /*  game.draw();
    if !game.update() {
        break;
    } */
}
