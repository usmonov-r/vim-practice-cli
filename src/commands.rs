use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone, Copy)]
pub enum Command {
    Left,
    Right,
    WordForward,
    WordBackward,
    Down,
    Up,
    Delete,
    Quit,
}

pub fn map_key(event: KeyEvent) -> Option<Command> {
    if matches!(event.code, KeyCode::Char('c')) && event.modifiers.contains(KeyModifiers::CONTROL) {
        return Some(Command::Quit);
    }

    match event.code {
        KeyCode::Char('h') => Some(Command::Left),
        KeyCode::Char('l') => Some(Command::Right),
        KeyCode::Char('w') => Some(Command::WordForward),
        KeyCode::Char('b') => Some(Command::WordBackward),
        KeyCode::Char('j') => Some(Command::Down),
        KeyCode::Char('k') => Some(Command::Up),
        KeyCode::Char('x') => Some(Command::Delete),
        _ => None,
    }
}
