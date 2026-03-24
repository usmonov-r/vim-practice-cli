use std::io::Write;

use crossterm::{
    cursor::MoveTo,
    style::{Attribute, Print, SetAttribute},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::editor::{Challenge, Editor};

pub fn render(stdout: &mut impl Write, editor: &Editor) -> std::io::Result<()> {
    stdout.queue(Clear(ClearType::All))?;
    stdout.queue(MoveTo(0, 0))?;
    stdout.queue(Print("Use h, l, w, b, j, k, x for Vim training"))?;

    for (row, line) in editor.lines.iter().enumerate() {
        stdout.queue(MoveTo(0, (row + 2) as u16))?;

        for (col, ch) in line.chars().enumerate() {
            let is_cursor = row == editor.cursor_row && col == editor.cursor_col;
            let is_target = match &editor.challenge {
                Challenge::MoveTo {
                    row: target_row,
                    col: target_col,
                } => row == *target_row && col == *target_col,
                Challenge::DeleteChar {
                    row: target_row,
                    col: target_col,
                    ..
                } => row == *target_row && col == *target_col,
            };

            if is_target {
                stdout.queue(SetAttribute(Attribute::Underlined))?;
            }

            if is_cursor {
                stdout.queue(SetAttribute(Attribute::Reverse))?;
            }

            stdout.queue(Print(ch))?;
            stdout.queue(SetAttribute(Attribute::Reset))?;
        }

        if line.is_empty() && row == editor.cursor_row {
            stdout.queue(SetAttribute(Attribute::Reverse))?;
            stdout.queue(Print(" "))?;
            stdout.queue(SetAttribute(Attribute::Reset))?;
        }
    }

    let footer_row = (editor.lines.len() + 3) as u16;
    stdout.queue(MoveTo(0, footer_row))?;
    stdout.queue(Print(editor.challenge_description()))?;

    stdout.queue(MoveTo(0, footer_row + 1))?;
    stdout.queue(Print(format!(
        "cursor: ({}, {})",
        editor.cursor_row, editor.cursor_col
    )))?;

    stdout.queue(MoveTo(0, footer_row + 2))?;
    if editor.show_success {
        stdout.queue(Print("Success!"))?;
    } else {
        stdout.queue(Print("        "))?;
    }

    stdout.flush()
}
