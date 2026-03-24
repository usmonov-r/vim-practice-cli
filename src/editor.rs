use rand::RngExt;

#[derive(Clone)]
pub enum Challenge {
    MoveTo { row: usize, col: usize },
    DeleteChar { row: usize, col: usize, ch: char },
}

pub struct Editor {
    pub lines: Vec<String>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub challenge: Challenge,
    pub show_success: bool,
}

impl Editor {
    pub fn new() -> Self {
        let lines = vec![
            "Rust is fast and memory-efficient".to_string(),
            "It powers performance-critical services".to_string(),
            "Vim motions make editing extremely fast".to_string(),
            "Practice daily to build muscle memory".to_string(),
        ];

        let mut editor = Self {
            lines,
            cursor_row: 0,
            cursor_col: 0,
            challenge: Challenge::MoveTo { row: 0, col: 0 },
            show_success: false,
        };

        editor.pick_new_challenge();
        editor
    }

    pub fn move_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        }
    }

    pub fn move_right(&mut self) {
        let line_len = self.current_line_len();
        if self.cursor_col + 1 < line_len {
            self.cursor_col += 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor_row + 1 < self.lines.len() {
            self.cursor_row += 1;
            self.clamp_cursor_col();
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.clamp_cursor_col();
        }
    }

    pub fn move_word_forward(&mut self) {
        let chars = self.current_line_chars();
        let len = chars.len();
        let mut i = self.cursor_col;

        while i < len && chars[i] != ' ' {
            i += 1;
        }

        while i < len && chars[i] == ' ' {
            i += 1;
        }

        if i < len {
            self.cursor_col = i;
        }
    }

    pub fn move_word_backward(&mut self) {
        if self.cursor_col == 0 {
            return;
        }

        let chars = self.current_line_chars();
        let mut i = self.cursor_col - 1;

        while i > 0 && chars[i] == ' ' {
            i -= 1;
        }

        while i > 0 && chars[i - 1] != ' ' {
            i -= 1;
        }

        self.cursor_col = i;
    }

    pub fn delete_char(&mut self) -> Option<char> {
        let line_len = self.current_line_len();
        if self.cursor_col >= line_len {
            return None;
        }

        let removed = self.lines[self.cursor_row].remove(self.cursor_col);

        let new_len = self.current_line_len();
        if new_len == 0 {
            self.cursor_col = 0;
        } else if self.cursor_col >= new_len {
            self.cursor_col = new_len - 1;
        }

        Some(removed)
    }

    pub fn apply_command_outcome(&mut self, deleted: Option<(usize, usize, char)>) {
        let success = match &self.challenge {
            Challenge::MoveTo { row, col } => self.cursor_row == *row && self.cursor_col == *col,
            Challenge::DeleteChar { row, col, ch } => {
                if let Some((dr, dc, removed)) = deleted {
                    dr == *row && dc == *col && removed == *ch
                } else {
                    false
                }
            }
        };

        if success {
            self.show_success = true;
            self.pick_new_challenge();
        } else {
            self.show_success = false;
        }
    }

    pub fn challenge_description(&self) -> String {
        match &self.challenge {
            Challenge::MoveTo { row, col } => {
                format!("MoveTo challenge: reach row {}, col {}", row, col)
            }
            Challenge::DeleteChar { row, col, ch } => {
                format!("DeleteChar challenge: delete '{}' at row {}, col {}", ch, row, col)
            }
        }
    }

    fn pick_new_challenge(&mut self) {
        let mut rng = rand::rng();
        let choice = rng.random_range(0..2);

        if choice == 0 {
            if let Some((row, col)) = self.random_non_space_position() {
                self.challenge = Challenge::MoveTo { row, col };
            }
            return;
        }

        if let Some((row, col, ch)) = self.random_non_space_char() {
            self.challenge = Challenge::DeleteChar { row, col, ch };
        } else if let Some((row, col)) = self.random_non_space_position() {
            self.challenge = Challenge::MoveTo { row, col };
        }
    }

    fn random_non_space_position(&self) -> Option<(usize, usize)> {
        let mut candidates = Vec::new();

        for (row, line) in self.lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    candidates.push((row, col));
                }
            }
        }

        if candidates.is_empty() {
            return None;
        }

        let mut rng = rand::rng();
        let idx = rng.random_range(0..candidates.len());
        Some(candidates[idx])
    }

    fn random_non_space_char(&self) -> Option<(usize, usize, char)> {
        let mut candidates = Vec::new();

        for (row, line) in self.lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    candidates.push((row, col, ch));
                }
            }
        }

        if candidates.is_empty() {
            return None;
        }

        let mut rng = rand::rng();
        let idx = rng.random_range(0..candidates.len());
        Some(candidates[idx])
    }

    fn current_line_chars(&self) -> Vec<char> {
        self.lines[self.cursor_row].chars().collect()
    }

    fn current_line_len(&self) -> usize {
        self.lines[self.cursor_row].chars().count()
    }

    fn clamp_cursor_col(&mut self) {
        let len = self.current_line_len();
        if len == 0 {
            self.cursor_col = 0;
        } else if self.cursor_col >= len {
            self.cursor_col = len - 1;
        }
    }
}
