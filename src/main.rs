mod game;
mod input;
mod map;
mod types;

use std::{
    io::{Write, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    cursor,
    event::{Event, KeyCode, poll, read},
    execute,
    style::SetForegroundColor,
    terminal::{self, Clear},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    terminal::enable_raw_mode().expect("Failed to enable");
    let mut stdout = stdout();
    execute!(stdout, Clear(terminal::ClearType::All), cursor::Hide)?;
    execute!(stdout, SetForegroundColor(crossterm::style::Color::White))?;

    // Default 40 x 20
    let mut game = game::Game::new(60, 20);

    game.draw()?;

    let mut last_fps = Instant::now();
    let mut frames = 0;
    let mut ticks = 0;

    loop {
        // Game logic tick
        ticks += 1;

        // Drawing
        frames += 1;

        // Render every second FPS and TPS
        if last_fps.elapsed() >= Duration::from_secs(1) {
            let mut lock = std::io::stdout().lock();
            writeln!(lock, "FPS: {}, TPS: {}", frames, ticks).unwrap();
            //println!("FPS: {}, TPS: {}", frames, ticks);
            frames = 0;
            ticks = 0;
            last_fps = Instant::now();
        }

        //let start = Instant::now();

        //let mut last_input_time: HashMap<KeyCode, Instant> = HashMap::new();
        //let input_delay = Duration::from_millis(100); // 100ms between inputs

        if poll(Duration::from_millis(10))? {
            if let Event::Key(/* KeyEvent { code, .. } */ key_event) = read()? {
                /* let now = Instant::now();
                let allowed = last_input_time
                                    .get(&key_event.code)
                                    .map_or(true, |&t| now - t > input_delay);
                */
                // if allowed {
                match key_event.code {
                    KeyCode::Char('w') => game.move_player(0, -1),
                    KeyCode::Char('s') => game.move_player(0, 1),
                    KeyCode::Char('a') => game.move_player(-1, 0),
                    KeyCode::Char('d') => game.move_player(1, 0),
                    KeyCode::Char('q') => break,
                    _ => {}
                }
                // last_input_time.insert(key_event.code, now);
                game.draw()?;
                // }
            }
        }

        //let elapsed = start.elapsed();
        //let target_frame_time = Duration::from_millis(33);

        //if elapsed < target_frame_time {
        //    std::thread::sleep(target_frame_time - elapsed);
        //}
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
    execute!(stdout, cursor::Show)?;
    Ok(())
}
