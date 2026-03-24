# Vim Practice (Rust Terminal Game)

A minimal terminal game to practice core Vim motions on static multi-line text.

## Requirements

- Rust (stable) with `cargo`
- A terminal that supports ANSI styling

## Getting Started

1. Open a terminal in this project directory:
   ```bash
   cd /Users/user/Documents/md-lessons/project/vimPractice
   ```
2. Run the app:
   ```bash
   cargo run
   ```

`cargo` will build dependencies automatically on the first run.

## How to Use

When the UI appears:

- Top line shows instructions.
- Middle area shows four static lines of text, a highlighted challenge target, and the cursor.
- Bottom area shows the current challenge and cursor position.

Supported keys:

- `h` -> move cursor left
- `l` -> move cursor right
- `w` -> jump to beginning of next word
- `b` -> jump to beginning of previous word
- `j` -> move cursor down
- `k` -> move cursor up
- `x` -> delete character under cursor

Challenges:

- `MoveTo { row, col }`: move cursor to the highlighted target position.
- `DeleteChar { row, col }`: delete the highlighted character with `x`.

When a challenge is solved, `Success!` appears and a new challenge is generated.

## Notes

- This project intentionally stays in normal-command training scope only.
- Unsupported keys are ignored.
- To stop the app, use your terminal interrupt (for example, `Ctrl+C`).
