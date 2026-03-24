mod commands;
mod editor;
mod ui;

use std::io::{self, stdout};
use std::time::Duration;

use commands::{map_key, Command};
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use editor::Editor;

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let run_result = run(&mut stdout);

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    run_result
}

fn run(stdout: &mut impl io::Write) -> std::io::Result<()> {
    let mut editor = Editor::new();
    ui::render(stdout, &editor)?;

    #[allow(unreachable_code)]
    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if let Some(command) = map_key(key) {
                    let deleted = match command {
                        Command::Left => {
                            editor.move_left();
                            None
                        }
                        Command::Right => {
                            editor.move_right();
                            None
                        }
                        Command::WordForward => {
                            editor.move_word_forward();
                            None
                        }
                        Command::WordBackward => {
                            editor.move_word_backward();
                            None
                        }
                        Command::Down => {
                            editor.move_down();
                            None
                        }
                        Command::Up => {
                            editor.move_up();
                            None
                        }
                        Command::Delete => {
                            let row = editor.cursor_row;
                            let col = editor.cursor_col;
                            editor.delete_char().map(|ch| (row, col, ch))
                        }
                        Command::Quit => break,
                    };

                    editor.apply_command_outcome(deleted);
                    ui::render(stdout, &editor)?;
                }
            }
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
