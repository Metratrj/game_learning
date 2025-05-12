use crossterm::event::{Event, KeyCode, KeyEvent, read};

pub fn read_direction() -> Option<(isize, isize)> {
    if let Ok(Event::Key(KeyEvent { code, .. })) = read() {
        match code {
            KeyCode::Char('w') => Some((0, -1)),
            KeyCode::Char('s') => Some((0, 1)),
            KeyCode::Char('a') => Some((-1, 0)),
            KeyCode::Char('d') => Some((1, 0)),
            KeyCode::Char('q') => None,
            _ => Some((0, 0)),
        }
    } else {
        Some((0, 0))
    }
}
